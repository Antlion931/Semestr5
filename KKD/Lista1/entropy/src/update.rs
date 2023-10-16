use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('k') | KeyCode::Char('K') => app.scrol_up(),
        KeyCode::Char('j') | KeyCode::Char('J') => app.scrol_down(),
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('l') | KeyCode::Char('L') => {
            app.toggle_screen()
        }
        _ => {}
    };
}
