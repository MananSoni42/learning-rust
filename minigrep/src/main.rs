use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(
                    |err| {
                        eprintln!("Error while parsing arguements: {} \n\n\
                                    Usage: minigrep [filename] [query word] \n\
                                    Environment vars: \n\
                                    - CASE_INSENSITIVE: performs case insensitive search if set \n\
                                    - EXACT: Only matches exact queries if set",
                                    err);
                        process::exit(1);
                });

    if let Err(e) = minigrep::run(&cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
