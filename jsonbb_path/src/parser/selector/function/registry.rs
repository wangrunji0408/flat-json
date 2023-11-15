use std::collections::HashMap;

use crate::core::spec::functions::{Function, LogicalType, NodesType, ValueType};
use jsonbb::ValueRef;
use once_cell::sync::Lazy;
use regex::Regex;

/// The main registry of functions for use in JSONPath queries
///
/// These come directly from the JSONPath specification, which includes a registry of standardized
/// functions.
pub(crate) static REGISTRY: Lazy<HashMap<&'static str, &'static Function>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("length", &LENGTH_FUNC);
    m.insert("count", &COUNT_FUNC);
    m.insert("match", &MATCH_FUNC);
    m.insert("search", &SEARCH_FUNC);
    m.insert("value", &VALUE_FUNC);
    m
});

fn value_length(value: ValueRef<'_>) -> Option<usize> {
    match value {
        ValueRef::String(s) => Some(s.chars().count()),
        ValueRef::Array(a) => Some(a.len()),
        ValueRef::Object(o) => Some(o.len()),
        _ => None,
    }
}

#[jsonbb_path_macros::register(target = LENGTH_FUNC)]
fn length(value: ValueType) -> ValueType {
    match value {
        ValueType::Value(v) => value_length(v.as_ref()),
        ValueType::Node(v) => value_length(v),
        ValueType::Nothing => None,
    }
    .map_or(ValueType::Nothing, |l| ValueType::Value(l.into()))
}

#[jsonbb_path_macros::register(target = COUNT_FUNC)]
fn count(nodes: NodesType) -> ValueType {
    nodes.len().into()
}

#[jsonbb_path_macros::register(name = "match", target = MATCH_FUNC)]
fn match_func(value: ValueType, rgx: ValueType) -> LogicalType {
    match (value.as_value(), rgx.as_value()) {
        (Some(ValueRef::String(s)), Some(ValueRef::String(r))) => {
            Regex::new(format!("^({r})$").as_str())
                .map(|r| r.is_match(s))
                .map(Into::into)
                .unwrap_or_default()
        }
        _ => LogicalType::False,
    }
}

#[jsonbb_path_macros::register(target = SEARCH_FUNC)]
fn search(value: ValueType, rgx: ValueType) -> LogicalType {
    match (value.as_value(), rgx.as_value()) {
        (Some(ValueRef::String(s)), Some(ValueRef::String(r))) => Regex::new(r)
            .map(|r| r.is_match(s))
            .map(Into::into)
            .unwrap_or_default(),
        _ => LogicalType::False,
    }
}

#[jsonbb_path_macros::register(target = VALUE_FUNC)]
fn value(nodes: NodesType) -> ValueType {
    if nodes.len() > 1 {
        ValueType::Nothing
    } else {
        match nodes.first() {
            Some(v) => ValueType::Node(v),
            None => ValueType::Nothing,
        }
    }
}
