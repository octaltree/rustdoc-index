use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Crate {
    // doc: String,
    p: Vec<(ParentType, String)>,

    // t, n, q, d, i, f are items array
    t: Vec<ItemType>,
    n: Vec<String>,
    f: F,
    q: Vec<String>,
    d: Vec<String>,
    i: Vec<usize> // p idx
}

/// <https://github.com/rust-lang/rust/blob/71a567fae4c282aa5ecb1e6e48f020ade8df23e7/src/librustdoc/html/render/mod.rs>
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum F {
    V1_53_0(Vec<Option<Types>>),
    V1_55_0(Vec<Option<Types1_55_0>>),
    V1_58_0(Vec<Option<Types1_58_0>>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Types {
    OnlyArgs((Vec<(String, ItemType)>,)),
    WithResponse(Vec<(String, ItemType)>, ResponseType)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseType {
    Single((String, ItemType)),
    Complex(Vec<(String, ItemType)>)
}

/// <https://github.com/rust-lang/rust/blob/71a567fae4c282aa5ecb1e6e48f020ade8df23e7/src/librustdoc/html/render/cache.rs#L219>
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Types1_55_0 {
    OnlyArgs((Vec<Type1_55_0>,)),
    WithMultiResponse(Vec<Type1_55_0>, Vec<Type1_55_0>),
    WithResponse(Vec<Type1_55_0>, Type1_55_0)
}

#[derive(Debug)]
pub struct Type1_55_0 {
    name: String,
    generics: Option<Vec<String>>,
    kind: ItemType
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Types1_58_0 {
    OnlyArgs((Vec<Type1_58_0>,)),
    WithMultiResponse(Vec<Type1_58_0>, Vec<Type1_58_0>),
    WithResponse(Vec<Type1_58_0>, Type1_58_0)
}

#[derive(Debug)]
pub struct Type1_58_0 {
    name: String,
    generics: Option<Vec<Type1_58_0>>,
    kind: ItemType
}

impl<'de> serde::Deserialize<'de> for Type1_55_0 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum De {
            Two(String, ItemType),
            Three(String, ItemType, Vec<String>)
        }
        Ok(match De::deserialize(deserializer)? {
            De::Two(name, kind) => Self {
                name,
                generics: None,
                kind
            },
            De::Three(name, kind, generics) => Self {
                name,
                generics: Some(generics),
                kind
            }
        })
    }
}

impl<'de> serde::Deserialize<'de> for Type1_58_0 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum De {
            Two(String, ItemType),
            Three(String, ItemType, Vec<Type1_58_0>)
        }
        Ok(match De::deserialize(deserializer)? {
            De::Two(name, kind) => Self {
                name,
                generics: None,
                kind
            },
            De::Three(name, kind, generics) => Self {
                name,
                generics: Some(generics),
                kind
            }
        })
    }
}

#[derive(Debug, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum ParentType {
    Struct = 3,
    Enum = 4,
    Typedef = 6,
    Trait = 8,
    Variant = 13,
    Primitive = 15,
    Union = 19
}

/// rust/src/librustdoc/formats/item_type.rs
#[derive(Debug, serde_repr::Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ItemType {
    Module = 0,
    ExternCrate = 1,
    Import = 2,
    Struct = 3,
    Enum = 4,
    Function = 5,
    Typedef = 6,
    Static = 7,
    Trait = 8,
    Impl = 9,
    TyMethod = 10,
    Method = 11,
    StructField = 12,
    Variant = 13,
    Macro = 14,
    Primitive = 15,
    AssocType = 16,
    Constant = 17,
    AssocConst = 18,
    Union = 19,
    ForeignType = 20,
    Keyword = 21,
    OpaqueTy = 22,
    ProcAttribute = 23,
    ProcDerive = 24,
    TraitAlias = 25,
    Generic = 26
}

pub const FILETYPE: &[ItemType] = &[
    ItemType::Struct,
    ItemType::Union,
    ItemType::Enum,
    ItemType::Typedef,
    // Positioning after ty
    ItemType::Function,
    ItemType::Static,
    ItemType::Trait,
    ItemType::Macro,
    ItemType::Primitive,
    ItemType::Constant,
    ItemType::Keyword,
    ItemType::ProcAttribute,
    ItemType::ProcDerive,
    ItemType::TraitAlias
];

pub const STD_PRIMITIVES: &[&str] = &[
    "array",
    "bool",
    "char",
    "f32",
    "f64",
    "fn",
    "i128",
    "i16",
    "i32",
    "i64",
    "i8",
    "isize",
    "never",
    "pointer",
    "reference",
    "slice",
    "str",
    "tuple",
    "u128",
    "u16",
    "u32",
    "u64",
    "u8",
    "unit",
    "usize"
];

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ItemType::Module => "mod",
            ItemType::ExternCrate => "externcrate",
            ItemType::Import => "import",
            ItemType::Struct => "struct",
            ItemType::Union => "union",
            ItemType::Enum => "enum",
            ItemType::Function => "fn",
            ItemType::Typedef => "type",
            ItemType::Static => "static",
            ItemType::Trait => "trait",
            ItemType::Impl => "impl",
            ItemType::TyMethod => "tymethod",
            ItemType::Method => "method",
            ItemType::StructField => "structfield",
            ItemType::Variant => "variant",
            ItemType::Macro => "macro",
            ItemType::Primitive => "primitive",
            ItemType::AssocType => "associatedtype",
            ItemType::Constant => "constant",
            ItemType::AssocConst => "associatedconstant",
            ItemType::ForeignType => "foreigntype",
            ItemType::Keyword => "keyword",
            ItemType::OpaqueTy => "opaque",
            ItemType::ProcAttribute => "attr",
            ItemType::ProcDerive => "derive",
            ItemType::TraitAlias => "traitalias",
            ItemType::Generic => "generic"
        }
    }
}

#[derive(Debug, Error)]
#[error("Failed to parse ItemType")]
pub struct ParseItemTypeError;

impl FromStr for ItemType {
    type Err = ParseItemTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mod" => Ok(ItemType::Module),
            "externcrate" => Ok(ItemType::ExternCrate),
            "import" => Ok(ItemType::Import),
            "struct" => Ok(ItemType::Struct),
            "union" => Ok(ItemType::Union),
            "enum" => Ok(ItemType::Enum),
            "fn" => Ok(ItemType::Function),
            "type" => Ok(ItemType::Typedef),
            "static" => Ok(ItemType::Static),
            "trait" => Ok(ItemType::Trait),
            "impl" => Ok(ItemType::Impl),
            "tymethod" => Ok(ItemType::TyMethod),
            "method" => Ok(ItemType::Method),
            "structfield" => Ok(ItemType::StructField),
            "variant" => Ok(ItemType::Variant),
            "macro" => Ok(ItemType::Macro),
            "primitive" => Ok(ItemType::Primitive),
            "associatedtype" => Ok(ItemType::AssocType),
            "constant" => Ok(ItemType::Constant),
            "associatedconstant" => Ok(ItemType::AssocConst),
            "foreigntype" => Ok(ItemType::ForeignType),
            "keyword" => Ok(ItemType::Keyword),
            "opaque" => Ok(ItemType::OpaqueTy),
            "attr" => Ok(ItemType::ProcAttribute),
            "derive" => Ok(ItemType::ProcDerive),
            "traitalias" => Ok(ItemType::TraitAlias),
            _ => Err(ParseItemTypeError)
        }
    }
}

impl Crate {
    // TODO: duplicated methods
    pub fn items(self) -> Vec<String> {
        let Self { p, t, n, q, i, .. } = self;
        let items = (0..)
            .zip(t.into_iter())
            .zip(n.into_iter())
            .zip(i.into_iter())
            .zip(q.into_iter())
            .map(|((((no, t), n), i), q)| (no, t, n, i, q));
        let mut cd: String = String::new();
        items
            .map(|(_no, t, n, i, q)| {
                if !q.is_empty() {
                    cd = q;
                }
                if i == 0 {
                    format!("{}::{}	{}", &cd, n, t.as_str())
                } else {
                    format!("{}::{}::{}	{}", &cd, p[i - 1].1, n, t.as_str())
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_f() {
        let _: F = serde_json::from_str("[null]").unwrap();
        let _: F = serde_json::from_str(r#"[[[]]]"#).unwrap();
        let _: F = serde_json::from_str(r#"[[[], ["osstr", 3]]]"#).unwrap();
        let _: F = serde_json::from_str(r#"[[[["usize", 15]]]]"#).unwrap();
        let _: Vec<Type1_55_0> =
            serde_json::from_str(r#"[["pathbuf", 3], ["result", 6, ["pathbuf"]]]"#).unwrap();
        let _: F = serde_json::from_str(
            r#"[[
                [],
                [["pathbuf", 3], ["result", 6, ["pathbuf"]]]
            ]]"#
        )
        .unwrap();
        let _: F = serde_json::from_str(
            r#"[[
                [["utf8path", 3]],
                ["ordering", 4]
            ]]"#
        )
        .unwrap();
    }
}
