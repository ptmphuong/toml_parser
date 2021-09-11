/*
 * Pairs are used to:
 *  Determining which rule produced the Pair
 *  Using the Pair as a raw &str
 *  Inspecting the inner named sub-rules that produced the Pair
 *
*/

use anyhow::{Context, Result};
use std::fs;
use toml_parser::parser;

fn main() {
    let this_cargo = "files/this_Cargo.toml";

    let toml_file = fs::read_to_string(this_cargo).expect("cannot read file");

    let parsed_cargo_toml = parser::parse_toml(&toml_file);
    println!("{:?}", parsed_cargo_toml);

    if let Ok(parsed_map) = parsed_cargo_toml {
        let package_vals = parsed_map.get("package");
        println!("package vals: {:?}", package_vals);
    }
}
