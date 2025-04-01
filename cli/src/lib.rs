use clap::{Parser, Subcommand};
use core::session_tracker::{SessionTracker, GitAnalyzer};
use daemon::watcher::start_watching;
use db::database::{init_db, get_history, export_sessions};
use tui_viewer::dashboard::run_tui;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc, NaiveDate};
use std::process::Command;

#[derive(Parser)]
#[command(name = "CodeTimeLens")]
#[command(about = "Track your code time and Git activity", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long)]
        path: PathBuf,
    },
    History {},
    Status {},
    Tui {},
    Export {
        #[arg(short, long)]
        format: String,
    },
    GitInsight {
        #[arg(short, long)]
        path: PathBuf,
    },
    Uninstall {},
    Update {},
}

pub fn run() {
    let cli = Cli::parse();
    let _ = init_db();

    match cli.command {
        Commands::Start { path } => {
            println!("Starting to watch path: {:?}", path);
            start_watching(path);
        }

        Commands::History {} => {
            if let Ok(history) = get_history() {
                let mut sessions_by_day: HashMap<NaiveDate, Vec<(DateTime<Utc>, DateTime<Utc>, String)>> = HashMap::new();
                let mut total_minutes = 0;

                for entry in &history {
                    if let (Ok(start), Ok(end)) = (
                        DateTime::parse_from_rfc3339(&entry.start).map(|d| d.with_timezone(&Utc)),
                        DateTime::parse_from_rfc3339(&entry.end).map(|d| d.with_timezone(&Utc)),
                    ) {
                        let day = start.date_naive();
                        let duration = end.signed_duration_since(start).num_minutes();
                        total_minutes += duration;
                        sessions_by_day
                            .entry(day)
                            .or_insert(vec![])
                            .push((start, end, entry.project_path.clone()));
                    }
                }

                let mut days: Vec<_> = sessions_by_day.into_iter().collect();
                days.sort_by_key(|(day, _)| *day);

                for (day, sessions) in days {
                    println!("\n📅 {}", day);
                    for (start, end, path) in sessions {
                        let duration = end.signed_duration_since(start).num_minutes();
                        println!(
                            "  🟢 {} → {} ({} min) | {}",
                            start.format("%H:%M"),
                            end.format("%H:%M"),
                            duration,
                            path
                        );
                    }
                }

                let total_sessions = history.len();
                let avg = if total_sessions > 0 {
                    total_minutes / total_sessions as i64
                } else {
                    0
                };

                println!("\n🧾 Summary:");
                println!("- Total sessions: {}", total_sessions);
                println!("- Total time: {}h {}m", total_minutes / 60, total_minutes % 60);
                println!("- Avg session: {}m", avg);
            } else {
                println!("No history available.");
            }
        }

        Commands::Status {} => {
            if let Ok(history) = get_history() {
                if let Some(latest) = history.last() {
                    if let (Ok(start), Ok(end)) = (
                        DateTime::parse_from_rfc3339(&latest.start).map(|d| d.with_timezone(&Utc)),
                        DateTime::parse_from_rfc3339(&latest.end).map(|d| d.with_timezone(&Utc)),
                    ) {
                        let duration = end.signed_duration_since(start).num_minutes();
                        println!("🔍 Latest session:");
                        println!("📂 Path: {}", latest.project_path);
                        println!("⏰ {} → {} ({} min)",
                            start.format("%Y-%m-%d %H:%M"),
                            end.format("%H:%M"),
                            duration
                        );
                    }
                } else {
                    println!("ℹ️ No session history found.");
                }
            } else {
                println!("❌ Failed to fetch session history.");
            }
        }

        Commands::Tui {} => {
            let _ = run_tui();
        }

        Commands::Export { format } => {
            let _ = export_sessions(&format);
        }
        Commands::GitInsight { path } => {
            match GitAnalyzer::analyze_recent_commits(path) {
                Ok(_) => {},
                Err(e) => eprintln!("❌ Git insight error: {}", e),
            }
        }
        Commands::Uninstall {} => {
            println!("📦 Uninstalling CodeTimeLens...");
            let status = Command::new("cargo")
                .arg("uninstall")
                .arg("codetimelens")
                .status()
                .expect("Failed to execute uninstall");
        
            if status.success() {
                println!("✅ CodeTimeLens successfully uninstalled.");
            } else {
                println!("❌ Uninstall failed.");
            }
        }
        
        Commands::Update {} => {
            println!("⬆️ Updating CodeTimeLens from source...");
            let status = Command::new("cargo")
                .arg("install")
                .arg("--path")
                .arg("./codetimelens")
                .arg("--force")
                .status()
                .expect("Failed to execute update");
        
            if status.success() {
                println!("✅ CodeTimeLens updated successfully.");
            } else {
                println!("❌ Update failed.");
            }
        }
    }
}