use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

pub fn run_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("CodeTimeLens Dashboard")
            .borders(Borders::ALL);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)]);
        let area = layout.split(size)[0];
        let text = Paragraph::new("Welcome to CodeTimeLens!\nView your session history, usage analytics, and more.");
        f.render_widget(block, size);
        f.render_widget(text, area);
    })?;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    Ok(())
}