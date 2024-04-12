#![allow(unused, unused_imports)]

use cli_project::{CliParams, run};
use std::{env, process};
// gives access to environment values/variables 

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let config_var = CliParams::build(&args);
    let mut config_var = match config_var {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
    if let Err(e) =  run(config_var) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}



