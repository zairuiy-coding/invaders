use crate::{NUM_COLS, NUM_ROWS};

// Define a type alias for the Frame. Frame is a 2D vector that holds static string slices.
pub type Frame = Vec<Vec<&'static str>>;

/**
 * Function to create a new frame with the specified number of columns and rows.
 * Each cell in this frame is initialized with a space " ", representing an empty state.
 */
pub fn new_frame() -> Frame {
    // Initialize a vector with capacity equal to the number of columns
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        // For each column, initialize a vector with capacity equal to the number of rows
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            // Fill each row cell in the column with a space " " to denote an empty cell.
            col.push(" ");
        }
        // Add the filled column to the cols vector.
        cols.push(col);
    }
    // Return the fully initialized frame.
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}