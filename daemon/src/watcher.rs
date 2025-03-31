use chrono::{DateTime, Utc};
use notify::{RecursiveMode, Watcher, RecommendedWatcher, Config, EventKind};
use std::{
    path::PathBuf,
    sync::mpsc::channel,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use db::database::save_session;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::process;

pub fn start_watching(path: PathBuf) {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();
    watcher.watch(&path, RecursiveMode::Recursive).unwrap();

    println!("👀 Watching {:?} for changes (press Ctrl+C to stop)...", path);

    let session_start: Arc<Mutex<Option<DateTime<Utc>>>> = Arc::new(Mutex::new(None));
    let _last_change_time: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let changed_files: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Handle Ctrl+C
    {
        let session_start = Arc::clone(&session_start);
        let _last_change_time = Arc::clone(&_last_change_time);
        let changed_files = Arc::clone(&changed_files);
        let stop_flag = Arc::clone(&stop_flag);
        let project_path = path.display().to_string();

        ctrlc::set_handler(move || {
            println!("\n🛑 Ctrl+C detected. Saving session if active...");

            let start = session_start.lock().unwrap().take();
            let files = changed_files.lock().unwrap().join(",");

            if let Some(start_time) = start {
                let end_time = Utc::now();
                let _ = save_session(start_time, end_time, &project_path, &files);
                println!("✅ Session saved on exit: {} → {}", start_time, end_time);
            } else {
                println!("ℹ️ No active session to save.");
            }

            stop_flag.store(true, Ordering::SeqCst);
        }).expect("Error setting Ctrl+C handler");
    }

    let session_timeout = Duration::from_secs(300); // 5 minutes

    while !stop_flag.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(event)) => {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)) {
                    let now = Utc::now();
                    let mut session = session_start.lock().unwrap();
                    let mut last = _last_change_time.lock().unwrap();
                    let mut files = changed_files.lock().unwrap();

                    if session.is_none() {
                        *session = Some(now);
                    }
                    *last = Some(Instant::now());

                    for path in event.paths {
                        files.push(path.display().to_string());
                    }

                    println!("📁 File change detected at {}", now);
                }
            }
            Ok(Err(e)) => {
                eprintln!("❌ Watch error: {:?}", e);
            }
            Err(_) => {
                let mut session = session_start.lock().unwrap();
                let mut last = _last_change_time.lock().unwrap();
                let mut files = changed_files.lock().unwrap();

                if let (Some(start_time), Some(last_time)) = (*session, *last) {
                    if last_time.elapsed() >= session_timeout {
                        let end_time = Utc::now();
                        let file_str = files.join(",");
                        let _ = save_session(start_time, end_time, &path.display().to_string(), &file_str);
                        println!("✅ Session saved after idle timeout: {} → {}", start_time, end_time);

                        *session = None;
                        *last = None;
                        files.clear();
                    }
                }
            }
        }
    }

    println!("👋 Exiting watcher loop.");
    process::exit(0);
}