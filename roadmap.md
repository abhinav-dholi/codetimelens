## Feature Summary
------------------------------

🔧 ROLLED OUT
--------------------------

1. Session Tracking
   - Automatically starts on first file change.
   - Ends after 5 minutes of inactivity or Ctrl+C.
   - File changes tracked using the `notify` crate.
   - Sessions saved to local SQLite DB.

2. Session History
   - Grouped by date.
   - Displays start/end time, duration, and project path.
   - Summary includes total sessions, total time, and average duration.

3. Session Status
   - Shows details of the latest session (project path, start → end, duration).

4. Export
   - JSON: `export.json`
   - CSV:  `export.csv`
   - Files generated in the project directory.

5. TUI Dashboard (Initial)
   - Basic placeholder interface with `ratatui`.
   - Launchable via `codetimelens tui`.

6. Git Insight
   - Parses the last 10 commits in a given Git repo.
   - Displays timestamp, author, and commit summary.

7. Update Command
   - Runs `cargo install --path ./codetimelens --force` to reinstall from source.

8. Uninstall Command
   - Runs `cargo uninstall codetimelens` to remove the CLI.

9. Fully Modular Workspace
   - Crates: cli, core, db, daemon, tui_viewer, codetimelens (binary)

----------------------------------------

🚀 ROADMAP
----------------------------------------

1. TUI Dashboard (Full)
   - Timeline view of coding activity.
   - Navigation between days/weeks.
   - Git-insight overlay.

2. Session Insights (via SessionTracker)
   - Most productive day/time.
   - Session length trends.
   - Project-specific engagement.

3. Filtering & Search
   - `codetimelens history --project X --since DATE`
   - Tags or labels for sessions.

4. Git-to-Session Correlation
   - Match Git commits with tracked sessions.
   - Highlight coding effort vs commit frequency.

5. Configurable Idle Timeout
   - Via CLI flag or config file.

6. Live Status Reporting
   - Background daemon or socket.
   - Real-time time tracking indicators.

7. VSCode Plugin (optional)
   - Start/stop tracker from inside VSCode.
   - Embedded dashboard panel.

8. Markdown / HTML Reports
   - Weekly/monthly summary generation.
   - Exportable time usage for freelance/dev contracts.

9. Session Recovery
   - If the tracker is killed or crashes, partial session is recovered.

10. Cross-platform Binaries (via GitHub Actions)
    - Prebuilt binaries for Linux/macOS/Windows.
    - `brew`, `scoop`, or `cargo install` options.

----------------------------------------

Maintained by: ABHINAV DHOLI