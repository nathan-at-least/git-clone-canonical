use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Cli(clap::Error),
    InvalidRepoPath(PathBuf),
    IO(std::io::Error),
    ChildExit(String, Option<i32>),
}

impl Error {
    pub fn exit(self) -> ! {
        use Error::*;

        match self {
            Cli(e) => e.exit(),
            InvalidRepoPath(p) => log::error!("Invalid repository path: {:?}", p.display()),
            IO(e) => log::error!("{}", e),
            ChildExit(cmd, code) => {
                log::error!("Child exited with error code {:?}", code);
                log::error!("Child command: {}", cmd);
            }
        }

        std::process::exit(1);
    }
}
