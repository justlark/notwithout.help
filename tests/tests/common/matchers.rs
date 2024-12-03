use std::{fmt, marker::PhantomData};

use serde_json::Value as JsonValue;
use xpct::{all, be_some, core::Matcher, why};

pub trait JsonType {
    type Type;

    fn from_json(value: JsonValue) -> Option<Self::Type>;
}

#[derive(Debug)]
pub struct JsonAny;

impl JsonType for JsonAny {
    type Type = JsonValue;

    fn from_json(value: JsonValue) -> Option<JsonValue> {
        Some(value)
    }
}

#[derive(Debug)]
pub struct JsonString;

impl JsonType for JsonString {
    type Type = String;

    fn from_json(value: JsonValue) -> Option<Self::Type> {
        if let JsonValue::String(s) = value {
            Some(s)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct JsonArray<T = JsonAny> {
    _phantom: PhantomData<T>,
}

impl<T> JsonType for JsonArray<T>
where
    T: JsonType,
{
    type Type = Vec<T::Type>;

    fn from_json(value: JsonValue) -> Option<Self::Type> {
        if let JsonValue::Array(vec) = value {
            vec.into_iter()
                .map(T::from_json)
                .collect::<Option<Vec<_>>>()
        } else {
            None
        }
    }
}

pub fn have_type<'a, T>() -> Matcher<'a, JsonValue, T::Type, ()>
where
    T: JsonType + 'a,
    T::Type: fmt::Debug,
{
    all(move |ctx| {
        ctx.map(|v| T::from_json(v))
            .to(why(be_some(), "JSON value is not the expected type"))
    })
}

pub fn have_field<'a, T>(field: &'a str) -> Matcher<'a, JsonValue, T::Type, ()>
where
    T: JsonType + 'a,
    T::Type: fmt::Debug,
{
    all(move |ctx| {
        ctx.map(|v: JsonValue| {
            if let JsonValue::Object(mut map) = v {
                map.remove(field)
            } else {
                None
            }
        })
        .to(why(be_some(), "JSON object is missing expected field"))?
        .map(T::from_json)
        .to(why(be_some(), "JSON field is not the expected type"))
    })
}
