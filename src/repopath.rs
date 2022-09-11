use crate::Url;
use std::convert::AsRef;
use std::path::{Path, PathBuf};

pub fn get_repo_path<P>(basedir: P, url: &Url) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut path = basedir.as_ref().to_path_buf();
    path.push(url.domain());
    let mut segments = url.path_segments().peekable();
    while let Some(segment) = segments.next() {
        let segment = if segments.peek().is_none() {
            segment.strip_suffix(".git").unwrap_or(segment)
        } else {
            segment
        };
        path.push(segment);
    }
    path
}

#[cfg(test)]
mod tests;
