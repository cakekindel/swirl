use std::sync::mpsc;

use kiss3d::window;
use kiss3d::light;
use nalgebra as na;

fn main() {
    let (sender, _) = mpsc::channel();
    let mut canvas = window::Canvas::open(
        "Test",
        false,
        200,
        200,
        None,
        sender
    );
    let mut cube = window.add_cube(1.0, 1.0, 1.0);

    cube.set_color(1.0, 0.0, 0.0);
    window.set_light(light::Light::StickToCamera);
    let rot = na::UnitQuaternion::from_axis_angle(&na::Vector3::y_axis(), 0.014);

    while window.render() {
        cube.prepend_to_local_rotation(&rot);
    }
}
