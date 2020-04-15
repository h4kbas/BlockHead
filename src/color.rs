use crate::block::BlockType;

pub type Color = (f32,f32,f32);

pub const BEACH: Color = (250.0 / 255.0 ,235.0 / 255.0 ,215.0 / 255.0);
pub const DIRT: Color = (216.0 / 255.0, 182.0 / 255.0, 151.0 / 255.0);
pub const ROCK: Color = (116.0 / 255.0, 114.0 / 255.0, 116.0 / 255.0);
pub fn get_blocktype_color(t: &BlockType)-> Color{
  match t {
    BlockType::DIRT => DIRT,
    BlockType::BEACH => BEACH,
    BlockType::ROCK => ROCK
  }
}