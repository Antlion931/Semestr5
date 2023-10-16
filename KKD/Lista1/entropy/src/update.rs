use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, CurrentScreen, SavingMode, PreviousScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Entropy | CurrentScreen::ConditionalEntropy => 
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.soft_quit(),
                KeyCode::Char('k') | KeyCode::Char('K') => app.scrol_up(),
                KeyCode::Char('j') | KeyCode::Char('J') => app.scrol_down(),
                KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('l') | KeyCode::Char('L') => {
                    app.toggle_screen()
                }
                KeyCode::Char('w') | KeyCode::Char('W') => match app.current_screen {
                    CurrentScreen::Entropy => app.current_screen = CurrentScreen::Saving(SavingMode::Entropy),
                    CurrentScreen::ConditionalEntropy => app.current_screen = CurrentScreen::Saving(SavingMode::ConditionalEntropy),
                    _ => unreachable!(),
                }
                KeyCode::Char('r') | KeyCode::Char('R') => app.current_screen = CurrentScreen::Saving(SavingMode::Results),
                _ => {}
            }
        CurrentScreen::Saving(ref s) => {},
        CurrentScreen::Exiting(ref p) => match key_event.code {
            KeyCode::Char('n') | KeyCode::Char('N') => match p {
                PreviousScreen::Entropy => app.current_screen = CurrentScreen::Entropy,
                PreviousScreen::ConditionalEntropy => app.current_screen = CurrentScreen::ConditionalEntropy,
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => app.hard_quit(),
            _ => {}
        },
    }
}
