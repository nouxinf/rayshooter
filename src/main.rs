use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("Cat&Mouse").build();

    let mut cat_position = Vector3::new(0.0, 0.0, 0.0);

    let mut yaw: f32 = 45.0_f32.to_radians();
    let mut pitch: f32 = 25.0_f32.to_radians();

    let camera_distance = 30.0;
    let mouse_sensitivity = 0.003;
    let speed = 5.0;
    let mut camera = Camera3D::perspective(
        Vector3::new(5.0, 5.0, 5.0),
        cat_position,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
	// CAT MODEL
    let mut model = rl
        .load_model(&thread, "assets/cat.glb")
        .expect("Failed to load cat");

    let palette = rl
        .load_texture(&thread, "assets/colormap-animal.png")
        .expect("Failed to load colormap-animal");

    for material in model.materials_mut() {
        material.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, &palette);
    }

	// BLASTER MODEL
    let mut blaster_model: Model = rl
        .load_model(&thread, "assets/blaster-p.glb")
        .expect("Failed to load blaster");
	let blaster_palette = rl
        .load_texture(&thread, "assets/colormap-blaster.png")
        .expect("Failed to load colormap-blaster");

    for material in blaster_model.materials_mut() {
        material.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, &blaster_palette);
    }

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
            cat_position.x += forward.x * speed * delta;
            cat_position.z += forward.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            cat_position.x -= forward.x * speed * delta;
            cat_position.z -= forward.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            cat_position.x -= right.x * speed * delta;
            cat_position.z -= right.z * speed * delta;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            cat_position.x += right.x * speed * delta;
            cat_position.z += right.z * speed * delta;
        }

        camera.position = Vector3::new(
            cat_position.x + camera_distance * pitch.cos() * yaw.cos(),
            cat_position.y + camera_distance * pitch.sin(),
            cat_position.z + camera_distance * pitch.cos() * yaw.sin(),
        );

        camera.target = Vector3::new(
            cat_position.x - yaw.cos(),
            cat_position.y + 5.0,
            cat_position.z - yaw.sin(),
        );
        camera.up = Vector3::new(0.0, 1.0, 0.0);

        let forward = Vector3::new(-yaw.cos(), 0.0, -yaw.sin());
        let right = Vector3::new(-forward.z, 0.0, forward.x);

        let blaster_position = Vector3::new(
            cat_position.x + right.x * 1.0 + forward.x * 0.5,
            cat_position.y + 3.0,
            cat_position.z + right.z * 1.0 + forward.z * 0.5,
        );
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(20, 1.0);
            d3d.draw_model_ex(
                &model,
                cat_position,
                Vector3::new(0.0, 1.0, 0.0),
                angle - 90.0,
                Vector3::new(2.0, 2.0, 2.0),
                Color::WHITE,
            );
            d3d.draw_model_ex(
                &blaster_model,
                blaster_position,
                Vector3::new(0.0, 1.0, 0.0),
                angle + 90.0,
                Vector3::new(5.0, 5.0, 5.0),
                Color::WHITE,
            );
        }

        d.draw_text("Cat&Mouse Prototype", 12, 12, 20, Color::BLACK);
    }
}
