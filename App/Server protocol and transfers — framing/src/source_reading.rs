//! The four values a file on the machine is told not to have changed by.
//!
//! **Nothing here reads them.** Taking the reading is a command-channel act and
//! it stays next door in `smoozy-server-protocol`, which re-exports this shape
//! at `transfers::SourceReading`, the path it has always been reached at. It
//! sits on this side of the split because the machine's own program reads a
//! job's material against these values and must not reach the connecting half
//! to name them.

/// **S13's four values, recorded when a download is arranged.**
///
/// A file on the server can be told not to have changed *"from size, inode,
/// modification time and change time together"*; a save-by-rename changes the
/// inode; and **a hand-restored modification time still moves the change time,
/// which cannot be set back**. It was posed live, where a byte changed
/// mid-fetch and *"the copy returned 0 and delivered a different file"*, with
/// only the timestamps revealing it.
///
/// **This is a command-channel reading and not a subsystem one.** Revision 3's
/// attribute structure carries no inode and no change time, so the two values
/// this rule leans on hardest are two the file-transfer surface does not
/// carry. **Where it is unavailable the resume simply does not happen and the
/// transfer starts over**, which is the same outcome as any difference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceReading {
    pub size: u64,
    pub inode: String,
    pub modification_time: i64,
    pub change_time: i64,
}

impl SourceReading {
    /// **All four are compared and any difference starts the transfer over.**
    pub fn is_the_same_file_as(&self, other: &SourceReading) -> bool {
        self == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_four_source_values_are_compared_and_any_difference_is_a_difference() {
        // S13. A hand-restored modification time still moves the change time,
        // which cannot be set back — so the fourth value is the one that
        // catches the case the other three miss.
        let arranged = SourceReading {
            size: 100,
            inode: "12345".to_string(),
            modification_time: 1_700_000_000,
            change_time: 1_700_000_000,
        };
        let restored_modification_time = SourceReading {
            change_time: 1_700_000_500,
            ..arranged.clone()
        };
        assert!(!arranged.is_the_same_file_as(&restored_modification_time));
        assert!(arranged.is_the_same_file_as(&arranged.clone()));
    }
}
