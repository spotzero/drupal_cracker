use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use pwhash::bcrypt;
use std::process::ExitCode;
use std::env;

mod phpass;

#[derive(PartialEq, Eq)]
enum Types {
    Drupal9,
    Drupal10,
    Unknown,
}
fn main() -> ExitCode {
    let passwords = passwords();
    /*
    let hashs = vec![
        "$2y$10$QczU42cYr1/bjaBJpY08DeV3lqM1MDBjV9obq7Pe75w3NWRf680/a",
        "$S$EEv3if8vYWhQVza25Hrs5yjOm9zPAcUxR//O6d1oxxqGIEcpAnBP",
    ];*/
    let hash = env::args().nth(1).unwrap();
    let hash_str = hash.as_str();

    let hash_type = if &hash_str[0..4] == "$2y$" {
      Types::Drupal10
    }
    else if &hash[0..3] == "$S$" {
      Types::Drupal9
    }
    else {
      Types::Unknown
    };

    match hash_type {
      Types::Drupal10 => println!("Detected D10 hash"),
      Types::Drupal9 => println!("Detected D9 hash"),
      Types::Unknown => {
        println!("Unknown hash type");
        return ExitCode::FAILURE;
        },
    }

    let mut count = 0;
    for password in &passwords {
        count = count + 1;
        if count % 100 == 0 {
            println!("{} / {}", count, passwords.len());
        }
        if verify(&password, hash_str, &hash_type) {
            println!("The password is {}", password);
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

fn passwords() -> Vec<String> {
    let f = File::open("passwords.txt").unwrap();
    let f = BufReader::new(f);
    f.lines().map(|l| l.expect("Couldn't read line")).map(|a|{return a.trim().to_string()}).collect()
}