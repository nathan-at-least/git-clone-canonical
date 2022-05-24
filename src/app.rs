use crate::{clone, Options, Result};

pub fn run() {
    if let Some(error) = run_result().err() {
        error.exit();
    }
}

pub fn run_result() -> Result<()> {
    let Options { basedir, url } = Options::try_parse()?;
    clone(basedir, url)
}
