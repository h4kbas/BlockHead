extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::camera::FirstPerson;
use kiss3d::{event::Key, window::Window};
use na::Point3;
use map::{noisegen::gen_map_with_noise, map::Map};

mod color;
mod block;
mod map;

fn prepare_camera() -> FirstPerson {
    let eye = Point3::new(10.0f32, 10.0, 1.0);
    let at = Point3::origin();
    let mut first_person = FirstPerson::new(eye, at);
    first_person.rebind_up_key(Some(Key::W));
    first_person.rebind_down_key(Some(Key::S));
    first_person.rebind_left_key(Some(Key::A));
    first_person.rebind_right_key(Some(Key::D));
    first_person
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut camera = prepare_camera();
    window.set_light(Light::StickToCamera);

    let mut new_map: Map = Map::new("Hello World");
    gen_map_with_noise(&mut new_map);
    new_map.draw(&mut window);


    while window.render_with_camera(&mut camera){
        // c.prepend_to_local_rotation(&rot);
    }
}