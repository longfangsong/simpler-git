use git2::{Cred, RemoteCallbacks, IndexAddOption, Oid, Direction};

pub struct LocalRepository {
    pub inner: git2::Repository,
}

impl LocalRepository {
    pub fn pull<F>(&mut self, branch_name: &str, credential: Option<F>) -> Result<(), git2::Error>
    where
        F: Fn() -> Result<Cred, git2::Error>,
    {
        let mut callbacks = RemoteCallbacks::new();
        if let Some(credential) = credential {
            callbacks.credentials(move |_url, _username_from_url, _allowed_types| credential());
        }
        let mut fetch_option = git2::FetchOptions::new();
        fetch_option.remote_callbacks(callbacks);
        self.inner
            .find_remote("origin")?
            .fetch(&[branch_name], Some(&mut fetch_option), None)?;
        let fetch_head = self.inner.find_reference("FETCH_HEAD")?.peel_to_commit()?;
        self.inner.branch(branch_name, &fetch_head, false).unwrap();
        self.inner
            .set_head(&format!("refs/heads/{}", branch_name))?;
        self.inner.checkout_head(None)?;
        self.inner
            .reset(fetch_head.as_object(), git2::ResetType::Hard, None)?;
        Ok(())
    }

    pub fn add_all(&mut self) -> Result<git2::Oid, git2::Error> {
        let mut index = self.inner.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write_tree()
    }

    pub fn commit(
        &mut self,
        tree_id: Oid,
        message: &str,
        signature: git2::Signature,
    ) -> Result<(), git2::Error> {
        let parent_commit = self.inner.head().unwrap().peel_to_commit().unwrap();
        let tree = self.inner.find_tree(tree_id).unwrap();
        self.inner.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;
        Ok(())
    }

    pub fn push<F>(&mut self, credential: F, branch_name: &str) -> Result<(), git2::Error>
    where
        F: Fn() -> Result<Cred, git2::Error>,
    {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| credential());
        let mut push_option = git2::PushOptions::new();
        push_option.remote_callbacks(callbacks);
        let mut remote = self.inner.find_remote("origin")?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| credential());
        remote.connect_auth(Direction::Push, Some(callbacks), None)?;
        remote.push(
            &[format!("HEAD:refs/heads/{}", branch_name)],
            Some(&mut push_option),
        )
    }
}
