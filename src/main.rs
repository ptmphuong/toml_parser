/*
 * Pairs are used to:
 *  Determining which rule produced the Pair
 *  Using the Pair as a raw &str
 *  Inspecting the inner named sub-rules that produced the Pair
 *
*/

use toml_parser::parser;

fn main() {
    let this_cargo = "files/this_Cargo.toml";

    let parsed_cargo_toml = parser::parse_toml(this_cargo);
    println!("{:?}", parsed_cargo_toml);

    if let Ok(parsed_map) = parsed_cargo_toml {
        let package_vals = parsed_map.get("package");
        println!("package vals: {:?}", package_vals);
    }
}
