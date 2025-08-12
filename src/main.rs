mod write_xml;
mod read_json;
mod errors;
mod parse_dm;
mod read_xlsx;
mod parse_xlsx;
mod parse_hcl;
mod read_hcl;
mod cli;
mod operations;
mod expression_trait;
mod api;
mod create_hcl;
mod path_operations;
mod write_csv;
mod read_csv;

use std::error::Error;
use read_xlsx::extract::Extract;

fn main() {
    cli::read::read_in();
}

