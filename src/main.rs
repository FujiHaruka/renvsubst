extern crate regex;

use std::io::{self, Read};
use std::env;
// use std::fmt;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let var_re = Regex::new(r"\$([[:word:]]+)").unwrap();

    let mut output = input.clone();
    for caps in var_re.captures_iter(&input) {
        let var_name = &caps[1];
        let var_value = &value_from_env(var_name);
        output = replace_var(&output, (var_name, var_value));
    }
    println!("{}", output);
}

fn value_from_env(var_name: &str) -> String {
    match env::vars().find(|x| x.0 == var_name) {
        Some(key_value) => key_value.1,
        None => String::from(""),
    }
}

fn replace_var(target: &str, var: (&str, &str)) -> String {
    let var_name = var.0;
    let var_value = var.1;
    target.replace(&format!("${}", var_name), var_value)
          // `{` character is escaped with `{{`
          .replace(&format!("${{{}}}", var_name), var_value)
}
