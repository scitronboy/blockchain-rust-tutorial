use blockchainlib::*;

fn main () {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(0, 0, vec![0; 32], 1, "First block".to_owned(), difficulty);
    block.mine();
    println!("Mined first block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block]
    };

    println!("Verify Genesis: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 1, "A beautiful block".to_owned(), difficulty);

        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }
}
