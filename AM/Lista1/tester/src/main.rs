// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use rand::seq::SliceRandom;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::path::Path;
use std::process;
use tsp_tester::*;
use tspf::TspBuilder;
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong amount of operands, usage: tsp_tester <file_path>");
        process::exit(1)
    }

    let file_path = args.get(1).expect("We know there are args is len 2");

    match TspBuilder::parse_path(Path::new(file_path)) {
        Ok(tsp) => {
            let nodes: Vec<[i64; 2]> = tsp
                .node_coords()
                .values()
                .map(|point| [point.pos()[0] as i64, point.pos()[1] as i64])
                .collect();
            // Create an application.
            let mut app = App::new(&nodes);

            // Initialize the terminal user interface.
            let backend = CrosstermBackend::new(std::io::stderr());
            let terminal = Terminal::new(backend)?;
            let events = EventHandler::new(250);
            let mut tui = Tui::new(terminal, events);

            let result = run(&mut tui, &mut app);
            // Exit the user interface.
            tui.exit()?;
            result?;
            Ok(())
        }
        Err(e) => {
            println!("Problem with file");
            process::exit(1);
        }
    }
}

fn run(tui: &mut Tui, app: &mut App) -> Result<()> {
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(app)?;
        // Handle events.
        match tui.events.next() {
            Some(Event::Tick) => {}
            Some(Event::Key(key_event)) => update(app, key_event),
            Some(Event::Mouse(_)) => {}
            Some(Event::Resize(_, _)) => {}
            None => {}
        };
    }

    Ok(())
}
