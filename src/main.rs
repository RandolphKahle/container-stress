//! # Container Stress
//!
//! The purpose of this utility is to create stress on the computational environment.
//!
//! This can be used to ensure that environments have propert limits set and recovery mechanisms
//! implemented.
//!
//! There are several potential modes of stress. The first implementation will support memory
//! allocation stress.


////////////////////////////////////////////////////////////////////////////////
//use structopt::StructOpt;
//use rand::prelude::*;

use log::{info, warn};
use std::{thread, time};

// For Signal Handling

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
//use std::thread;
//use std::time;
// This is just a collection of ints that represent kill signals.
// More specifically, they are the common kill signals used to
// terminate a program
// You can do println!("{:?}", TERM_SIGNALS) to see them
// They are just SIGINT(2), SIGTERM(15) and SIGQUIT(3)
use signal_hook::consts::TERM_SIGNALS;

// Module that sets boolean flags when kill signal is received
use signal_hook::flag;

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;




// const MEMORY_BLOCK_SIZE: usize = 1048576;


// #[derive(StructOpt)]
// struct Cli {
//     /// The command we are looking for
//     command: String,
// }
//
// #[allow(dead_code)]
// struct MemoryBlock {
//     array: [i16; 16384]
// }

// Worry about termination signals
// https://cloud.google.com/blog/products/containers-kubernetes/kubernetes-best-practices-terminating-with-grace
// https://dev.to/talzvon/handling-unix-kill-signals-in-rust-55g6
//


fn main() -> Result<(), Error> {
    // A special boolean that can be used across threads
    // The first time a kill signal is received, it will be set to
    // true by flag::register
    // The second time a kill signal is received, our process will
    // be killed by flag::register_conditional_shutdown
    let term_now = Arc::new(AtomicBool::new(false));

    for sig in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    // Our actual work thread
    let t = thread::spawn(move || {
        while !term_now.load(Ordering::Relaxed)
        {
            println!("Doing work...");
            thread::sleep(time::Duration::from_secs(1));
        }

        println!("\nThread exiting...");
    });

    // Create iterator over signals
    let mut signals = Signals::new(TERM_SIGNALS)?;

    // This loop runs forever, and blocks until a kill signal is received
    'outer: loop {
        for signal in signals.pending() {
            match signal {
                SIGINT => {
                    println!("\nGot SIGINT");
                    break 'outer;
                },
                SIGTERM => {
                    println!("\nGot SIGTERM");
                    break 'outer;
                },
                term_sig => {
                    println!("\nGot {:?}", term_sig);
                    break 'outer;
                },
            }
        }
    }

    // Wait for thread to exit
    t.join().unwrap();

    // Cleanup code goes here
    println!("\nReceived kill signal. Wait 10 seconds, or hit Ctrl+C again to exit immediately.");
    thread::sleep(time::Duration::from_secs(10));
    println!("Exited cleanly");

    Ok(())


// fn main() {
//     info!("Starting program.");
//
//     env_logger::init();
//
//     loop {
//         warn!("Nothing has been implemented...");
//
//         let ten_millis = time::Duration::from_millis(10000);
//         let now = time::Instant::now();
//
//         thread::sleep(ten_millis);
//
//     }
//
//     info!("Stopping program");


    // How do we send information to a log such that the operating environment can capture
    // and report - such as K8s and GKE?
    //




    // // No arguments at first - that will be a refinement
    // // let args = Cli::from_args();
    // //
    // // println!("Command: {}", &args.command);
    //
    // let mut vec = Vec::new();
    //
    // let mut total_memory :i64 = 0;
    //
    // loop {
    //     let mut memory_block_reference : Box<[u8; MEMORY_BLOCK_SIZE]> = Box::new([0; MEMORY_BLOCK_SIZE]);
    //     total_memory = total_memory + MEMORY_BLOCK_SIZE as i64;
    //
    //     for n in 0..MEMORY_BLOCK_SIZE {
    //         memory_block_reference[n] = random();
    //     }
    //
    //     vec.push(memory_block_reference);
    //     println!("Total memory requested:: {}", total_memory);
    //
    // }
}
