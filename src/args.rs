use std::ffi::OsString;
use clap::{arg, Command};
use crate::commands::{decode, encode, remove};

fn cli() -> Command {
    Command::new("pngyinx")
        .about("The CLI to secure your secrets")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("encode")
                .about("To encode your secret")
                .arg(arg!(<PATH> "The path to your png"))
                .arg(arg!(<CHUNKTYPE> "The secret key"))
                .arg(arg!(<MESSAGE> "Your secret message"))
                .arg_required_else_help(true)
        ).subcommand(
            Command::new("decode")
                .about("To decode your secret")
                .arg(arg!(<PATH> "The path to your png"))
                .arg(arg!(<CHUNKTYPE> "The secret key"))
                .arg_required_else_help(true)
        ).subcommand(
        Command::new("remove")
            .about("To remove your secret")
            .arg(arg!(<PATH> "The path to your png"))
            .arg(arg!(<CHUNKTYPE> "The secret key"))
            .arg_required_else_help(true)
    )
}

pub fn args_processing() {
    let matches = cli().get_matches();
    
    match matches.subcommand() { 
        Some(("encode", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("required");
            let key = sub_matches.get_one::<String>("CHUNKTYPE").expect("required");
            let message = sub_matches.get_one::<String>("MESSAGE").expect("required");
            println!(
                "Encoding your secret from {} with key {}",
                path,
                key,
            );
            encode(path, key, message);
        },
        Some(("decode", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("required");
            let key = sub_matches.get_one::<String>("CHUNKTYPE").expect("required");
            println!(
                "Decoding your secret from {} with key {}",
                path,
                key,
            );
            decode(path, key);
        },
        Some(("remove", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("required");
            let key = sub_matches.get_one::<String>("CHUNKTYPE").expect("required");
            println!(
                "Removing secret message from {} with key {}",
                path,
                key,
            );
            remove(path, key);
        },
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("La commande {ext:?} n'est pas reconnue");
            println!("Vous avez essayé de lui donner les arguments suivants : {args:?}");
        }
        _ => unreachable!()
    }
}