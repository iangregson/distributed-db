extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("A string value")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove the given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => panic!("unimplemented"),
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument not given.");
            let value = matches
                .value_of("VALUE")
                .expect("VALUE argument not given.");
            let mut kvs = KvStore::open(current_dir()?)?;
            kvs.set(key.to_string(), value.to_string())?;
        }
        ("rm", Some(matches)) => panic!("unimplemented"),
        _ => unreachable!(),
    }
    Ok(())
}
