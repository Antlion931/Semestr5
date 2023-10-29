use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Screen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.get_current_screen() {
        Screen::View => match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => app.should_quit = true,
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('l') | KeyCode::Char('L') => {
                app.toggle_screen()
            }
            KeyCode::Char('j') | KeyCode::Char('J') => app.scrol_down(),
            KeyCode::Char('k') | KeyCode::Char('K') => app.scrol_up(),
            KeyCode::Char('r') | KeyCode::Char('R') => app.reset_zoom(),
            KeyCode::Char('m') | KeyCode::Char('M') => app.set_to_minimal(),
            KeyCode::Char('w') | KeyCode::Char('W') => {
                app.max_y += 1;
                app.min_y += 1;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                app.max_y -= 1;
                app.min_y -= 1;
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                app.max_x += 1;
                app.min_x += 1;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                app.max_x -= 1;
                app.min_x -= 1;
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                app.max_x -= 10;
                app.min_x += 10;

                app.max_y -= 5;
                app.min_y += 5;
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                app.max_x += 10;
                app.min_x -= 10;

                app.max_y += 5;
                app.min_y -= 5;
            }
            _ => {}
        },
        Screen::Results => match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => app.should_quit = true,
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('l') | KeyCode::Char('L') => {
                app.toggle_screen()
            }
            _ => {}
        },
    }
}
