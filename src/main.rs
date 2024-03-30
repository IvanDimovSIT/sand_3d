extern crate kiss3d;

mod model;
mod scene_generator;

use kiss3d::nalgebra::{Translation, UnitQuaternion, Vector3};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Sand 3D");
    //let mut c   = window.add_cube(1.0, 1.0, 1.0);
    //c.prepend_to_local_translation(&Translation::from(Vector3::new(0.0, 0.0, 3.0)));
    //c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);
    let mut face = window.add_quad(1.0, 1.0, 1, 1);
    face.prepend_to_local_translation(&Translation::from(Vector3::new(0.0f32, 0.0f32, 1.0f32)));
    face.set_color(1.0, 0.1, 0.0);

    //let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let t = Translation::from(Vector3::new(0.0f32, 0.0f32, 0.1f32));

    while window.render() {
        face.prepend_to_local_rotation(&rot);
        //i += 1;
        //c.prepend_to_local_rotation(&rot);
    }
}
