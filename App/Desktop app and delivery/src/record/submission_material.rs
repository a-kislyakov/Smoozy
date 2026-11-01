//! The exact material kept before a submission, and the debt of a never-taken job.
//! `Rendering` S4b and S9. A candidate is not permission to remove anything.
use super::RecordFailure;
use smoozy_state::identities::{Identity, IdentityKind};

#[derive(Clone, Debug)]
pub struct Material {
    pub job: Identity,
    pub server: Identity,
    pub installation: Identity,
    pub material_folder: String,
    pub material: Vec<u8>,
    pub remap_path: String,
    pub remap: Vec<u8>,
    pub output_folder: Option<String>,
    pub done: [bool; 3],
}
impl Material {
    pub fn keep(&self, connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
        connection.execute(
            "INSERT INTO submission_material
            (job, server, installation, material_folder, material, remap_path, remap, output_folder)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                self.job.as_text(),
                self.server.as_text(),
                self.installation.as_text(),
                self.material_folder,
                self.material,
                self.remap_path,
                self.remap,
                self.output_folder
            ],
        )?;
        Ok(())
    }
}

pub fn owed(
    connection: &rusqlite::Connection,
    server: &Identity,
    installation: &Identity,
) -> Result<Vec<Material>, RecordFailure> {
    let mut query = connection.prepare(
        "SELECT job, material_folder, material, remap_path, remap, output_folder,
        material_done, remap_done, folder_done FROM submission_material
        WHERE server = ?1 AND installation = ?2 AND debt = 1
        AND NOT (material_done = 1 AND remap_done = 1 AND folder_done = 1)
        AND NOT EXISTS (SELECT 1 FROM job_record WHERE job_record.job = submission_material.job)",
    )?;
    let rows = query.query_map(
        rusqlite::params![server.as_text(), installation.as_text()],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Vec<u8>>(4)?,
                row.get::<_, Option<String>>(5)?,
                [
                    row.get::<_, bool>(6)?,
                    row.get::<_, bool>(7)?,
                    row.get::<_, bool>(8)?,
                ],
            ))
        },
    )?;
    let mut out = Vec::new();
    for row in rows {
        let (job, material_folder, material, remap_path, remap, output_folder, done) = row?;
        if let Some(job) = Identity::read_back(IdentityKind::Job, &job) {
            out.push(Material {
                job,
                server: server.clone(),
                installation: installation.clone(),
                material_folder,
                material,
                remap_path,
                remap,
                output_folder,
                done,
            });
        }
    }
    Ok(out)
}

pub fn settled(
    connection: &rusqlite::Connection,
    material: &Material,
    unit: usize,
) -> Result<(), RecordFailure> {
    let column = ["material_done", "remap_done", "folder_done"][unit];
    connection.execute(
        &format!(
            "UPDATE submission_material SET {column} = 1
        WHERE job = ?1 AND server = ?2 AND installation = ?3 AND debt = 1"
        ),
        rusqlite::params![
            material.job.as_text(),
            material.server.as_text(),
            material.installation.as_text()
        ],
    )?;
    Ok(())
}

/// **The kept material goes with its History entry, when it owes nothing** —
/// so the table does not grow without end (the check of record version 45,
/// 2026-10-19). Called by `super::deletions::TheDeletions::perform` inside the
/// transaction that removes the entry, **before the job's row goes**, because
/// whether the job was accepted is read from that row.
///
/// A row is removed only when **its job was accepted** (the row is
/// `confirmed`, and no debt was ever made of it) or **its cleanup debt is
/// settled** (all three units done). A row whose debt is unpaid, or whose job
/// is unconfirmed, is never removed here: it is still the only record of what
/// to clean off the server.
///
/// Older, hand-made test records may predate the table, so a record without
/// it prunes nothing.
pub fn pruned_with_its_entry(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<(), RecordFailure> {
    let has_the_table: bool = connection.query_row(
        "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = 'submission_material'",
        [],
        |row| row.get(0),
    )?;
    if !has_the_table {
        return Ok(());
    }
    connection.execute(
        "DELETE FROM submission_material WHERE job = ?1 AND (
            (debt = 0 AND EXISTS (
                SELECT 1 FROM job_record WHERE job_record.job = ?1 AND standing = 'confirmed'
            ))
            OR (debt = 1 AND material_done = 1 AND remap_done = 1 AND folder_done = 1)
        )",
        [job.as_text()],
    )?;
    Ok(())
}
