use crate::{BaseDir, Error, Result, Url};
use clap::Parser;

/// Run git clone into a path derived from the source url
#[derive(Debug, Parser)]
#[clap()]
pub struct Options {
    /// base directory for git clone
    #[clap(long, default_value_t)]
    pub basedir: BaseDir,

    /// show the associated path, performing no other operations
    #[clap(long, short)]
    pub show_path: bool,

    /// The git clone url (git-url-like sources are not supported)
    #[clap()]
    pub url: Url,
}

impl Options {
    pub fn try_parse() -> Result<Self> {
        <Self as Parser>::try_parse().map_err(Error::from)
    }
}
