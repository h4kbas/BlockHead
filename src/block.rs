pub enum BlockType{
  DIRT = 1,
  BEACH,
  ROCK
}

pub struct Block{
  pub block_type: BlockType
}

impl Block{
  pub fn new(t: BlockType)-> Block{
    Block{
      block_type: t,
    }
  }
}