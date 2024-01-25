use blockchainlib::*; 

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 118318, "First Block!".to_owned(), 0x0000000ffffffffffffffffffffffffff);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}
