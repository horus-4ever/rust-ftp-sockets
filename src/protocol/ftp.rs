use std::fmt::{Debug};
use std::convert::{Into};
use std::iter::{FromIterator};

#[derive(Debug)]
pub struct Header {
    pub paths: Vec<EntryKind>
}

#[derive(Debug)]
pub enum EntryKind {
    File(String, u64),
    Directory(String)
}

impl EntryKind {
    pub fn get_type_num(&self) -> u8 {
        match self {
            Self::File(_, _) => 0,
            Self::Directory(_) => 1
        }
    }
}

impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for path in self.paths {
            match &path {
                EntryKind::File(name, size) => {
                    let total_size = name.len() + std::mem::size_of::<u64>() + 1;
                    result.extend(&total_size.to_be_bytes());
                    result.push(path.get_type_num());
                    result.extend(name.as_bytes());
                    result.extend(&size.to_be_bytes());
                }
                EntryKind::Directory(name) => {
                    let total_size = name.len() + 1;
                    result.extend(&total_size.to_be_bytes());
                    result.push(path.get_type_num());
                }
            }
        }
        let mut other = Vec::from_iter(result.len().to_be_bytes().iter().cloned());
        other.extend(result);
        other
    }
}