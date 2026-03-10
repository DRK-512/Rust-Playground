use std::env; // read input args
use std::error::Error;
use std::fs; // read file contents
use std::process; // exit code // dyn error

use minigrep::{search, search_case_insensitive};

struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
}

impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
                if args.len() != 3 {
                        // A call to panic! is more appropriate for a
                        // programming problem than a usage
                        // problem
                        return Err("Usage: <query> <file_path>");
                }
                let query = args[1].clone();
                let file_path = args[2].clone();
                let ignore_case = env::var("IGNORE_CASE").is_ok();
                Ok(Config { query, file_path, ignore_case })
        }
}

// Box<dyn Error> means the function will return a type that implements the
// Error trait, but we don’t have to specify what particular type the return
// value will be (NOTE: dyn = dynamic)
fn run(config: Config) -> Result<(), Box<dyn Error>> {
        // Read the file contents
        // ? will return the error value from the current function for the
        // caller to handle
        let contents = fs::read_to_string(config.file_path)?;

        let results = if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
        } else {
                search(&config.query, &contents)
        };

        for line in results {
                println!("{line}");
        }
        Ok(())
}

fn main() {
        // This allows us to take the input params, and stores them into a list
        // NOTE: Note that std::env::args will panic if any argument contains
        // invalid Unicode
        let args: Vec<String> = env::args().collect();

        let config = Config::build(&args).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                process::exit(1);
        });
        println!(
                "Searching for: {0}\n in {1}",
                config.query, config.file_path
        );
        // if let rather than unwrap_or_else to check whether run returns an Err
        // since run doesnt return a value that we want to unwrap
        if let Err(e) = run(config) {
                eprintln!("ERROR: {e}");
                process::exit(1);
        }
}
