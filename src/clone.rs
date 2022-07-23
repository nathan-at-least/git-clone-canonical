use crate::{get_repo_path, git, log_init, BaseDir, Error, Result, Url};
use std::path::PathBuf;

pub fn clone(basedir: BaseDir, url: Url) -> Result<()> {
    log_init()?;
    let repopath = get_repo_path(basedir, &url);
    log::info!("repository path {:?}", repopath.display());
    if repopath.exists() {
        git::fetch(&repopath, url)
    } else {
        new_clone(repopath, url)
    }
}

fn new_clone(repopath: PathBuf, url: Url) -> Result<()> {
    let parent = repopath
        .parent()
        .ok_or_else(|| Error::InvalidRepoPath(repopath.clone()))?;
    if !parent.exists() {
        log::info!("creating parent directory {:?}", parent.display());
        std::fs::create_dir_all(parent)?;
    }
    git::clone(parent, url)
}
