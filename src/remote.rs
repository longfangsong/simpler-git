use std::path::Path;

use git2::{Cred, RemoteCallbacks, build::RepoBuilder};

use crate::LocalRepository;


#[derive(Debug)]
pub struct RemoteRepository<const TMP: &'static str> {
    pub owner: String,
    pub name: String,
}

impl<const TMP: &'static str> RemoteRepository<TMP> {
    pub fn clone<F>(
        &self,
        dir: impl AsRef<Path>,
        credential: Option<F>,
    ) -> Result<LocalRepository, git2::Error>
    where
        F: Fn() -> Result<Cred, git2::Error>,
    {
        let mut callbacks = RemoteCallbacks::new();
        if let Some(credential) = credential {
            callbacks.credentials(move |_url, _username_from_url, _allowed_types| credential());
        }
        let mut fetch_option = git2::FetchOptions::new();
        fetch_option.remote_callbacks(callbacks);
        let repo = RepoBuilder::new().fetch_options(fetch_option).clone(
            &format!("{}/{}/{}", TMP, self.owner, self.name),
            dir.as_ref(),
        )?;
        Ok(LocalRepository { inner: repo })
    }
}

pub type GitHubRepository = RemoteRepository<"https://github.com">;
pub type GitEERepository = RemoteRepository<"https://gitee.com">;
