use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cat&Mouse").build();

    let mut cube_position = Vector3::new(0.0, 1.0, 0.0);

    // Camera rotation
    let mut yaw: f32 = 45.0_f32.to_radians();
    let mut pitch: f32 = 25.0_f32.to_radians();

    // Camera settings
    let camera_distance = 7.0;
    let mouse_sensitivity = 0.003;
    let speed = 5.0;

    let mut camera = Camera3D::perspective(
        Vector3::new(5.0, 5.0, 5.0),
        cube_position,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.disable_cursor();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();

        // Mouse rotation
        let mouse_delta = rl.get_mouse_delta();

        yaw += mouse_delta.x * mouse_sensitivity;
        pitch += mouse_delta.y * mouse_sensitivity;

        // Prevent the camera from flipping upside down
        pitch = pitch.clamp(-1.4, 1.4);

        let forward = Vector3::new(-yaw.cos(), 0.0, -yaw.sin());

        let right = Vector3::new(-forward.z, 0.0, forward.x);

        // Movement
        if rl.is_key_down(KeyboardKey::KEY_W) {
            cube_position.x += forward.x * speed * delta;
            cube_position.z += forward.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            cube_position.x -= forward.x * speed * delta;
            cube_position.z -= forward.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            cube_position.x -= right.x * speed * delta;
            cube_position.z -= right.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            cube_position.x += right.x * speed * delta;
            cube_position.z += right.z * speed * delta;
        }

        // Calculate camera position from yaw and pitch
        camera.position = Vector3::new(
            cube_position.x + camera_distance * pitch.cos() * yaw.cos(),
            cube_position.y + camera_distance * pitch.sin(),
            cube_position.z + camera_distance * pitch.cos() * yaw.sin(),
        );

        camera.target = cube_position;
        camera.up = Vector3::new(0.0, 1.0, 0.0);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(20, 1.0);

            d3d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
        }

        d.draw_text("Cat&Mouse Prototype", 12, 12, 20, Color::BLACK);
    }
}
