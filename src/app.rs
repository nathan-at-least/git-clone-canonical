use crate::{clone, get_repo_path, Options, Result};

pub fn run() {
    if let Some(error) = run_result().err() {
        error.exit();
    }
}

pub fn run_result() -> Result<()> {
    let Options {
        basedir,
        show_path,
        url,
    } = Options::try_parse()?;

    if show_path {
        println!("{}", get_repo_path(basedir, &url).display());
        Ok(())
    } else {
        clone(basedir, url)
    }
}
