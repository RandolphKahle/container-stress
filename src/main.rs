use structopt::StructOpt;
use rand::prelude::*;

const MEMORY_BLOCK_SIZE: usize = 1048576;


#[derive(StructOpt)]
struct Cli {
    /// The command we are looking for
    command: String,
}

#[allow(dead_code)]
struct MemoryBlock {
    array: [i16; 16384]
}


fn main() {
    let args = Cli::from_args();

    println!("Command: {}", &args.command);
    let mut vec = Vec::new();

    loop {
        let mut memory_block_reference : Box<[u8; MEMORY_BLOCK_SIZE]> = Box::new([0; MEMORY_BLOCK_SIZE]);

        for n in 0..MEMORY_BLOCK_SIZE {
            memory_block_reference[n] = random();
        }

        vec.push(memory_block_reference);
    }
}
