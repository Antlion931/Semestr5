use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Screen, SavingMode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.get_current_screen() {
        Screen::Entropy | Screen::ConditionalEntropy => 
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.soft_quit(),
                KeyCode::Char('k') | KeyCode::Char('K') => app.scrol_up(),
                KeyCode::Char('j') | KeyCode::Char('J') => app.scrol_down(),
                KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('l') | KeyCode::Char('L') => {
                    app.toggle_screen()
                }
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    app.file_name.clear();
                    match app.get_current_screen() {
                        Screen::Entropy => app.change_screen(Screen::Saving(SavingMode::Entropy)),
                        Screen::ConditionalEntropy => app.change_screen(Screen::Saving(SavingMode::ConditionalEntropy)),
                        _ => unreachable!(),
                    }
                }                
                KeyCode::Char('r') | KeyCode::Char('R') =>{
                    app.file_name.clear();
                    app.change_screen(Screen::Saving(SavingMode::Results));
                }
                _ => {}
            }
        Screen::Saving(_) => match key_event.code {
            KeyCode::Enter => {
                app.save().unwrap();
                app.change_screen(*app.get_previous_screen());
            },
            KeyCode::Backspace => {app.file_name.pop();},
            KeyCode::Esc => app.change_screen(*app.get_previous_screen()),
            KeyCode::Char(value) => app.file_name.push(value),
            _ => {}
    }
        Screen::Exiting => match key_event.code {
            KeyCode::Char('n') | KeyCode::Char('N') => app.change_screen(*app.get_previous_screen()),
            KeyCode::Char('y') | KeyCode::Char('Y') => app.hard_quit(),
            _ => {}
        },
    }
}
