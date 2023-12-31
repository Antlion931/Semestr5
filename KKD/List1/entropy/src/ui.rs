use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use ratatui::{prelude::*, widgets::*};

use crate::app::{App, Screen, SavingMode};
use crate::tui::Frame;

const MY_COLOR: Color = Color::Rgb(0xB0, 0x00, 0xB0);

pub fn render(app: &mut App, f: &mut Frame) {
    match app.get_current_screen() {
        Screen::Entropy | Screen::ConditionalEntropy => render_entropy_or_conditional_entropy(app, f),
        Screen::Saving(ref s) => render_saving_info(app, f, *s),
        Screen::Exiting => render_exiting_info(app, f),
    }
}

fn render_exiting_info(app: &mut App, f: &mut Frame) {
    let popup_block = Block::default()
        .title("WARNIG")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(MY_COLOR))
        .style(Style::default());


    let mut things_to_save = Vec::with_capacity(3);

    if !app.entropy_saved {
        things_to_save.push("entropy counts");
    }

    if !app.conditional_entropy_saved {
        things_to_save.push("conditional entropy counts");
    }

    if !app.results_saved {
        things_to_save.push("results");
    }

    let exit_text = Text::styled(
        format!("You did not save {}\nare you sure you want to quit?\n(y/n)", things_to_save.join(", ")),
        Style::default().fg(Color::White),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}

fn render_saving_info(app: &mut App, f: &mut Frame, mode: SavingMode) {
    let popup_block = Block::default()
        .title("SAVING")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(MY_COLOR))
        .style(Style::default());


    let exit_text = Text::styled(
        format!("Your are curently saving {}\n\
            file will be saved in csv format with ; as separator\n\
            file name: {}\n\
            Esc - quit | Enter - save | Backspace - delete last character", match mode {
                SavingMode::Entropy => "entropy counts",
                SavingMode::ConditionalEntropy => "conditional entropy counts",
                SavingMode::Results => "results",
            } ,app.file_name),
        Style::default().fg(Color::White),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
}

fn render_entropy_or_conditional_entropy(app: &mut App, f: &mut Frame) {
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

    match app.get_current_screen() {
        Screen::Entropy => {
            entropy_tab = entropy_tab.block(chosen_block);
            conditional_entropy_tab = conditional_entropy_tab.block(notchosen_block);
        }
        Screen::ConditionalEntropy => {
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

    let difference_text = Paragraph::new(Text::styled(
        format!(
            "Difference = {}",
            app.difference().unwrap()
        ),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .block(Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Double)
        .style(Style::default()));

    f.render_widget(difference_text, chunks[1]);

    let help_text = Paragraph::new(
                "Press q to quit\n\
                Press w to save current window\n\
                Press r to save results\n\
                Press k/j to move up and down\n\
                Press h/l to change active window\n"
                ).block(Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Double)
        .style(Style::default())).alignment(Alignment::Center);

    f.render_widget(help_text, chunks[3]);

    match app.get_current_screen() {
        Screen::Entropy => {
            let list = make_non_zero_elements_list(app.get_single_byte_counts(), |key, value| format!("{:#04X} : {}", key, value));
            f.render_stateful_widget(list, chunks[2], &mut app.entropy_list_state);
        }

        Screen::ConditionalEntropy => {
            let list = make_non_zero_elements_list(app.get_double_byte_counts(), |key, value| format!("{:#06X} : {}", key, value));
            f.render_stateful_widget(list, chunks[2], &mut app.conditional_entropy_list_state);
        }
        _ => {}
    }
}

fn make_non_zero_elements_list(from: &[u64], format: impl Fn(usize, u64) -> String) -> List<'static> {
    let mut list_items = Vec::<ListItem>::new();

    for (key, value) in from.iter().enumerate() {
        if *value == 0 {
            continue;
        }
        list_items.push(ListItem::new(Line::from(Span::styled(
            format(key, *value),
            Style::default().fg(MY_COLOR),
        ))));
    }

    List::new(list_items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
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
