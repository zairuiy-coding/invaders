use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Drawable;

// represents projectiles in the game.
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool, // State of the shot, true if it is exploding
    timer: Timer, // Timer to manage shot movement and explosion duration
}

impl Shot {
    // Constructor for creating a new shot at a specific location
    pub fn new(x: usize, y:usize) -> Self {
        Self { 
            x, 
            y,
            exploding: false, // Initially, the shot is not exploding
            timer: Timer::from_millis(50), // Timer set for shot movement
        }
    }

    // Updates the shot's position or explosion status based on time elapsed
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta); // Update the timer with the time elapsed since last update
        if self.timer.ready && !self.exploding { // If the timer is ready and the shot isn't exploding
            if self.y > 0 {
                self.y -= 1; // Move the shot up by one row
            }
            self.timer.reset(); // Reset the timer for the next movement
        }
    }

    // Triggers the explosion of the shot
    pub fn explode(&mut self) {
        self.exploding = true; // Set the shot to exploding
        self.timer = Timer::from_millis(250); // Change the timer to manage explosion duration
    }

    // Determines if the shot is "dead" (i.e., if it has finished exploding or moved off-screen)
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}


impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        // Use '*' for exploding shots, '|' for moving shots
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}