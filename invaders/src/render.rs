use std::io::{Stdout, Write};
use crossterm::{cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};

use crate::frame::Frame;


/**
 * Renders the game to the terminal. 
 * This function updates the display based on the difference
 * between the current frame and the last frame, or forces a full re-render if required.
 */
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // Check if a forced render is requested or necessary (e.g., for full screen refresh).
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    // Iterate through each character in the current frame
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // Check if the character at the current position has changed from the last frame or if a forced render is active.
            if *s != last_frame[x][y] || force {
                // Move the cursor to the correct position on the terminal.
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                // Print the current character to the terminal at the moved position.
                print!("{}", *s); // print out the charater
            }
        }
    }

    // Ensure all queued commands and outputs are applied to the terminal.
    stdout.flush().unwrap();
}