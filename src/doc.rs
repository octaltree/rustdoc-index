use std::iter;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Deserialize)]
pub struct Crate {
    doc: Atom,
    p: Vec<(ParentType, String)>,

    // t, n, q, d, i, f are items array
    t: Vec<ItemType>,
    pub n: Vec<String>,
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
#[derive(Debug, serde_repr::Deserialize_repr)]
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

impl Crate {
    pub fn items(self) -> Vec<String> {
        let Self { p, t, n, q, i, .. } = self;
        let mut vars = iter::repeat(String::new())
            .take(p.len() + 1)
            .collect::<Vec<String>>();
        let mut result = iter::repeat(String::new())
            .take(t.len())
            .collect::<Vec<String>>();
        let items = (0..)
            .zip(t.into_iter())
            .zip(n.into_iter())
            .zip(i.into_iter())
            .zip(q.into_iter())
            .map(|((((no, t), n), i), q)| (no, t, n, i, q));
        let mut last_path: &str = "";
        for (no, _t, n, i, q) in items {
            if q != "" {
                vars[i] = q;
            } else if vars[i] == "" {
                vars[i] = last_path.to_owned();
            }
            result[no] = format!("{}::{}", &vars[i], n);
            last_path = &result[no];
        }
        result
    }
}
