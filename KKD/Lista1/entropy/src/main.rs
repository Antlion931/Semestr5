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
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::fs;
use std::process;
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong amount of operands, usage: entropy <file_path>");
        process::exit(1)
    }

    let file_path = args.get(1).expect("We know there are args is len 2");

    let contents = fs::read(file_path).expect("Should have been able to read the file");

    // Create an application.
    let mut app = App::new();

    for b in contents {
        app.read_byte(b).unwrap();
    }

    println!("entropy = {}", app.entropy().unwrap());
    println!(
        "conditional entropy = {}",
        app.conditional_entropy().unwrap()
    );

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

fn run(tui: &mut Tui, app: &mut App) -> Result<()> {
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => update(app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    Ok(())
}
