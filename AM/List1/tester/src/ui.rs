use std::borrow;

use angular_units::Deg;
use prisma::{Hsv, Rgb};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph, Wrap},
};
use ratatui::{prelude::*, widgets::*};

use crate::app::{App, Screen};
use crate::tui::Frame;

const MY_COLOR: Color = Color::Rgb(0xB0, 0x00, 0xB0);

pub fn render(app: &mut App, f: &mut Frame) {
    match app.get_current_screen() {
        Screen::View => render_view(app, f),
        Screen::Results => render_results(app, f),
    }
}

fn render_tabs(app: &App, f: &mut Frame, rect: Rect) {
    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    let chosen_block = Block::default()
        .fg(Color::Indexed(236))
        .bg(Color::Indexed(232))
        .borders(Borders::ALL);

    let notchosen_block = Block::default()
        .fg(Color::Indexed(236))
        .bg(Color::Indexed(232))
        .padding(Padding::new(0, 0, 1, 0));

    let mut results_tab = Paragraph::new(Text::styled(
        "Results",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    let mut view_tab = Paragraph::new(Text::styled(
        "View",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    match app.get_current_screen() {
        Screen::Results => {
            results_tab = results_tab.block(chosen_block);
            view_tab = view_tab.block(notchosen_block);
        }
        Screen::View => {
            results_tab = results_tab.block(notchosen_block);
            view_tab = view_tab.block(chosen_block);
        }
    }

    f.render_widget(results_tab, tab_chunks[0]);
    f.render_widget(view_tab, tab_chunks[1]);
}

fn render_help(f: &mut Frame, rect: Rect, keys: &[(&str, &str)]) {
    let spans: Vec<_> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(
                format!(" {} ", key),
                Style::new().fg(Color::Black).bg(Color::DarkGray),
            );
            let desc = Span::styled(
                format!(" {} ", desc),
                Style::new().fg(Color::DarkGray).bg(Color::Black),
            );
            [key, desc]
        })
        .collect();
    let help = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .fg(Color::Indexed(236))
        .bg(Color::Indexed(232))
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(help, rect);
}

fn render_view(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.size());

    render_tabs(app, f, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(11), Constraint::Min(1)])
        .split(chunks[1]);
    f.render_stateful_widget(make_list(app), main_chunks[0], &mut app.view_list_state);

    match app.view_list_state.selected().unwrap() {
        0 => render_mst_in_rectangle(app, f, main_chunks[1]),
        1 => render_cycle_in_rectangle(app, f, main_chunks[1], &app.cycle),
        x => render_cycle_in_rectangle(app, f, main_chunks[1], &app.random_permutations[x - 2]),
    }

    let keys = [
        ("Q", "Quit"),
        ("H/L", "Change Tab"),
        ("J/K", "Change Showed"),
        ("W/A/S/D", "Move Map"),
        ("Z/X", "Zoom In/Out Map"),
        ("R", "Reset Map"),
        ("M", "Minimal Permutation"),
    ];

    render_help(f, chunks[2], &keys);
}

fn make_list(app: &App) -> List<'static> {
    let mut list_items = Vec::<ListItem>::new();

    list_items.push(ListItem::new(Line::from(Span::styled(
        "MST",
        Style::default().fg(Color::DarkGray),
    ))));
    list_items.push(ListItem::new(Line::from(Span::styled(
        "MST Cycle",
        Style::default().fg(Color::DarkGray),
    ))));
    for i in 0..1000 {
        if i == app.minimal {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("MIN {}", i),
                Style::default().fg(Color::DarkGray),
            ))));
        } else {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("P {}", i),
                Style::default().fg(Color::DarkGray),
            ))));
        }
    }

    List::new(list_items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(MY_COLOR))
        .highlight_symbol(">")
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .fg(Color::Indexed(236))
                .bg(Color::Indexed(232)),
        )
}

fn render_mst_in_rectangle(app: &App, f: &mut Frame, rect: Rect) {
    let map = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .fg(Color::Indexed(236))
                .bg(Color::Indexed(232)),
        )
        .paint(|ctx| {
            for x in 0..app.mst_matrix.len() {
                for y in x + 1..app.mst_matrix.len() {
                    if app.mst_matrix[x][y] {
                        ctx.draw(&canvas::Line {
                            x1: app.nodes[x][0] as f64,
                            y1: app.nodes[x][1] as f64,
                            x2: app.nodes[y][0] as f64,
                            y2: app.nodes[y][1] as f64,
                            color: MY_COLOR,
                        });
                    }
                }
            }

            ctx.layer();

            for x in app.nodes {
                ctx.print(
                    x[0] as f64,
                    x[1] as f64,
                    Span::styled("X", Style::default().fg(Color::DarkGray)),
                );
            }
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([app.min_x as f64, app.max_x as f64])
        .y_bounds([app.min_y as f64, app.max_y as f64]);
    f.render_widget(map, rect);
}

fn render_cycle_in_rectangle(app: &App, f: &mut Frame, rect: Rect, cycle: &[usize]) {
    let map = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .fg(Color::Indexed(236))
                .bg(Color::Indexed(232)),
        )
        .paint(|ctx| {
            for (n, x) in cycle.windows(2).enumerate() {
                let color: Rgb<_> =
                    Hsv::new(Deg(360.0 * (n as f64 / app.cycle.len() as f64)), 1.0, 1.0).into();
                let rgb = Color::Rgb(
                    (color.red() * u8::MAX as f64) as u8,
                    (color.green() * u8::MAX as f64) as u8,
                    (color.blue() * u8::MAX as f64) as u8,
                );

                ctx.draw(&canvas::Line {
                    x1: app.nodes[x[0]][0] as f64,
                    y1: app.nodes[x[0]][1] as f64,
                    x2: app.nodes[x[1]][0] as f64,
                    y2: app.nodes[x[1]][1] as f64,
                    color: rgb,
                });
            }

            ctx.layer();

            for x in app.nodes {
                ctx.print(
                    x[0] as f64,
                    x[1] as f64,
                    Span::styled("X", Style::default().fg(Color::DarkGray)),
                );
            }
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([app.min_x as f64, app.max_x as f64])
        .y_bounds([app.min_y as f64, app.max_y as f64]);
    f.render_widget(map, rect);
}

fn render_results(app: &mut App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.size());

    render_tabs(app, f, chunks[0]);

    let exercise_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let exercise_one = Paragraph::new(format!("MST weight = {}", app.mst_weight))
        .block(
            Block::default()
                .title("Exercise 1")
                .title_style(Style::default().fg(MY_COLOR))
                .borders(Borders::ALL)
                .fg(Color::Indexed(236))
                .bg(Color::Indexed(232))
                .border_type(BorderType::Rounded)
        ).fg(Color::DarkGray);
    f.render_widget(exercise_one, exercise_chunks[0]);

    let exercise_two = Paragraph::new(format!("MST cycle weight = {}", app.cycle_weight))
        .block(
            Block::default()
                .title("Exercise 2")
                .title_style(Style::default().fg(MY_COLOR))
                .borders(Borders::ALL)
                .fg(Color::Indexed(236))
                .bg(Color::Indexed(232))
                .border_type(BorderType::Rounded)
        ).fg(Color::DarkGray);
    f.render_widget(exercise_two, exercise_chunks[1]);

    let exercise_three = Paragraph::new(
            format!(
            "a) {}\nb) {}\nc) {}",
            app.avg_from_min_in_ten,
            app.avg_from_min_in_fifty,
            app.random_permutations_weight[app.minimal]))
    .block(
        Block::default()
            .title("Exercise 3")
            .title_style(Style::default().fg(MY_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
    ).fg(Color::DarkGray);
    f.render_widget(exercise_three, exercise_chunks[2]);

    let keys = [("Q", "Quit"), ("H/L", "Change Tab")];

    render_help(f, chunks[2], &keys);
}
