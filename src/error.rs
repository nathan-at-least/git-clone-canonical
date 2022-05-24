pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Cli(clap::Error),
}

impl Error {
    pub fn exit(self) -> ! {
        use Error::*;

        match self {
            Cli(e) => e.exit(),
        }
    }
}
