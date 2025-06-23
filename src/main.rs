mod write_xml;
mod read_json;
mod errors;
mod parse_dm;
mod read_xlsx;
mod parse_xlsx;
mod parse_info;
mod read_hcl;
mod special_propnames;
mod cli;
mod operations;
mod expression_trait;
mod api;
mod write_hcl;
mod path_operations;

use std::error::Error;
use read_xlsx::extract::Extract;

fn main() {
    cli::read::read_in();
}

