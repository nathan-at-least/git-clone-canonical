use crate::{git, log_init, BaseDir, Error, Result, Url};
use std::path::PathBuf;

pub fn clone(basedir: BaseDir, url: Url) -> Result<()> {
    log_init();
    let repopath = get_repo_path(basedir.into(), &url);
    if repopath.exists() {
        git::fetch(&repopath, url)
    } else {
        new_clone(repopath, url)
    }
}

fn get_repo_path(mut path: PathBuf, url: &Url) -> PathBuf {
    path.push(url.domain());
    for segment in url.path_segments() {
        path.push(segment);
    }
    path
}

fn new_clone(repopath: PathBuf, url: Url) -> Result<()> {
    let parent = repopath
        .parent()
        .ok_or_else(|| Error::InvalidRepoPath(repopath.clone()))?;
    if !parent.exists() {
        log::info!("Creating parent directory: {:?}", parent.display());
        std::fs::create_dir_all(parent)?;
    }
    git::clone(parent, url)
}
