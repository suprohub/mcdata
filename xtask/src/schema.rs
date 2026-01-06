use std::{borrow::Cow, collections::BTreeMap};

use serde::{Deserialize, Serialize};

pub type FeaturesJson = Vec<Feature>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub mc: String,
    pub extractor: u8,
}

///////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksJson {
    pub blocks: Vec<Block>,
    pub enums: Vec<Enum>,
    pub map_colors: BTreeMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    #[serde(default)]
    pub experimental: bool,
    pub properties: Vec<Property>,
    pub map_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Property {
    Bool {
        name: String,
    },
    Int {
        name: String,
        min: u8,
        max: u8,
    },
    Enum {
        name: String,
        #[serde(rename = "enum")]
        enum_name: String,
    },
}

impl Property {
    pub fn name(&self) -> &str {
        match self {
            Property::Bool { name } => name,
            Property::Int { name, .. } => name,
            Property::Enum { name, .. } => name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}

///////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesJson<'a> {
    pub entities: Vec<Entity<'a>>,
    pub types: Vec<EntityType<'a>>,
    pub compound_types: Vec<CompoundType<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity<'a> {
    pub id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    #[serde(default)]
    pub experimental: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityType<'a> {
    pub name: Cow<'a, str>,
    pub parent: Option<Cow<'a, str>>,
    pub nbt: NbtCompound<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NbtCompound<'a> {
    pub entries: BTreeMap<String, NbtCompoundEntry<'a>>,
    pub unknown_keys: Option<NbtElement<'a>>,
    #[serde(default)]
    pub flattened: Vec<NbtElement<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompoundType<'a> {
    pub name: String,
    #[serde(flatten)]
    pub compound: NbtCompound<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NbtCompoundEntry<'a> {
    pub value: NbtElement<'a>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NbtElement<'a> {
    Any,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    ByteArray,
    IntArray,
    LongArray,
    Uuid,
    Boolean,
    Either {
        left: Box<NbtElement<'a>>,
        right: Box<NbtElement<'a>>,
    },
    List {
        inner: Box<NbtElement<'a>>,
    },
    AnyCompound {
        #[serde(rename = "valueType")]
        value_type: Box<NbtElement<'a>>,
    },
    Compound {
        name: Cow<'a, str>,
    },
    Boxed {
        name: Cow<'a, str>,
    },
    NestedEntity,
    BlockState,
}

impl<'a> NbtElement<'a> {
    pub fn as_rust_type(&self) -> Cow<'a, str> {
        match self {
            NbtElement::Any => "fastnbt::Value".into(),
            NbtElement::Byte => "i8".into(),
            NbtElement::Short => "i16".into(),
            NbtElement::Int => "i32".into(),
            NbtElement::Long => "i64".into(),
            NbtElement::Float => "f32".into(),
            NbtElement::Double => "f64".into(),
            NbtElement::String => "CowStr".into(),
            NbtElement::ByteArray => "fastnbt::ByteArray".into(),
            NbtElement::IntArray => "fastnbt::IntArray".into(),
            NbtElement::LongArray => "fastnbt::LongArray".into(),
            NbtElement::Uuid => "u128".into(),
            NbtElement::Boolean => "bool".into(),
            NbtElement::Either { left, right } => format!(
                "either::Either<{}, {}>",
                left.as_rust_type(),
                right.as_rust_type()
            )
            .into(),
            NbtElement::List { inner } => format!("Vec<{}>", inner.as_rust_type()).into(),
            NbtElement::AnyCompound { value_type } => {
                format!("HashMap<CowStr, {}>", value_type.as_rust_type()).into()
            }
            NbtElement::Compound { name } => format!("super::compounds::{name}").into(),
            NbtElement::Boxed { name } => format!("Box<super::compounds::{name}>").into(),
            NbtElement::NestedEntity => "Box<super::Entity>".into(),
            NbtElement::BlockState => "super::BlockState".into(),
        }
    }
}
