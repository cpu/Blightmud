use crate::event::Event;
use crate::VERSION;
use anyhow::Result;
use git2::build::{CloneLocal, RepoBuilder};
use git2::Repository;
use std::cmp::Ordering;
use std::{sync::mpsc::Sender, thread};

const BLIGHTMUD_GIT_URL: &str = "https://github.com/Blightmud/Blightmud.git";
const BLIGHTMUD_RELEASE_URL_PREFIX: &str = "https://github.com/Blightmud/Blightmud/releases/tag/";

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
trait FetchVersionInformation {
    fn fetch(&self) -> Vec<String>;
}

struct Fetcher {}

impl Fetcher {
    fn new() -> Self {
        Self {}
    }
}

impl Fetcher {
    fn blightmud_repo() -> Result<Repository> {
        let clone_dir = crate::DATA_DIR.join("bare");
        if clone_dir.is_dir() {
            let repo = Repository::discover(clone_dir)?;
            {
                let mut origin_remote = repo.find_remote("origin")?;
                origin_remote.fetch(&["main"], None, None)?;
            }
            Ok(repo)
        } else {
            let mut rbuilder = RepoBuilder::new();
            rbuilder.bare(true);
            rbuilder.clone_local(CloneLocal::Auto);
            Ok(rbuilder.clone(BLIGHTMUD_GIT_URL, &clone_dir)?)
        }
    }
}

impl FetchVersionInformation for Fetcher {
    fn fetch(&self) -> Vec<String> {
        // Best-effort fetch tags of the form "v*" from the Blightmud repo. Provide an empty
        // vec if something goes wrong.
        let mut version_tags = Fetcher::blightmud_repo()
            .map(|repo| {
                repo.tag_names(Some("v*"))
                    .map(|tag_array| {
                        tag_array
                            .iter()
                            .filter_map(|opt_t| opt_t.map(|tag| tag.to_owned()))
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or(Vec::default())
            })
            .unwrap_or(Vec::default());

        // Sort in descending order so newest is first.
        version_tags.sort_by(|a, b| b.cmp(a));
        version_tags
    }
}

fn run(writer: Sender<Event>, current: &str, fetcher: &dyn FetchVersionInformation) {
    // If we fetched tags and found a latest version...
    if let Some(latest) = fetcher.fetch().first() {
        // And the latest version is greater than the current version...
        if let Ordering::Greater = latest.cmp(&current.to_owned()) {
            // Then write information to tell the user where to get the upgrade.
            let url = format!("{}{}", BLIGHTMUD_RELEASE_URL_PREFIX, latest);
            writer
                .send(Event::Info(format!(
                    "There is a newer version of Blightmud available. (current: {current}, new: {latest})"
                )))
                .unwrap();
            writer
                .send(Event::Info(format!(
                    "Visit {url} to upgrade to latest version"
                )))
                .unwrap();
        }
    }
}

pub fn check_latest_version(writer: Sender<Event>) {
    thread::Builder::new()
        .name("check-version-thread".to_string())
        .spawn(move || {
            let fetcher = Fetcher::new();
            let version = format!("v{VERSION}");
            run(writer, &version, &fetcher);
        })
        .ok();
}

#[cfg(test)]
mod test_version_diff {
    use super::*;
    use std::sync::mpsc::{channel, Receiver};

    #[test]
    fn test_check() {
        let mut fetcher = MockFetchVersionInformation::new();
        let (writer, reader): (Sender<Event>, Receiver<Event>) = channel();

        fetcher.expect_fetch().times(1).returning(|| {
            vec![
                "v10.0.0".to_owned(),
                "v9.0.0".to_owned(),
                "v8.0.0".to_owned(),
            ]
        });

        run(writer, "v1.0.0", &fetcher);
        assert_eq!(
            reader.try_recv().unwrap(),
            Event::Info(
                "There is a newer version of Blightmud available. (current: v1.0.0, new: v10.0.0)"
                    .to_string()
            )
        );
        assert_eq!(
            reader.try_recv().unwrap(),
            Event::Info("Visit https://github.com/Blightmud/Blightmud/releases/tag/v10.0.0 to upgrade to latest version".to_string())
        );
    }

    #[test]
    fn test_no_new_version() {
        let mut fetcher = MockFetchVersionInformation::new();
        let (writer, reader): (Sender<Event>, Receiver<Event>) = channel();

        fetcher
            .expect_fetch()
            .times(1)
            .returning(|| vec!["v1.0.0".to_owned(), "v0.9.9".to_owned()]);

        run(writer, "v1.0.0", &fetcher);
        assert!(reader.try_recv().is_err());
    }

    #[test]
    fn test_no_data() {
        let mut fetcher = MockFetchVersionInformation::new();
        let (writer, reader): (Sender<Event>, Receiver<Event>) = channel();

        fetcher.expect_fetch().times(1).returning(|| Vec::default());

        run(writer, "v1.0.0", &fetcher);
        assert!(reader.try_recv().is_err());
    }
}
