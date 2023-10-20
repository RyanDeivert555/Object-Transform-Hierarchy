mod tests;
mod transform_map;
use raylib::prelude::*;
use transform_map::TransformMap;

fn main() {
    set_trace_log(TraceLogLevel::LOG_NONE);
    let (mut rl, thread) = raylib::init()
        .title("Object Transform")
        .size(800, 600)
        .build();
    rl.set_target_fps(60);

    let mut transform_map = TransformMap::new();
    let player_base = transform_map.new_transform(true);
    let camera_node = transform_map.new_transform(true);
    let gun_node = transform_map.new_transform(true);

    transform_map.add_child_from_id(player_base, camera_node);
    transform_map.move_vertical(camera_node, 1.0);
    transform_map.add_child_from_id(camera_node, gun_node);
    transform_map.move_horizontal(gun_node, -0.5);
    transform_map.move_vertical(gun_node, -0.5);
    transform_map.move_vertical(gun_node, -0.5);

    let gun_model = rl.load_model(&thread, "assets/blasterD.glb").unwrap();
    // recursion here
    let camera_position = transform_map.world_position(camera_node);
    let camera_target =
        Vector3::new(0.0, 0.0, 1.0).transform_with(transform_map.world_matrix(camera_node));
    let mut camera = Camera::perspective(
        camera_position,
        camera_target,
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    while !rl.window_should_close() {
        // update camera
        transform_map.set_camera(camera_node, &mut camera);

        let dt = rl.get_frame_time();
        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            transform_map.rotate_vertical(player_base, -0.5);
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            transform_map.rotate_vertical(player_base, 0.5);
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            transform_map.move_depth(player_base, dt * 10.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            transform_map.move_depth(player_base, dt * -10.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            transform_map.move_horizontal(player_base, dt * 10.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            transform_map.move_horizontal(player_base, dt * -10.0);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        {
            let mut d2 = d.begin_mode3D(camera);
            d2.draw_grid(10, 10.0);
            d2.draw_cube(Vector3::new(0.0, 0.5, 10.0), 1.0, 1.0, 1.0, Color::RED);

            transform_map.push_matrix(gun_node);
            d2.draw_model(&gun_model, Vector3::new(0.0, 0.0, 1.5), 1.0, Color::WHITE);
            transform_map.pop_matrix();
        }
        d.draw_fps(0, 0);
    }
}
