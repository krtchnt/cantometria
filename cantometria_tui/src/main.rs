use std::error::Error;

use app::{App, CurrentScreen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.current_screen = CurrentScreen::SelectingMidi,
                    _ => {}
                },
                CurrentScreen::SelectingMidi => match key.code {
                    KeyCode::Left => app.current_screen = CurrentScreen::Main,
                    KeyCode::Up => app.midi_path_list.state.select_previous(),
                    KeyCode::Down => app.midi_path_list.state.select_next(),
                    KeyCode::Right if app.midi_path_list.state.selected().is_some() => {
                        app.current_screen = CurrentScreen::SelectingWav;
                    }
                    _ => {}
                },
                CurrentScreen::SelectingWav => match key.code {
                    KeyCode::Left => app.current_screen = CurrentScreen::SelectingMidi,
                    KeyCode::Up => app.wav_path_list.state.select_previous(),
                    KeyCode::Down => app.wav_path_list.state.select_next(),
                    KeyCode::Right if app.wav_path_list.state.selected().is_some() => {
                        app.current_screen = CurrentScreen::ConfirmingSelection;
                    }
                    _ => {}
                },
                CurrentScreen::ConfirmingSelection => match key.code {
                    KeyCode::Left => app.current_screen = CurrentScreen::SelectingWav,
                    KeyCode::Right => app.current_screen = CurrentScreen::Grading,
                    _ => {}
                },
                CurrentScreen::Grading => {
                    if key.code == KeyCode::Right {
                        app.current_screen = CurrentScreen::Main;
                        app.midi_path_list.state.select(None);
                        app.wav_path_list.state.select(None);
                    }
                }
            }
        }
    }
}
