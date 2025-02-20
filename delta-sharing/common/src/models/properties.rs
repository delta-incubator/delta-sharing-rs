use std::collections::HashMap;

use crate::error::{Error, Result};

pub type JsonStruct = serde_json::Map<String, serde_json::Value>;
pub type ProtoStruct = pbjson_types::Struct;
pub type ProtoValue = pbjson_types::Value;
pub type JsonValue = serde_json::Value;
pub type PropertyMap = HashMap<String, JsonValue>;

pub trait IntoJson {
    fn into_json(self) -> serde_json::Value;
}

impl IntoJson for ProtoValue {
    fn into_json(self) -> serde_json::Value {
        match self.kind {
            Some(kind) => match kind {
                pbjson_types::value::Kind::NullValue(_) => serde_json::Value::Null,
                pbjson_types::value::Kind::NumberValue(n) => {
                    serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap())
                }
                pbjson_types::value::Kind::StringValue(s) => serde_json::Value::String(s),
                pbjson_types::value::Kind::BoolValue(b) => serde_json::Value::Bool(b),
                pbjson_types::value::Kind::StructValue(struct_value) => serde_json::Value::Object(
                    struct_value
                        .fields
                        .into_iter()
                        .map(|(name, val)| (name, val.into_json()))
                        .collect(),
                ),
                pbjson_types::value::Kind::ListValue(list_value) => serde_json::Value::Array(
                    list_value
                        .values
                        .into_iter()
                        .map(|value| value.into_json())
                        .collect(),
                ),
            },
            None => serde_json::Value::Null,
        }
    }
}

impl IntoJson for JsonValue {
    fn into_json(self) -> serde_json::Value {
        self
    }
}

impl IntoJson for PropertyMap {
    fn into_json(self) -> serde_json::Value {
        serde_json::Value::Object(self.into_json_struct())
    }
}

impl IntoProtoStruct for PropertyMap {
    fn into_proto_struct(self) -> ProtoStruct {
        ProtoStruct {
            fields: self
                .into_iter()
                .map(|(name, value)| (name, value.into_proto()))
                .collect(),
        }
    }
}

pub trait IntoJSONStruct {
    fn into_json_struct(self) -> serde_json::Map<String, serde_json::Value>;
}

impl IntoJSONStruct for ProtoStruct {
    fn into_json_struct(self) -> serde_json::Map<String, serde_json::Value> {
        self.fields
            .into_iter()
            .map(|(name, value)| (name, value.into_json()))
            .collect()
    }
}

impl IntoJSONStruct for PropertyMap {
    fn into_json_struct(self) -> serde_json::Map<String, serde_json::Value> {
        self.into_iter().collect()
    }
}

pub trait IntoProto {
    fn into_proto(self) -> pbjson_types::Value;
}

impl IntoProto for JsonValue {
    fn into_proto(self) -> pbjson_types::Value {
        match self {
            serde_json::Value::Null => pbjson_types::Value { kind: None },
            serde_json::Value::Bool(b) => pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::BoolValue(b)),
            },
            serde_json::Value::Number(n) => pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::NumberValue(
                    n.as_f64().unwrap_or_default(),
                )),
            },
            serde_json::Value::String(s) => pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::StringValue(s)),
            },
            serde_json::Value::Array(a) => pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::ListValue(
                    pbjson_types::ListValue {
                        values: a.into_iter().map(|v| v.into_proto()).collect(),
                    },
                )),
            },
            serde_json::Value::Object(o) => pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::StructValue(ProtoStruct {
                    fields: o.into_iter().map(|(k, v)| (k, v.into_proto())).collect(),
                })),
            },
        }
    }
}

impl IntoProto for ProtoValue {
    fn into_proto(self) -> pbjson_types::Value {
        self
    }
}

pub trait IntoProtoStruct {
    fn into_proto_struct(self) -> ProtoStruct;
}

impl IntoProtoStruct for JsonStruct {
    fn into_proto_struct(self) -> ProtoStruct {
        let fields = self
            .into_iter()
            .map(|(name, value)| {
                let proto_value = value.into_proto();
                (name, proto_value)
            })
            .collect();

        ProtoStruct { fields }
    }
}

#[inline]
pub fn to_json_struct(value: impl IntoJson) -> Result<JsonStruct> {
    value
        .into_json()
        .as_object()
        .cloned()
        .ok_or_else(|| Error::generic("Expected JSON object"))
}

pub struct PropertyMapHandler;

impl PropertyMapHandler {
    pub fn json_to_json_struct(value: impl IntoJson) -> Result<JsonStruct> {
        to_json_struct(value)
    }

    pub fn json_to_proto_struct(value: impl IntoJson) -> Result<ProtoStruct> {
        match value.into_json().into_proto() {
            pbjson_types::Value {
                kind: Some(pbjson_types::value::Kind::StructValue(s)),
            } => Ok(s),
            _ => Err(Error::generic("Expected JSON object")),
        }
    }

    pub fn proto_struct_to_json(value: ProtoStruct) -> JsonValue {
        JsonValue::Object(value.into_json_struct())
    }

    pub fn to_proto_struct(value: PropertyMap) -> ProtoStruct {
        value.into_proto_struct()
    }
}
