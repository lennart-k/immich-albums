use std::path::{Path, PathBuf};

use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use crate::AlbumMetadata;

pub(crate) trait AlbumLocator {
    /// Checks if a path matches an album path and if so returns the path match
    /// The path match can be used as a default value to initialise a new album.toml if not
    /// existing
    fn matches(&self, path: &Path) -> Option<AlbumMetadata>;
    fn locate_iter(&self) -> impl Iterator<Item = (AlbumMetadata, DirEntry)>;
}

pub(crate) struct DefaultAlbumLocator {
    basepath: PathBuf,
    regex: Regex,
}

///
/// Matches:
/// 2021/2021-10-02 Hallo
/// 2021/2021-10-02-2021-10-05 Hallo 2
/// 2021-10-02-2021-10-05 Hallo
/// Doesn't match:
/// asldkasd
/// 2021-10-02-2021-10-05 Hallo/asd
/// 2021-10-02-2021-10-05 Hallo/raw
impl DefaultAlbumLocator {
    pub(crate) fn new(basepath: PathBuf) -> Self {
        Self {
            basepath,
            regex: Regex::new(r#"^(?P<year>\d+/)?(?P<start_date>\d+-\d{1,2}-\d{1,2})?(?:-(?P<end_date>\d+-\d{1,2}-\d{1,2}))? (?P<name>[^/]+)$"#).unwrap()
        }
    }
}

impl AlbumLocator for DefaultAlbumLocator {
    // {year}/?{start_date?{-end_date}?} {name}
    fn matches(&self, path: &Path) -> Option<AlbumMetadata> {
        if let Ok(relpath) = path.strip_prefix(&self.basepath) {
            let captures = self.regex.captures(relpath.to_str().unwrap())?;
            Some(AlbumMetadata {
                name: captures.name("name").unwrap().as_str().to_owned(),
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn locate_iter(&self) -> impl Iterator<Item = (AlbumMetadata, DirEntry)> {
        WalkDir::new(&self.basepath)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_dir())
            .filter_map(|entry| Some((self.matches(entry.path())?, entry)))
    }
}
