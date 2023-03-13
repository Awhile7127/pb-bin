// Clap used for CLI argument parsing
use clap::Parser;

// Request crate
use isahc::prelude::*;

// Standard Libraries
use std::error::Error;
use std::fs;

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

    // Parse arguments and declare URL
    let args = Args::parse();

    if args.method == "POST" {

        // Send a synchronous POST request with content read from a file
        let file_content = fs::read_to_string(args.file)?;

        let mut _resp = isahc::post(&args.base_url, file_content)?
            .text()?;

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
