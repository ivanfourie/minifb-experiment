use std::cmp;

#[derive(Debug)]
pub struct Display {
    color_buffer: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl Display {
    pub fn new(window_width: u32, window_height: u32) -> Result<Self, String> {

        let buffer = vec![0; (window_width * window_height) as usize];

        Ok(Display {
            color_buffer: buffer,
            width: window_width,
            height: window_height,
        })
    }

    pub fn color_buffer(&mut self) -> &mut Vec<u32> {
        &mut self.color_buffer
    }

    pub fn clear_color_buffer(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.color_buffer[(self.width * y) as usize + x as usize] = color;
            }
        }
    }
}

pub trait Drawable {
    fn draw_pixel(&mut self, x: usize, y: usize, color: u32);
    fn draw_grid(&mut self, size: usize, color: u32);
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32);
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32);
}

impl Drawable for Display {
    fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width as usize && y < self.height as usize {
            self.color_buffer[(self.width as usize * y) + x] = color;
        }
    }

    fn draw_grid(&mut self, size: usize, color: u32) {
        // Draw a background grid that fills the entire window.
        // Lines should be rendedered at every row/col multiple of 10.
    
        for y in (0..self.width).step_by(size) {
            for x in (0..self.width).step_by(size) {
                self.draw_pixel(x as usize, y as usize, color)
            }
        }
    }

    // Use Digital Differential Analyzer line drawing algorythm to draw a line
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        // Calculate the difference in x and y between the start and end points
        let delta_x = x1 - x0;
        let delta_y = y1 - y0;
    
        // Find the length of the longest side of the line
        let longest_side_length = cmp::max(delta_x.abs(), delta_y.abs());
    
        // Calculate the increments in x and y for each step
        let x_inc = delta_x as f32 / longest_side_length as f32;
        let y_inc = delta_y as f32 / longest_side_length as f32;
    
        // Initialize the current x and y values
        let mut current_x = x0 as f32;
        let mut current_y = y0 as f32;
    
        // Iterate for the length of the longest side
        for _ in 0..=longest_side_length {
            // Draw a pixel at the current x and y position, rounded to the nearest integer
            self.draw_pixel(current_x.round() as usize, current_y.round() as usize, color);
    
            // Increment the current x and y values
            current_x += x_inc;
            current_y += y_inc;
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for i in 0..width {
            for j in 0..height {
                let current_x = x + i;
                let current_y = y + j;
                self.draw_pixel(current_x as usize, current_y as usize, color);
            }
        }
    }    
}
