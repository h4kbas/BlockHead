
use std::collections::HashMap;
use crate::{color::get_blocktype_color, block::Block};
use kiss3d::window::Window;
use nalgebra::Translation3;

pub struct Map{
  title: &'static str,
  pub world: HashMap<(i64, i64, i64), Block>
}


impl Map{
  pub fn new(t: &'static str)-> Map{
    Map{
      title: t,
      world: HashMap::new()
    }
  }
  pub fn draw(&self, window: &mut Window){
    for (p, b) in self.world.iter(){
      let mut c = window.add_cube(1.0, 1.0, 1.0);

      let color = get_blocktype_color(&b.block_type);
      c.set_color(color.0, color.1, color.2);

      c.append_translation(&Translation3::new(p.0 as f32, p.1 as f32, p.2 as f32));
    }
  }
}

