use chrono::{DateTime, Utc};
use git2::Repository;

#[derive(Debug, Clone)]
pub struct Session {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub project_path: String,
    pub files: Vec<String>,
}

pub struct SessionTracker;

impl SessionTracker {
    pub fn create_session(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        path: &str,
        files: Vec<String>,
    ) -> Session {
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
    pub fn analyze_recent_commits(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Repository::open(path)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        println!("📚 Recent Git Commits:");

        for (i, oid) in revwalk.enumerate().take(10) {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let author = commit.author();
            let message = commit.summary().unwrap_or("No commit message");

            // Convert commit time to chrono DateTime<Utc>
            let time = commit.time();
            let secs = time.seconds();
            let datetime = DateTime::<Utc>::from_timestamp(secs, 0)
                .ok_or("Invalid timestamp")?;

            println!(
                "- {} | {} | {}",
                datetime.format("%Y-%m-%d %H:%M"),
                author.name().unwrap_or("Unknown"),
                message
            );

            if i == 9 {
                break;
            }
        }

        Ok(())
    }
}