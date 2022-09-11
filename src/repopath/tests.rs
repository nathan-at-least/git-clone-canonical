use test_case::test_case;

#[test_case("https://repos.net/a/b/c" => "FAKE/src/repos.net/a/b/c".to_string())]
#[test_case("https://repos.net/a/b/c.git" => "FAKE/src/repos.net/a/b/c".to_string())]
fn test(url: &str) -> String {
    use crate::{get_repo_path, Url};
    use std::path::Path;
    use std::str::FromStr;

    let basedir = Path::new("FAKE/src");
    let url = Url::from_str(url).unwrap();
    let repopath = get_repo_path(basedir, &url);
    repopath.display().to_string()
}
