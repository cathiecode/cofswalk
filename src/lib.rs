use serde::{Deserialize, Serialize};
use std::{
    ffi::{OsStr, OsString},
    fs::{DirEntry, ReadDir},
    path::{Path, PathBuf},
    vec,
};

#[derive(Debug)]
pub enum Error {
    FailedToPopulate,
    SomethingWentWrong(&'static str),
}

type Result<T> = std::result::Result<T, Error>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirWalker {
    base_path: OsString,
    directory_path: Vec<OsString>,
    last_file_name: Option<OsString>,

    #[serde(skip)]
    current_iterator: Option<std::fs::ReadDir>,
}

impl DirWalker {
    pub fn new(path: &OsStr) -> Result<Self> {
        let read_dir = std::fs::read_dir(&path).or(Err(Error::SomethingWentWrong(
            "ed232402-b33e-41c0-8326-b938145d1faf",
        )))?;

        Ok(Self {
            base_path: path.into(),
            directory_path: vec![OsString::from("."), OsString::from(".")],
            current_iterator: None,
            last_file_name: None
        })
    }

    fn current_iterator(&mut self) -> Result<&mut std::fs::ReadDir> {
        if let None = &self.current_iterator {
            self.populate()?;
        }

        Ok(self.current_iterator.as_mut().unwrap())
    }

    fn populate(&mut self) -> Result<()> {
        self.current_iterator =
            Some(self.read_dir(&self.directory_path.iter().collect::<PathBuf>(), self.last_file_name.as_ref().map(|e| e.as_os_str()))?);

        Ok(())
    }

    fn read_dir(&self, path: &Path, skip_until: Option<&OsStr>) -> Result<ReadDir> {
        let path = PathBuf::from(&self.base_path).join(PathBuf::from(path));

        let mut read_dir = std::fs::read_dir(path).unwrap(); //.or(Err(Error::SomethingWentWrong("1f5973a9-66c5-4120-9844-ce2352a34257")))?;

        if let Some(skip_until) = skip_until {
            for entry in &mut read_dir {
                if entry
                    .or(Err(Error::SomethingWentWrong(
                        "062921ba-660e-4258-a1d2-e23e2d1a3534",
                    )))?
                    .file_name()
                    == skip_until
                {
                    break;
                }
            }
        }

        Ok(read_dir)
    }

    fn next(&mut self) -> Result<Option<DirEntry>> {
        let iterator = self.current_iterator()?;

        let result = iterator.next().reverse().or(Err(Error::SomethingWentWrong(
            "cd280b00-0d42-48c8-8578-ab8b039ba491",
        )))?;

        // FIXME: Hard link
        if let Some(result) = &result {
            let file_type = result.file_type().or(Err(Error::SomethingWentWrong(
                "b7d7bddc-aab2-4ae9-abfd-e9d5f2d1de92",
            )))?;

            self.last_file_name = Some(result.file_name().to_os_string());

            if file_type.is_file() {
                
            } else if file_type.is_dir() {
                self.directory_path.push(result.file_name());
                self.current_iterator =
                    Some(self.read_dir(&self.directory_path.iter().collect::<PathBuf>(), None)?);
            } else {
                // Symlink?
            }
        } else {
            let last_dirname = self.directory_path.pop();

            if let Some(last_dirname) = last_dirname {
                // unwind
                self.current_iterator = Some(self.read_dir(
                    &self.directory_path.iter().collect::<PathBuf>(),
                    Some(&last_dirname),
                )?);
                return self.next();
            } else {
                return Ok(None);
            }
        }

        return Ok(result);
    }
}

pub struct DirWalkerIter<'a> {
    walker: &'a mut DirWalker,
}

impl<'a> Iterator for DirWalkerIter<'a> {
    type Item = Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next().reverse()
    }
}

impl<'a> IntoIterator for &'a mut DirWalker {
    type Item = Result<DirEntry>;
    type IntoIter = DirWalkerIter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            walker: self,
        }
    }
}

trait ReverseOptionResult<T, E> {
    fn reverse(self) -> std::result::Result<Option<T>, E>;
}

impl<T, E> ReverseOptionResult<T, E> for Option<std::result::Result<T, E>> {
    fn reverse(self) -> std::result::Result<Option<T>, E> {
        match self {
            Some(Ok(value)) => Ok(Some(value)),
            None => Ok(None),
            Some(Err(err)) => Err(err),
        }
    }
}

trait ReverseResultOption<T, E> {
    fn reverse(self) -> Option<std::result::Result<T, E>>;
}

impl<T, E> ReverseResultOption<T, E> for std::result::Result<Option<T>, E> {
    fn reverse(self) -> Option<std::result::Result<T, E>> {
        match self {
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
