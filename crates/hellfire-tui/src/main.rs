mod app;
mod theme;
mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use app::App;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(stdout))?;

    let mut app = App::new();
    app.search_results = vec![
        "Lightning Bolt".into(),
        "Ragavan, Nimble Pilferer".into(),
        "Blood Moon".into(),
    ];

    let res = run_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    res
}

fn run_loop(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.quit = true;
                    return Ok(());
                }
                KeyCode::Char('v') => app.raise_fist(),
                KeyCode::Down => app.next_result(),
                KeyCode::Up => app.prev_result(),
                _ => {}
            }
        }
        if app.quit {
            return Ok(());
        }
    }
}
