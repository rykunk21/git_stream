use anyhow::Result;
use git2::{Repository, Time};
use polars::prelude::*;
use std::path::Path;

pub struct GitHistory {
    repo: Repository,
}

impl GitHistory {
    /// Create a new GitHistory instance from a repository path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }

    /// Convert git history to a DataFrame
    pub fn to_df(&self, since: Option<&str>, author: Option<&str>) -> Result<DataFrame> {
        let mut commit_hashes = Vec::new();
        let mut authors = Vec::new();
        let mut messages = Vec::new();
        let mut timestamps = Vec::new();

        // Initialize walk through git history
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        // Collect basic commit information
        for oid in revwalk {
            let commit = self.repo.find_commit(oid?)?;

            // Skip if doesn't match author filter
            if let Some(author_filter) = author {
                if !commit.author().name().unwrap_or("").contains(author_filter) {
                    continue;
                }
            }

            // Collect basic commit data
            commit_hashes.push(commit.id().to_string());
            authors.push(commit.author().name().unwrap_or("").to_string());
            messages.push(commit.message().unwrap_or("").to_string());
            timestamps.push(commit.time().seconds());
        }

        // Create DataFrame with basic columns
        let df = df!(
            "commit_hash" => commit_hashes,
            "author" => authors,
            "message" => messages,
            "timestamp" => timestamps
        )?;

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_history_creation() {
        let history = GitHistory::new(".").unwrap();
        let df = history.to_df(None, None).unwrap();
        assert!(df.shape().0 > 0); // Should have at least one commit
    }
}
