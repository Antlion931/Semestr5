use crossterm::event::ModifierKeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use ratatui::{prelude::*, widgets::*};

use crate::app::{App, CurrentScreen};
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(6),
        ])
        .split(f.size());

    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let chosen_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let notchosen_block = Block::default()
        .style(Style::default())
        .padding(Padding::new(0, 0, 1, 0));

    let mut entropy_tab = Paragraph::new(Text::styled(
        format!("Entropy = {}", app.entropy().unwrap()),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    let mut conditional_entropy_tab = Paragraph::new(Text::styled(
        format!(
            "Conditional Entropy = {}",
            app.conditional_entropy().unwrap()
        ),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    match app.current_screen {
        CurrentScreen::Entropy => {
            entropy_tab = entropy_tab.block(chosen_block);
            conditional_entropy_tab = conditional_entropy_tab.block(notchosen_block);
        }
        CurrentScreen::ConditionalEntropy => {
            entropy_tab = entropy_tab.block(notchosen_block);
            conditional_entropy_tab = conditional_entropy_tab.block(chosen_block);
        }
        _ => {
            entropy_tab = entropy_tab.block(notchosen_block.clone());
            conditional_entropy_tab = conditional_entropy_tab.block(notchosen_block);
        }
    }

    f.render_widget(entropy_tab, tab_chunks[0]);
    f.render_widget(conditional_entropy_tab, tab_chunks[1]);

    let difference_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Double)
        .style(Style::default());

    let difference_text = Paragraph::new(Text::styled(
        format!(
            "Difference = {}",
            (app.entropy().unwrap() - app.conditional_entropy().unwrap()).abs()
        ),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .block(difference_block);

    f.render_widget(difference_text, chunks[1]);

    let help_block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Double)
        .style(Style::default());

    let help_text = Paragraph::new(
                "Press q to quit\n\
                Press w to save current window\n\
                Press r to save results\n\
                Press k/j to move up and down\n\
                Press h/l to change active window\n"
                ).block(help_block).alignment(Alignment::Center);

    f.render_widget(help_text, chunks[3]);

    match app.current_screen {
        CurrentScreen::Entropy => {
            let mut list_items = Vec::<ListItem>::new();

            for (key, value) in app.get_single_byte_counts().iter().enumerate() {
                if *value == 0 {
                    continue;
                }
                list_items.push(ListItem::new(Line::from(Span::styled(
                    format!("{:#04X} : {}", key, value),
                    Style::default().fg(Color::Rgb(0xB0, 0x00, 0xB5)),
                ))));
            }

            let list = List::new(list_items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[2], &mut app.entropy_list_state);
        }

        CurrentScreen::ConditionalEntropy => {
            let mut list_items = Vec::<ListItem>::new();

            for (key, value) in app.get_double_byte_counts().iter().enumerate() {
                if *value == 0 {
                    continue;
                }
                list_items.push(ListItem::new(Line::from(Span::styled(
                    format!("{:#06X} : {}", key, value),
                    Style::default().fg(Color::Rgb(0xB0, 0x00, 0xB5)),
                ))));
            }

            let list = List::new(list_items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[2], &mut app.conditional_entropy_list_state);
        }
        _ => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
