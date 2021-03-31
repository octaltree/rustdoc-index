use string_cache::DefaultAtom as Atom;

#[derive(Debug, Deserialize)]
pub struct Crate {
    doc: Atom,
    p: Vec<(ParentType, String)>,

    // t, n, q, d, i, f are items array
    t: Vec<ItemType>,
    n: Vec<String>,
    f: Vec<Option<Types>>,
    q: Vec<String>,
    d: Vec<String>,
    i: Vec<usize> // p idx
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Types {
    OnlyArgs((Vec<(String, usize)>,)),
    WithResponse(Vec<(String, usize)>, ResponseType)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseType {
    Single((String, usize)),
    Complex(Vec<(String, usize)>)
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
    TraitAlias = 25
}

impl ItemType {
    fn as_str(&self) -> &'static str {
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
            ItemType::TraitAlias => "traitalias"
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
