use std::ffi::OsString;
use clap::{arg, Command};

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
            println!(
                "Encoding your secret from {} with key {}",
                sub_matches.get_one::<String>("PATH").expect("required"),
                sub_matches.get_one::<String>("CHUNKTYPE").expect("required")
            );
        },
        Some(("decode", sub_matches)) => {
            println!(
                "Decoding your secret from {} with key {}",
                sub_matches.get_one::<String>("PATH").expect("required"),
                sub_matches.get_one::<String>("CHUNKTYPE").expect("required")
            );
        },
        Some(("remove", sub_matches)) => {
            println!(
                "Removing secret message from {} with key {}",
                sub_matches.get_one::<String>("PATH").expect("required"),
                sub_matches.get_one::<String>("CHUNKTYPE").expect("required")
            )
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