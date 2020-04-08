extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
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
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument not given.");
            let value = matches
                .value_of("VALUE")
                .expect("VALUE argument not given.");
            let mut kvs = KvStore::open(current_dir()?)?;
            kvs.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument not given.");

            let mut kvs = KvStore::open(current_dir()?)?;
            if let Some(value) = kvs.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument not given.");

            let mut kvs = KvStore::open(current_dir()?)?;

            match kvs.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
