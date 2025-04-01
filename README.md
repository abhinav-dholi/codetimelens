# CodeTimeLens

CodeTimeLens is a blazing-fast, privacy-respecting developer productivity tool built in Rust. It monitors your coding sessions, tracks file changes, correlates with Git commits, and gives you deep insights into how you code — all while running locally and efficiently.

Whether you’re a solo hacker, freelancer, or engineering lead, CodeTimeLens helps you answer:
- How much time did I actually spend on this feature?
- What files took the most effort?
- What are my most productive hours?
- Can I get a better sense of where my dev time goes?

All locally, with no external dependencies or cloud sync. Ideal for developers, freelancers, researchers, and productivity nerds.


## Installation

### Prerequisite

Make sure you have Rust and Cargo installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### From Source

```bash
git clone https://github.com/abhinav-dholi/codetimelens
cd codetimelens
cargo install --path ./codetimelens --force
```

This installs the `codetimelens` binary globally in `~/.cargo/bin`.

Make sure that path is in your shell's environment:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## Quick Start

### Start Tracking a Project

```bash
codetimelens start --path /path/to/your/project
```

- Session starts on first file change  
- Ends after 5 minutes of inactivity or on `Ctrl+C`  
- Saves session to a local SQLite database  

---

### View Session History

```bash
codetimelens history
```

- Shows daily grouped sessions with start/end time and durations  
- Includes summary of total time and average session length  

---

### Check Latest Session

```bash
codetimelens status
```

- Displays the most recent tracked session  

---

### Export Session Data

```bash
codetimelens export --format json
codetimelens export --format csv
```

- Saves session data to `export.json` or `export.csv` in the current directory  

---

### TUI Dashboard (Coming Soon)

```bash
codetimelens tui
```

- A terminal user interface to visualize time usage (currently in development)  

---

### Maintenance Commands

```bash
codetimelens update      # Reinstall from latest local source
codetimelens uninstall   # Remove CodeTimeLens from system
```

---

You’re now ready to track, explore, and optimize your coding sessions.

Run `codetimelens --help` to view all available commands.


## Contributing and Roadmap

CodeTimeLens is designed to be modular, privacy-first, and open-source.

View upcoming ideas and contribute by checking out the [roadmap.md](./roadmap.md) file.