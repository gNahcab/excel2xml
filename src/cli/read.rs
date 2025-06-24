use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::operations::{excel2xml, write_hcl_default};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 1)]
    debug: u8,

    ///takes an argument
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// transform XML-File
    XML {
        #[arg(short, long, value_name = "TRANSFORM PATH")]
        transform: PathBuf,
    },
     HCL {
         #[arg(short, long, value_name = "TRANSFORM PATH")]
         folder: PathBuf,
     }
}
pub fn read_in() -> () {
    let cli = Cli::parse();
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    /*
    match cli.debug {
        0 => println!("Debug mode if off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode in on"),
        _ => println!("Don't be crazy"),
    }
     */

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::XML {transform}) => {
            println!("[Xml] transform: {:?}" , transform);
            excel2xml(transform);
        },
        Some(Commands::HCL { folder }) => {
            println!("[Hcl] write based on folder: {:?}", folder);
            write_hcl_default(folder);
        }
        None => println!("Command '{:?}' does not exist: Commands are 'xml'.", cli.command),
    }
}



