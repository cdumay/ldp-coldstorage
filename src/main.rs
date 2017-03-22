#[macro_use]
extern crate clap;
extern crate leveldb;
extern crate glob;

use glob::glob;
use clap::App;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let dbpath = match matches.value_of("dbpath") {
        Some(userpath) => PathBuf::new().join(userpath),
        _ => std::env::temp_dir().join("ldp-coldstorage")
    };

    let srcpattern = matches.value_of("srcpattern").unwrap_or("*");
    let srcpath = match matches.value_of("srcpath") {
        Some(userpath) => PathBuf::new().join(userpath).join(srcpattern),
        _ => std::env::current_dir().unwrap().join(srcpattern)
    };
    println!("Using source pattern: {}", srcpath.display());

    let mut options = Options::new();
    options.create_if_missing = true;
    let db: Database<i32> = match Database::open(dbpath.as_path(), options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e)
    };
    println!("Using database path: {}", dbpath.display());

    for entry in glob(srcpath.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_dir() == false {
                    let mut f = File::open(path.to_str().unwrap()).unwrap();
                    let mut file = BufReader::new(&f);
                    for line in file.lines() {
                        let l = line.unwrap();
                        println!("{}", l);
                    };
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
