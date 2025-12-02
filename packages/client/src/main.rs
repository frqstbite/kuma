use std::{error::Error, io, result::Result};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode
    }
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout();
    
    // Set up board
    enable_raw_mode()?;
    execute!(
        output,
        EnterAlternateScreen
    )?;
    
    // Program loop
    println!("Press q to exit.");
    loop {
        let input = read()?;
        if let Event::Key(event) = input {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Cleanup and exit
    execute!(
        output,
        LeaveAlternateScreen
    )?;
    Ok(())
}