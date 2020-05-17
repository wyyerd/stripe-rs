use serde_json::{json, Value as Json};
use std::collections::{BTreeMap, BTreeSet};

pub struct TypeKeyword {
    Enum,
    Struct,
}

pub struct TypeSourceKind {
    Schema,
    RequestParams,
}

pub struct TypeData {

}