// Clap used for CLI argument parsing
use clap::Parser;

// Request crate
use isahc::prelude::*;

// Standard Libraries
use std::error::Error;
use std::fs;
use std::io::{self, Read};

// Struct used to declare command-line arguments
#[derive(Parser, Debug)]
struct Args {

    #[clap(long="base-url", short='b', default_value="", required=true)]
    base_url: String,

    #[clap(long="method", short='m', default_value="GET", required=false)]
    method: String,

    #[clap(long="file", short='f', default_value="", required=false)]
    file: String,

    #[clap(long="get-url", short='u', default_value="", required=false)]
    fetch_url: String
}

// Function expects Ok() or Err()
fn main() -> Result<(), Box<dyn Error>> {

    // Fetch input from stdin, used for piped input
    let mut piped = io::stdin();

    // Parse arguments and declare URL
    let args = Args::parse();

    if args.method == "POST" {

        // Create a new String buffer for later use
        // This stores the content of the paste
        let mut content = String::new();

        // If a file isn't provided...
        if args.file == "" {

            // Read the content piped to the program to the buffer declared earlier
            piped.read_to_string(&mut content)?;
        } else {

            // If a file is provided, read its content instead
            content = fs::read_to_string(args.file)?;
        }

        // Send a synchronous POST request to the URL with the desired content
        let mut _resp = isahc::post(&args.base_url, content)?
            .text()?;

        // Print the response from the server (AKA the URL where the paste is)
        println!("{}{}", &args.base_url, _resp);

    } else if args.method == "GET" {

        // Replace a substring so raw text is fetched
        let fetch_url = args.fetch_url.replace("p/", "");

        let mut _resp = isahc::get(fetch_url)?
            .text()?;

        println!("{}", _resp);
    }

    Ok(())
}
