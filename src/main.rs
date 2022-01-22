use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // unwrap_or_else will return the value in Ok(value) if success.
    // Otherwise, it returns the error on Err(error),
    // and passes that to the closure.
    //
    // Ok(value) -> value goes to config -> config = value
    // Err(error) -> error goes to |err| -> println!("... {}", error);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // Get better output on fail
        process::exit(1); // Exit the program with exit status 1 
    });

    // We could use unwrap_or_else here as well,
    // but in this case we do not care about the
    // Ok(()) return as the value inside it is ().
    // A simple if let handles the error and exits accordingly.
    //
    // In other words: if there's an error while
    // running the config, get that error and use it
    // to print the message and exit the program.
    // Otherwise, don't assign anything.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
