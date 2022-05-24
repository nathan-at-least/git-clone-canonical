use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, derive_more::From)]
pub enum Error {
    Cli(clap::Error),
    InvalidRepoPath(PathBuf),
    IO(std::io::Error),
    ChildExit(String, Option<i32>),
    LogInit(log::SetLoggerError),
}

impl Error {
    pub fn exit(self) -> ! {
        use Error::*;

        match self {
            Cli(e) => e.exit(),
            InvalidRepoPath(p) => log::error!("invalid repository path; {:?}", p.display()),
            IO(e) => log::error!("{}", e),
            ChildExit(cmd, code) => {
                log::error!("child exited with error code {:?}", code);
                log::error!("child command: {}", cmd);
            }
            LogInit(e) => log::error!("failed to initialize logging; {}", e),
        }

        std::process::exit(1);
    }
}
