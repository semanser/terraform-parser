# terraform-parser
Incredibly fast, strictly typed terraform files parser written in Rust. Based on [serde](https://github.com/serde-rs/serde).


# Usage

```rust
use std::fs;
use terraform_parser::TerraformParser;

fn main() {
  let state =
    fs::read_to_string("./state.json").expect("Something went wrong reading the state file");

  let plan =
    fs::read_to_string("./plan.json").expect("Something went wrong reading the plan file");

  let parsed_state = TerraformParser::parse_state(&state);
  let parsed_plan = TerraformParser::parse_plan(&plan);

  println!("{}", parsed_state.unwrap().terraform_version);
  println!("{}", parsed_plan.unwrap().format_version);
}
```
