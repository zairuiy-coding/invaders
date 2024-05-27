use std::time::Duration;

use crate::{frame::Drawable, invaders::Invaders, shot::Shot, NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,   // Horizontal position of the player on the grid
    y: usize,   // Vertical position of the player on the grid
    shots: Vec<Shot>,   // Container for shots fired by the player
}

// Defines the Player structure with position and shots fired.
impl Player {
    // Constructs a new Player instance, positioning them at the center-bottom of the game grid.
    pub fn new() -> Self {
        Self { 
            x: NUM_COLS / 2,    // Start in the middle of the grid horizontally
            y: NUM_ROWS - 1,    // Start at the bottom of the grid vertically
            shots: Vec::new(),  // Initialize with no shots fired
        }
    }

     // Moves the player one unit to the left, if not already at the left edge.
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // Moves the player one unit to the right, if not already at the right edge
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    // Allows the player to shoot; limits the number of concurrent shots to 2.
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y-1));
            true
        } else {
            false
        }
    }

    // Updates the state of the player and their shots. Removes any shots that are "dead."
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta); // Update each shot based on the elapsed time
        } 
        self.shots.retain(|shot| !shot.dead()); // Remove shots that are no longer active
    }

    // Checks for and handles collisions between the player's shots and the invaders.
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;  // Tracks if any invaders have been hit
        for shot in self.shots.iter_mut() {
            if !shot.exploding {    // Only consider non-exploding shots for collision detection
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode(); // Trigger the shot to explode upon hitting an invader
                }
            }
        }
        hit_something   // Return whether any hits occurred
    }
}

impl Drawable for Player {
    // Draws the player and their shots on the game frame.
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A";    // Draw the player at their current position
        for shot in self.shots.iter() {
            shot.draw(frame);   // Draw each of the player's shots
        }
    }
}