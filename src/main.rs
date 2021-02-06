use blockchainlib::*;

fn main () {
    let block = Block::new(0, 0, vec![0; 32], 1, "Firsty blocko hahaha".to_owned());

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);
}
