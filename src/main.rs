extern crate colored;
extern crate xcb;
mod ruwm;
mod config_parser;

use ruwm::ruwm::Ruwm;
use std::process;
use colored::Colorize;
use std::error::Error;

/*
 * For general todo/plan go to plan/plan.md
 * Main should do a few things - but nothing too specific
 * mostly it manages the abstract objects
 * Ruwm, IPC, Argparser, Setting up the configuration
 * Everything else happens in Ruwm or libraries
 * I don't like making main do any kind of heavy work
 * so the only thing that goes here is things that would
 * otherwise clutter up the 'real' code. Eye-Candy
 * top-level error handling, argparsing, starting auxilary services,
 * etc.
 *
 * explicitly exit with `1` error code o error
 *
 */

fn main() {
    /* argparser goes here
     * handles passing configuration to Ruwm Struct
     * Starts IPC
     * handles making things look pretty
     */

    let wm = match Ruwm::new() {
        Ok(x) => x,
        Err(e) => {
            println!("Error!: {}", e.description().red().bold());
            process::exit(1);
        }
    };

    // IPC

    run(wm);
    process::exit(0);
}

fn run(ruwm: Ruwm) {
    match ruwm.run() {
        Ok(_) => println!("{}", "Exit Success!".green()),
        Err(e) => {
            println!("{}", e.description().red().bold());
            process::exit(1);
        }
    }
}
