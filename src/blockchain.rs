use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify (&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail");
                return false;
            } else if i != 0 {
                // NOT GEN BLOCK
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase (verification fail)");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Prev block hash mismatch");
                }
            } else {
                // GEN BLOCK
                if block.prev_block_hash != vec![0; 32] {
                    println!("GEN block prev hash invalid");
                    return false;
                }
            }
        }

        return true;
    }
}
