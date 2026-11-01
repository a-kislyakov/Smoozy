//! Dummy settings on a throwaway disk, with cuts at the file surface.
use smoozy_server_setup::acts::place_the_telegram_material;
use smoozy_server_setup::readings::{
    AListedThing, TheMachinesFileSurface, WhatTheMachineSaid as Said,
};
use std::path::PathBuf;

struct Files {
    root: PathBuf,
    cut: u8,
    writes: usize,
    renames: usize,
}
impl Files {
    fn new(cut: u8) -> Self {
        let root = std::env::temp_dir().join(format!(
            "smoozy-telegram-cut-{}-{}",
            std::process::id(),
            smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Operation
            )
            .unwrap()
            .as_text()
        ));
        std::fs::create_dir(&root).unwrap();
        for (name, bytes) in [
            ("telegram-token", b"old dummy token".as_slice()),
            ("telegram-role-configuration", b"old dummy role"),
        ] {
            std::fs::write(root.join(name), bytes).unwrap();
        }
        Self {
            root,
            cut,
            writes: 0,
            renames: 0,
        }
    }
    fn material(&self) -> Vec<(String, Vec<u8>)> {
        [
            ("telegram-token", b"new dummy token".as_slice()),
            ("telegram-role-configuration", b"new dummy role"),
        ]
        .into_iter()
        .map(|(name, bytes)| (self.root.join(name).display().to_string(), bytes.to_vec()))
        .collect()
    }
    fn old_is_usable(&self) {
        assert_eq!(
            std::fs::read(self.root.join("telegram-token")).unwrap(),
            b"old dummy token"
        );
        assert_eq!(
            std::fs::read(self.root.join("telegram-role-configuration")).unwrap(),
            b"old dummy role"
        );
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
fn said(e: std::io::Error) -> Said {
    if e.kind() == std::io::ErrorKind::NotFound {
        Said::ThePathIsNotThere
    } else {
        Said::ItRefused
    }
}
impl TheMachinesFileSurface for Files {
    async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, Said> {
        std::fs::read(path).map_err(said)
    }
    async fn write_whole(&mut self, path: &str, bytes: &[u8]) -> Result<(), Said> {
        use std::io::Write;
        self.writes += 1;
        if self.cut == 3 && self.writes == 2 {
            return Err(Said::TheChannelIsGone);
        }
        let mut file = std::fs::File::create(path).map_err(said)?;
        if self.writes == 1 && self.cut == 1 {
            return Err(Said::TheChannelIsGone);
        }
        if self.writes == 1 && self.cut == 2 {
            file.write_all(&bytes[..3]).unwrap();
            return Err(Said::TheChannelIsGone);
        }
        file.write_all(bytes).map_err(said)?;
        if self.cut == 4 && self.writes == 1 {
            std::fs::write(path, b"wrong bytes").unwrap();
        }
        Ok(())
    }
    async fn make_one_directory(&mut self, path: &str) -> Result<(), Said> {
        std::fs::create_dir(path).map_err(said)
    }
    async fn remove_one_file(&mut self, path: &str) -> Result<(), Said> {
        std::fs::remove_file(path).map_err(said)
    }
    async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), Said> {
        std::fs::remove_dir(path).map_err(said)
    }
    async fn list_one_directory(&mut self, _: &str) -> Result<Vec<AListedThing>, Said> {
        unreachable!()
    }
    async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), Said> {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(target, at).map_err(said)
        }
        #[cfg(not(unix))]
        {
            let _ = (target, at);
            Err(Said::ItRefused)
        }
    }
    async fn read_link(&mut self, path: &str) -> Result<Vec<u8>, Said> {
        Ok(std::fs::read_link(path)
            .map_err(said)?
            .to_string_lossy()
            .as_bytes()
            .to_vec())
    }
    async fn replace_by_rename(&mut self, from: &str, to: &str) -> Result<(), Said> {
        std::fs::rename(from, to).map_err(said)?;
        self.renames += 1;
        if self.cut == 5 && self.renames == 2 {
            return Err(Said::TheChannelIsGone);
        }
        Ok(())
    }
}

#[cfg(unix)]
#[tokio::test]
async fn a_cut_during_either_file_keeps_the_old_set_and_a_retry_replaces_both() {
    for cut in 1..=5 {
        let mut files = Files::new(cut);
        let material = files.material();
        let placed = place_the_telegram_material(&mut files, &material).await;
        assert!(
            placed.iter().all(|(_, went)| !went.it_was_carried_out()),
            "no half set is acknowledged"
        );
        files.old_is_usable();
        files.cut = 0;
        let placed = place_the_telegram_material(&mut files, &material).await;
        assert!(
            placed.iter().all(|(_, went)| went.it_was_carried_out()),
            "retry: {placed:?}"
        );
        for (path, bytes) in material {
            assert_eq!(std::fs::read(path).unwrap(), bytes);
        }
    }
}
