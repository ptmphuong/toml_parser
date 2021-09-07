use anyhow::{Context, Result};
use pest::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TomlParser;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
}

pub type ParsedMap = HashMap<String, HashMap<String, Vec<Value>>>;

pub fn parse_toml(filepath: &str) -> Result<ParsedMap> {
    let new_file = fs::read_to_string(filepath)
        .with_context(|| format!("failed to read file from {}", filepath))?;

    let file = TomlParser::parse(Rule::file, &new_file)
        .with_context(|| format!("failed to parse"))?
        .next()
        .unwrap();

    // let mut properties: ParsedMap = HashMap::new();
    let mut properties: HashMap<String, HashMap<String, Vec<Value>>> = HashMap::new();

    let mut cur_section_name = String::new();

    for line in file.into_inner() {
        match line.as_rule() {
            // iterator will always hit Section first, before proerty
            Rule::section => {
                // section = { "[" ~ name ~ "]" }
                let mut inner_rules = line.into_inner();
                cur_section_name = inner_rules.next().unwrap().as_str().to_string();
            }

            Rule::property => {
                // property = { name ~ "=" ~ value }
                let mut inner_rules = line.into_inner();
                let section = properties.entry(cur_section_name.clone()).or_default();

                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();
                let system_val = Value::String(value);

                let entry = section.entry(name.clone()).or_default();
                entry.push(system_val);
            }

            Rule::comment => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(properties)
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ok() {
        let filepath = "files/nginx.service.txt";
    }
}
*/
