use crate::Url;
use test_case::test_case;

#[test_case("https://github.com/nathan-at-least/git-clone-canonical", "github.com", &["nathan-at-least", "git-clone-canonical"])]
#[test_case("https://github.com/nathan-at-least/git-clone-canonical.git", "github.com", &["nathan-at-least", "git-clone-canonical.git"])]
#[test_case("git@github.com:nathan-at-least/git-clone-canonical.git", "github.com", &["nathan-at-least", "git-clone-canonical.git"])]
fn parse_to_path_segments(
    input: &str,
    expected_domain: &str,
    expected_path_segments: &[&str],
) -> Result<(), <Url as std::str::FromStr>::Err> {
    let url: Url = input.parse()?;
    assert_eq!(url.domain(), expected_domain);
    let segments: Vec<&str> = url.path_segments().collect();
    assert_eq!(segments.as_slice(), expected_path_segments);
    Ok(())
}
