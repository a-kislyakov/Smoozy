//! Pay only the recorded material debt of a submission proved never taken.
//! `Rendering` S4b and S9. No directory tree is discovered or removed.
use crate::record::{
    outcome_records::OutcomeRecords,
    submission_material::{self, Material},
};
use crate::uploading::TheRecordAnUploadWrites;
use smoozy_server_setup::readings::{TheMachinesFileSurface, WhatTheMachineSaid};
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{Keeps, UnitOutcome};

/// Called only after the connection has confirmed this installation. An
/// unpaid unit is tried by a new action; already-absent material is done.
pub async fn pay(
    record: &dyn TheRecordAnUploadWrites,
    files: &mut impl TheMachinesFileSurface,
    server: &Identity,
    installation: &Identity,
) -> usize {
    let mut debts = Vec::new();
    record.with_the_record(&mut |connection| {
        debts = submission_material::owed(connection, server, installation).unwrap_or_default();
    });
    let mut paid = 0;
    for debt in debts {
        let names = [
            format!("the material set at {}", debt.material_folder),
            format!("the remap material at {}", debt.remap_path),
            format!(
                "the empty output folder at {}",
                debt.output_folder.as_deref().unwrap_or("none was made")
            ),
        ];
        let units: Vec<&str> = names
            .iter()
            .enumerate()
            .filter(|(at, _)| !debt.done[*at])
            .map(|(_, name)| name.as_str())
            .collect();
        let mut action = None;
        record.with_the_record(&mut |connection| {
            action = OutcomeRecords::over(connection)
                .enumerate(crate::submitting::THE_CLEANUP_DEBT_OF_A_SUBMISSION, &units)
                .ok();
        });
        let Some(action) = action else {
            continue;
        };
        let mut all_done = true;
        for (unit, name) in names.iter().enumerate() {
            if debt.done[unit] {
                continue;
            }
            let done = match unit {
                0 => remove_material(files, &debt).await,
                1 => remove_exact_file(files, &debt.remap_path, &debt.remap).await,
                _ => match debt.output_folder.as_deref() {
                    Some(path) => absent_is_done(files.remove_one_empty_directory(path).await),
                    None => true,
                },
            };
            let mut settled = false;
            record.with_the_record(&mut |connection| {
                let ended = OutcomeRecords::over(connection).a_unit_ended(
                    &action,
                    name,
                    if done {
                        UnitOutcome::Done
                    } else {
                        UnitOutcome::Failed
                    },
                );
                if ended.is_ok() && done {
                    settled = submission_material::settled(connection, &debt, unit).is_ok();
                }
            });
            all_done &= settled;
        }
        if all_done {
            paid += 1;
        }
    }
    paid
}

fn absent_is_done(result: Result<(), WhatTheMachineSaid>) -> bool {
    matches!(result, Ok(()) | Err(WhatTheMachineSaid::ThePathIsNotThere))
}

async fn remove_exact_file(
    files: &mut impl TheMachinesFileSurface,
    path: &str,
    expected: &[u8],
) -> bool {
    match files.read_whole(path).await {
        Err(WhatTheMachineSaid::ThePathIsNotThere) => true,
        Ok(bytes) if bytes == expected => absent_is_done(files.remove_one_file(path).await),
        _ => false,
    }
}

async fn remove_material(files: &mut impl TheMachinesFileSurface, debt: &Material) -> bool {
    let entries = match files.list_one_directory(&debt.material_folder).await {
        Ok(entries) => entries,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => return true,
        Err(_) => return false,
    };
    // An accepted job can have attempts and other records. Never remove them,
    // or any unrecorded file someone placed beside the manifest.
    if entries
        .iter()
        .filter(|entry| entry.name != b"." && entry.name != b"..")
        .any(|entry| {
            entry.it_is_a_directory
                || entry.name != smoozy_job_files::manifest::MANIFEST_FILE_NAME.as_bytes()
        })
    {
        return false;
    }
    if !remove_exact_file(
        files,
        &format!(
            "{}/{}",
            debt.material_folder,
            smoozy_job_files::manifest::MANIFEST_FILE_NAME
        ),
        &debt.material,
    )
    .await
    {
        return false;
    }
    absent_is_done(
        files
            .remove_one_empty_directory(&debt.material_folder)
            .await,
    )
}
