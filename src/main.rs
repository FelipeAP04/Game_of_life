//game_of_life/src/main.rs
// Declare modules
mod framebuffer;
mod game_of_life; // New module for Game of Life logic

use raylib::prelude::*;
use framebuffer::Framebuffer;
use game_of_life::{GameOfLifeGrid, patterns}; 

fn main() {
    let screen_width = 100;
    let screen_height = 100;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life (Rust Raylib)")
        .build();

    rl.set_target_fps(10);

    let mut framebuffer = Framebuffer::new(screen_width, screen_height);
    let dead_cell_color = Color::new(20, 20, 30, 255);
    framebuffer.set_background_color(dead_cell_color); 

    let live_cell_color = Color::YELLOW;

    let mut game_grid = GameOfLifeGrid::new(screen_width, screen_height, live_cell_color, dead_cell_color);

    game_grid.apply_pattern(&patterns::glider(100, 100));
    game_grid.apply_pattern(&patterns::small_exploder(screen_width / 2 - 25, screen_height / 2 - 20));
    game_grid.apply_pattern(&patterns::ten_cell_row(screen_width / 2 - 5, screen_height / 2));
    game_grid.apply_pattern(&patterns::r_pentomino(screen_width / 2, screen_height / 2 + 30));
    game_grid.apply_pattern(&patterns::blinker(screen_width / 2 + 5, screen_height / 2 - 10));

    // 1. Create an initial Image (can be blank, it will be updated)
    let mut display_image = Image::gen_image_color(screen_width, screen_height, Color::BLANK);

    // 2. Load the texture ONCE from this initial image.
    let mut game_texture = rl.load_texture_from_image(&thread, &display_image)
                             .expect("Could not load initial texture");

    // Main game loop: continues as long as the window is open
    while !rl.window_should_close() {
        // Update game state to the next generation
        game_grid.next_generation();

        // Clear the framebuffer for the new frame's drawing
        framebuffer.clear();

        // Render the current state of the Game of Life grid to the framebuffer
        for y in 0..screen_height {
            for x in 0..screen_width {
                if let Some(cell_state) = game_grid.get_cell(x, y) {
                    framebuffer.set_pixel(x, y, game_grid.get_color(cell_state));
                }
            }
        }

        // 3. Update the 'display_image' with the current framebuffer data.
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let index = (y * framebuffer.width + x) as usize;
                display_image.draw_pixel(x, y, framebuffer.pixels[index]);
            }
        }

        // --- CORRECTED LINE (Using .to_vec() on ImageColors) ---
        let image_data_colors: Vec<Color> = display_image.get_image_data().to_vec(); 
        let pixels_as_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                image_data_colors.as_ptr() as *const u8,
                image_data_colors.len() * std::mem::size_of::<Color>(),
            )
        };
        game_texture.update_texture(pixels_as_bytes);
        // --- END CORRECTED LINE ---
        
        // Begin drawing operations
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK); // Clear the Raylib window background

        // 5. Draw the updated texture.
        d.draw_texture(&game_texture, 0, 0, Color::WHITE);
        
    }

    framebuffer.render_to_file("final_game_state.png");
}