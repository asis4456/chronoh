use crate::error::{Error, Result};
use git2::{IndexAddOption, Repository, Signature};
use std::path::Path;
use tracing::{debug, info};

pub struct GitBridge {
    path: std::path::PathBuf,
    repo: Option<Repository>,
}

impl GitBridge {
    pub async fn new(project_path: &Path) -> Result<Self> {
        Ok(Self {
            path: project_path.to_path_buf(),
            repo: None,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        let repo = Repository::init(&self.path)?;
        self.repo = Some(repo);
        info!("Git repository initialized at {:?}", self.path);
        Ok(())
    }

    pub async fn open(&mut self) -> Result<()> {
        let repo = Repository::open(&self.path)?;
        self.repo = Some(repo);
        Ok(())
    }

    pub async fn is_clean(&self) -> Result<bool> {
        let repo = self
            .repo
            .as_ref()
            .ok_or_else(|| Error::GitError("Repository not initialized".to_string()))?;

        let statuses = repo.statuses(None)?;
        Ok(statuses.is_empty())
    }

    pub async fn commit_all(&self, message: &str) -> Result<String> {
        let repo = self
            .repo
            .as_ref()
            .ok_or_else(|| Error::GitError("Repository not initialized".to_string()))?;

        let mut index = repo.index()?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let signature = Signature::now("Chrono Agent", "agent@chrono.h")?;

        let parents = match repo.head() {
            Ok(head) => {
                let parent = head.peel_to_commit()?;
                vec![parent]
            }
            Err(_) => vec![],
        };

        let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

        let commit_id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parent_refs,
        )?;

        let hash = commit_id.to_string();
        info!("Created commit: {}", &hash[..8]);

        Ok(hash)
    }

    pub async fn get_last_commit(&self) -> Result<Option<String>> {
        let repo = self
            .repo
            .as_ref()
            .ok_or_else(|| Error::GitError("Repository not initialized".to_string()))?;

        match repo.head() {
            Ok(head) => {
                let commit = head.peel_to_commit()?;
                Ok(Some(commit.id().to_string()))
            }
            Err(_) => Ok(None),
        }
    }
}
