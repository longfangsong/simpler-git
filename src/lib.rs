#![feature(adt_const_params)]

mod remote;
mod local;

pub use local::LocalRepository;
pub use remote::RemoteRepository;
pub use remote::{GitEERepository, GitHubRepository};
pub use git2::{Signature, Cred};
pub use git2;

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Signature, Cred};
    use std::fs::File;
    use std::io::Write;
    
    #[test]
    fn fuck() {
        std::fs::remove_dir_all("./lab2").unwrap();
        let github_repo = GitHubRepository {
            owner: "longfangsong".to_string(),
            name: "lab2".to_string(),
        };
        let mut repo = github_repo
            .clone(
                "./lab2",
                Some(|| {
                    Cred::userpass_plaintext(
                        "baipiao-bot",
                        "07b497cd5e083fe53080f8b5eccdd588e3286c5a",
                    )
                }),
            )
            .unwrap();

        repo.pull(
            "patch-1",
            Some(|| {
                Cred::userpass_plaintext("baipiao-bot", "07b497cd5e083fe53080f8b5eccdd588e3286c5a")
            }),
        )
        .unwrap();

        let mut f = File::create("./lab2/new4.txt").unwrap();
        f.write_all("hello".as_bytes()).unwrap();
        drop(f);

        let tree_id = repo.add_all().unwrap();

        let signature = Signature::now("baipiao-bot", "moss_the_bot@163.com").unwrap();
        repo.commit(tree_id, "test", signature).unwrap();
        repo.push(
            || Cred::userpass_plaintext("baipiao-bot", "07b497cd5e083fe53080f8b5eccdd588e3286c5a"),
            "patch-1",
        );
    }
}
