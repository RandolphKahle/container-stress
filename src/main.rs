use structopt::StructOpt;
use rand::prelude::*;

// Stressor program - memory
//
// The purpose of the Stressor programs is to create various forms of stress on the computational
// environment.
//
// Why? To help systems deployers ensure that they have proper systemic controls in place to
// deal with aberrant programmatic behaviours.
//
// The program creates memory stress by allocating memory in various ways.
//
// How?
//   * Steadily increasing demands on memory.

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
    // No arguments at first - that will be a refinement
    // let args = Cli::from_args();
    //
    // println!("Command: {}", &args.command);

    let mut vec = Vec::new();

    let mut total_memory :i64 = 0;

    loop {
        let mut memory_block_reference : Box<[u8; MEMORY_BLOCK_SIZE]> = Box::new([0; MEMORY_BLOCK_SIZE]);
        total_memory = total_memory + MEMORY_BLOCK_SIZE as i64;

        for n in 0..MEMORY_BLOCK_SIZE {
            memory_block_reference[n] = random();
        }

        vec.push(memory_block_reference);
        println!("Total memory requested:: {}", total_memory);

    }
}
