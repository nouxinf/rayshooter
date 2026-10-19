use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cat&Mouse").build();
    let camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 10.0), // Position
        Vector3::new(0.0, 0.0, 0.0),    // Target
        Vector3::new(0.0, 1.0, 0.0),    // Up vector
        45.0,                           // FovY
    );
    let mut cube_position = Vector3::new(0.0, 1.0, 0.0);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            cube_position.z -= 0.1;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            cube_position.z += 0.1;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            cube_position.x -= 0.1;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            cube_position.x += 0.1;
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        {
            // Enter 3D mode
            let mut d3d = d.begin_mode3D(camera);
            d3d.draw_grid(10, 1.0);
            d3d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            //d3d.draw_cube_wires(Vector3::ZERO, 2.0, 2.0, 2.0, Color::DARKGRAY);
        }
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
