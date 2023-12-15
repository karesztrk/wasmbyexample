use wasm_bindgen::prelude::*;

// Define the size of our "canvas"
const CANVAS_SIZE: usize = 11;

const OUTPUT_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }

    return pointer;
}

pub trait Element {
    fn contains(&self, x: u8, y: u8) -> bool;
}

struct Cell(u8, u8);

struct Shape {
    cells: Vec<Cell>,
}

impl Element for Shape {
    fn contains(&self, x: u8, y: u8) -> bool {
        let item = self
            .cells
            .iter()
            .find(|&item| item.0 == y as u8 && item.1 == x as u8);

        item.is_some()
    }
}

#[wasm_bindgen]
pub fn generate_canvas(
    dark_value_red: u8,
    dark_value_green: u8,
    dark_value_blue: u8,
    light_value_red: u8,
    light_value_green: u8,
    light_value_blue: u8,
) {
    let alien_shape: Shape = Shape {
        cells: vec![
            Cell(0, 2),
            Cell(0, 8),
            Cell(1, 3),
            Cell(1, 7),
            Cell(2, 2),
            Cell(2, 3),
            Cell(2, 4),
            Cell(2, 5),
            Cell(2, 6),
            Cell(2, 7),
            Cell(2, 8),
            Cell(3, 1),
            Cell(3, 2),
            Cell(3, 4),
            Cell(3, 5),
            Cell(3, 6),
            Cell(3, 8),
            Cell(3, 9),
            Cell(4, 0),
            Cell(4, 1),
            Cell(4, 2),
            Cell(4, 3),
            Cell(4, 4),
            Cell(4, 5),
            Cell(4, 6),
            Cell(4, 7),
            Cell(4, 8),
            Cell(4, 9),
            Cell(4, 10),
            Cell(5, 0),
            Cell(5, 2),
            Cell(5, 3),
            Cell(5, 4),
            Cell(5, 5),
            Cell(5, 6),
            Cell(5, 7),
            Cell(5, 8),
            Cell(5, 10),
            Cell(6, 0),
            Cell(6, 2),
            Cell(6, 8),
            Cell(6, 10),
            Cell(7, 3),
            Cell(7, 4),
            Cell(7, 6),
            Cell(7, 7),
        ],
    };

    // Since Linear memory is a 1 dimensional array, but we want a grid
    // we will be doing 2d to 1d mapping
    for y in 0..CANVAS_SIZE {
        for x in 0..CANVAS_SIZE {
            // Set our default case to be dark squares
            let mut is_dark_square: bool = true;

            let search_result = alien_shape.contains(x as u8, y as u8);

            if search_result {
                is_dark_square = !is_dark_square;
            }

            // Now that we determined if we are dark or light,
            // Let's set our square value
            let mut square_value_red: u8 = dark_value_red;
            let mut square_value_green: u8 = dark_value_green;
            let mut square_value_blue: u8 = dark_value_blue;
            if !is_dark_square {
                square_value_red = light_value_red;
                square_value_green = light_value_green;
                square_value_blue = light_value_blue;
            }

            // Let's calculate our index, using our 2d -> 1d mapping.
            // And then multiple by 4, for each pixel property (r,g,b,a).
            let square_number: usize = y * CANVAS_SIZE + x;
            let square_rgba_index: usize = square_number * 4;

            // Finally store the values.
            unsafe {
                OUTPUT_BUFFER[square_rgba_index + 0] = square_value_red; // Red
                OUTPUT_BUFFER[square_rgba_index + 1] = square_value_green; // Green
                OUTPUT_BUFFER[square_rgba_index + 2] = square_value_blue; // Blue
                OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
            }
        }
    }
}
