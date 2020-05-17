// use heck::{CamelCase, SnakeCase};
// use lazy_static::lazy_static;
// use regex::Regex;
use serde_json::{json, Value as Json};
// use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("openapi/spec3.json").unwrap();
    let spec: Json = serde_json::from_str(&raw).unwrap();
}
