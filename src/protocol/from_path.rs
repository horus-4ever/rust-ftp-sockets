use crate::{Header, EntryKind};
extern crate walkdir;
use walkdir::WalkDir;
use std::path::{Path};
use std::convert::{TryFrom};

impl TryFrom<&Path> for Header {

    type Error = std::io::Error;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        let mut result: Vec<EntryKind> = vec![];
        if p.is_file() {
            result.push(EntryKind::File(String::from(p.to_str().unwrap()), p.metadata()?.len()));
            return Ok(Self{ paths: result })
        }
        // for each sub-file / sub-directory (and reccursively)
        for entry in WalkDir::new(p) {
            match entry?.path() {
                path if path.is_file() => {
                    result.push(EntryKind::File(
                        String::from(path.to_str().unwrap()),
                        path.metadata()?.len())
                    )
                }
                path if path.is_dir() => {
                    result.push(EntryKind::Directory(String::from(path.to_str().unwrap())))
                }
                _ => unreachable!()
            }
        }
        Ok(Self{ paths: result })
    }
}