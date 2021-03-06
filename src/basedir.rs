use std::convert::AsRef;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct BaseDir(PathBuf);

impl Default for BaseDir {
    fn default() -> Self {
        BaseDir(
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("src"),
        )
    }
}

impl AsRef<Path> for BaseDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for BaseDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl FromStr for BaseDir {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(BaseDir)
    }
}
