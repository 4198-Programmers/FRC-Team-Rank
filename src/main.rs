use clap::{Parser};

/// Program to test the Meanscout API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[arg(short, long, default_value="csv")]
    output: String,

    /// The event code
    #[arg(short, long)]
    code: String,

//    #[command(subcommand)]
//     command: Option<Commands>,

}

// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Adds
//     Add(Add),
//     /// does a thing
//     Remove(Add),
// }

// #[derive(Args, Debug)]
// struct Add {
//     name: Option<String>,
// }

fn main() {
    
    // This is just some testing with commands
    // if args.command.is_some() {
    //     match &args.command.as_ref().unwrap() {
    //         Commands::Add(name) => {
    //             println!("'meantest add' was used, name is: {:?}", name.name.as_ref().unwrap())
    //         },
    //         Commands::Remove(name) => {
    //             println!("'meantest remove' was used, name is: {:?}", name.name.as_ref().unwrap())
    //         }
    //     }
    // }
}