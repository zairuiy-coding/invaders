use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{frame::Drawable, NUM_COLS, NUM_ROWS};

// Represents an individual invader with coordinates in the game grid.
pub struct Invader {
    pub x: usize,
    pub y: usize,
}

// Represents an individual invader with coordinates in the game grid.
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    // Constructs a new Invaders instance with a grid of invaders positioned in a checkerboard pattern.
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // Place invaders in a checkerboard pattern within specified bounds
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0) {
                        army.push(Invader {x, y});
                    }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000), // Initialize timer with 2-second interval
            direction: 1, // Start moving right
        }
    }

    // Updates the position of the invaders based on timer and direction.
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
             // Determine if movement should reverse or go downwards
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1; // Change direction to right
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1; // Change direction to left
                    downwards = true;
                }
            }
            if downwards {
                // Accelerate movement downwards by reducing the timer duration
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;  // Move all invaders down one row
                }
            } else {
                // Move all invaders horizontally in the current direction
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

    // Returns true if all invaders have been killed.
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    // Checks if any invader has reached the bottom of the grid.
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    // Attempts to kill an invader at specified coordinates, returns true if successful.
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|invader| invader.x == x && invader.y == y) {
            self.army.remove(idx); // Remove the invader
            true
        } else {
            false
        }
    }
}

// Implementation of Drawable trait for Invaders, used for drawing the invaders on a frame.
impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if ((self.move_timer.time_left.as_secs_f32())
                / self.move_timer.duration.as_secs_f32()) > 0.5 {
                    "x" // Use 'x' for the first half of the timer
                } else {
                    "+" // Switch to '+' for the second half
                };
        }
    }
}