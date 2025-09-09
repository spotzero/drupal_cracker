use clap::{Parser};
use pwhash::bcrypt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::process::ExitCode;

mod phpass;

#[derive(PartialEq, Eq)]
enum Types {
    Drupal9,
    Drupal10,
    Unknown,
}

#[derive(PartialEq, Eq)]
enum Output {
  Normal,
  Verbose,
  Debug,
}

#[derive(Parser)]
struct Cli {
    /// The hash to be cracked.
    hash: String,

    /// Vebose mode.
    #[arg(short, long)]
    verbose: bool,
   
    /// Debug mode.
    #[arg(short, long)]
    debug: bool,

    /// Password list (defaults to passwords.txt).
    #[arg(short, long)]
    passwords: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let passwords = passwords(cli.passwords);
    /*
    let hashs = vec![
        "$2y$10$QczU42cYr1/bjaBJpY08DeV3lqM1MDBjV9obq7Pe75w3NWRf680/a",
        "$S$EEv3if8vYWhQVza25Hrs5yjOm9zPAcUxR//O6d1oxxqGIEcpAnBP",
    ];*/

    let hash = cli.hash;

    let hash_type = if &hash[0..4] == "$2y$" {
      Types::Drupal10
    }
    else if &hash[0..3] == "$S$" {
      Types::Drupal9
    }
    else {
      Types::Unknown
    };

    match hash_type {
      Types::Drupal10 => output_message("Detected <= D10 hash", Output::Verbose, cli.verbose, cli.debug),
      Types::Drupal9 => output_message("Detected >= D9 hash", Output::Verbose, cli.verbose, cli.debug),
      Types::Unknown => {
        output_message("Unknown hash type", Output::Normal, cli.verbose, cli.debug);
        return ExitCode::FAILURE;
        },
    }

    let mut count = 0;
    for password in &passwords {
        count = count + 1;
        if count % 100 == 0 {
            output_message(format!("Tested {} / {} passwords", count, passwords.len()).as_str(), Output::Verbose, cli.verbose, cli.debug);
        }
        output_message(format!("Testing password \"{}\"", password).as_str(), Output::Debug, cli.verbose, cli.debug);
        if verify(&password, hash.as_str(), &hash_type) {
            output_message(format!("The password is {}", password).as_str(), Output::Normal, cli.verbose, cli.debug);
            return ExitCode::SUCCESS;
        }
    }
    return ExitCode::FAILURE;
}

fn verify(password: &str, hash: &str, hash_type: &Types) -> bool {
  return match hash_type {
    Types::Drupal10 => bcrypt::verify(password, hash),
    Types::Drupal9 => phpass::check(password, hash),
    Types::Unknown => false,
  }
}

fn passwords(passwords: Option<String>) -> Vec<String> {
    let filename = passwords.unwrap_or("passwords.txt".to_string());
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.expect("Couldn't read line")).map(|a|{return a.trim().to_string()}).collect()
}

fn output_message(message: &str, out: Output, verbose: bool, debug: bool) {
  if out == Output::Normal
    || out == Output::Verbose && verbose
    || out == Output::Debug && verbose
    || out == Output::Debug && debug
    {
      println!("{}", message);
    }

}