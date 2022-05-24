use crate::{Error, Result, Url};
use std::path::Path;
use std::process::Command;

pub fn clone(parentdir: &Path, url: Url) -> Result<()> {
    let urlstr = url.as_str();
    log::info!("cloning: {:?}", &urlstr);
    run(parentdir, "git", &["clone", urlstr])
}

pub fn fetch(repodir: &Path, url: Url) -> Result<()> {
    let urlstr = url.as_str();
    log::info!("fetching: {:?}", &urlstr);
    run(repodir, "git", &["fetch", urlstr])
}

fn run(cwd: &Path, exec: &str, args: &[&str]) -> Result<()> {
    let mut cmd = Command::new(exec);
    cmd.current_dir(cwd);
    cmd.args(args);
    let cmdstr = format!("{:?}", &cmd);
    log::debug!("Running: {}", &cmdstr);
    let status = cmd.status()?;
    log::debug!("Exit status: {:?}", &status);
    if status.success() {
        Ok(())
    } else {
        Err(Error::ChildExit(cmdstr, status.code()))
    }
}
