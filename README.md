# toml_parser
A minimal .toml parser written in Rust. 

This is my personal project to study Rust, and explore Rust's crates ecosystem (especially parser/encoder/decoder tools).

-----

Implemented:
- Grammar Rules
- Parse function
- A hashmap that stores all parsed values in String format

Implementing:
- A syntax tree that can be traversed (this tree will replace the current hashmap that stores all values).  
- Value type that can be serialized and allows conversions to other format (JSON, YAML, etc.). 
