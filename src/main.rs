use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cat&Mouse").build();

    let mut cube_position = Vector3::new(0.0, 0.0, 0.0);

    let mut yaw: f32 = 45.0_f32.to_radians();
    let mut pitch: f32 = 25.0_f32.to_radians();

    let camera_distance = 30.0;
    let mouse_sensitivity = 0.003;
    let speed = 5.0;
    let mut camera = Camera3D::perspective(
        Vector3::new(5.0, 5.0, 5.0),
        cube_position,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let model = rl
        .load_model(&thread, "assets/cat.gltf")
        .expect("Failed to laod model");

    let mut angle = 0.0_f32;

    rl.disable_cursor();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Read input and update state before beginning drawing.
        let delta = rl.get_frame_time();
        let mouse_delta = rl.get_mouse_delta();

        yaw += mouse_delta.x * mouse_sensitivity;
        pitch += mouse_delta.y * mouse_sensitivity;
        pitch = pitch.clamp(-1.4, 1.4);

        angle = -yaw.to_degrees();

        let forward = Vector3::new(-yaw.cos(), 0.0, -yaw.sin());
        let right = Vector3::new(-forward.z, 0.0, forward.x);

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

        camera.position = Vector3::new(
            cube_position.x + camera_distance * pitch.cos() * yaw.cos(),
            cube_position.y + camera_distance * pitch.sin(),
            cube_position.z + camera_distance * pitch.cos() * yaw.sin(),
        );

        camera.target = Vector3::new(
            cube_position.x - yaw.cos(),
            cube_position.y + 5.0,
            cube_position.z - yaw.sin(),
        );
        camera.up = Vector3::new(0.0, 1.0, 0.0);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(20, 1.0);

            /*d3d.draw_model_ex(
                &model,
                cube_position,
                Vector3::new(0.0, 1.0, 0.0),
                angle,
                Vector3::new(1.0, 1.0, 1.0),
                Color::RED,
            );*/
            d3d.draw_model_ex(
                &model,
                cube_position,
                Vector3::new(0.0, 1.0, 0.0),
                angle,
                Vector3::new(4.0, 4.0, 4.0),
                Color::WHITE,
            );
        }

        d.draw_text("Cat&Mouse Prototype", 12, 12, 20, Color::BLACK);
    }
}
