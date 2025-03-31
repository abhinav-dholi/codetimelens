use chrono::{DateTime, Utc};
use git2::Repository;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Session {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub project_path: String,
    pub files: Vec<String>,
}

pub struct SessionTracker;

impl SessionTracker {
    pub fn create_session(start: DateTime<Utc>, end: DateTime<Utc>, path: &str, files: Vec<String>) -> Session {
        Session {
            start,
            end,
            project_path: path.to_string(),
            files,
        }
    }
}

pub struct GitAnalyzer;

impl GitAnalyzer {
    pub fn analyze_commits(path: &str) {
        let repo = Repository::open(Path::new(path)).expect("Failed to open Git repo");
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();

        for oid in revwalk.take(10) {
            if let Ok(oid) = oid {
                let commit = repo.find_commit(oid).unwrap();
                println!(
                    "Commit {}: {}",
                    commit.id(),
                    commit.summary().unwrap_or("No message")
                );
            }
        }
    }
}