use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(0, now(), vec![0; 32],
        0, "Genesis block".to_owned(), difficulty);
    
    println!("Principia");

    block.mine();
    println!("{:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash,
        0, "Another block".to_owned(), difficulty);
    
        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
    }
        // tests for invalid blocks
        
        /*
        blockchain.blocks[3].hash[8] += 1; 
        println!("Verify: {}", &blockchain.verify());

        blockchain.blocks[3].index = 4;
        println!("Verify: {}", &blockchain.verify());
        
        blockchain.blocks[3].payload = "Nope".to_owned();
        println!("Verify: {}", &blockchain.verify());
        
        blockchain.blocks[3].prev_block_hash[18] = 8;
        println!("Verify: {}", &blockchain.verify());
        */
}