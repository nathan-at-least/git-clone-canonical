use crate::Url;
use std::convert::AsRef;
use std::path::{Path, PathBuf};

pub fn get_repo_path<P>(basedir: P, url: &Url) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut path = basedir.as_ref().to_path_buf();
    path.push(url.domain());
    for segment in url.path_segments() {
        path.push(segment);
    }
    path
}
