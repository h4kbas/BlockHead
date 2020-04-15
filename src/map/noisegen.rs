use super::map::Map;
use crate::block::{BlockType, Block};
use noise::{NoiseFn, Perlin};

fn get_noise(curr_x: i64, curr_z: i64, max_x: i64, max_y: i64, max_z: i64) -> i64{
  let noise = Perlin::new();

  let nx = curr_x as f64 / max_x as f64;
  let nz = curr_z as f64 / max_z as f64;
  let mut curr_y = (noise.get([nx, nz, max_y as f64]) * 10.0).round() as i64;
  
  if curr_y > max_y{
    curr_y = max_y;
  }
  else if curr_y < -max_y {
    curr_y = -max_y;
  }
  curr_y
}

fn nature_block(curr_y: i64) -> BlockType{
  if curr_y < -3{
    BlockType::ROCK
  }
  else if curr_y < 0 {
    BlockType::BEACH    
  }
  else if curr_y < 8 {
    BlockType::DIRT
  }
  else if curr_y < 15 {
    BlockType::ROCK
  }
  else {
    BlockType::DIRT
  }
}

pub fn gen_map_with_noise(map: &mut Map){
  const MAX_X: i64 = 50;
  const MAX_Z: i64 = 50;
  const MAX_Y: i64 = 20;
  
  for _z in 0..MAX_Z {
    for _x in 0..MAX_X {
      let y = get_noise(_x, _z, MAX_X, MAX_Y, MAX_Z);
        let curr_point = (_x, y, _z);
        let curr_block = Block::new(nature_block(y));
        map.world.insert(curr_point, curr_block);
    }
  }
}