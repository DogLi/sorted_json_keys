use crate::{Error, JsonValue, Result};
use serde_json;
use std::collections::{BTreeMap, HashMap};
use serde::Serialize;

fn sort_list(value: JsonValue) -> Result<JsonValue> {
    let list: Vec<JsonValue> = value
        .as_array()
        .map(|x| x.to_owned())
        .ok_or(Error::ValueError)?;
    let mut new_list: Vec<JsonValue> = vec![];
    for json_value in list.into_iter() {
        new_list.push(sorted_json(json_value)?)
    }
    let v = serde_json::to_value(new_list)?;
    Ok(v)
}

fn sort_map(value: JsonValue) -> Result<JsonValue> {
    let map = value
        .as_object()
        .map(|x| x.to_owned())
        .ok_or(Error::ValueError)?;
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        new_map.insert(key, sorted_json(value)?);
    }
    let btree_map: BTreeMap<_, _> = new_map.iter().collect();
    let v = serde_json::to_value(btree_map)?;
    Ok(v)
}

pub fn sorted_json<S: Serialize>(data: S) -> Result<JsonValue> {
    let value = serde_json::to_value(data)?;
    if value.is_object() {
        return sort_map(value);
    }
    if value.is_array() {
        return sort_list(value);
    }
    Ok(value)
}
