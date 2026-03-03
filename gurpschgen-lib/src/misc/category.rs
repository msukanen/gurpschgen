use std::{collections::HashMap, fmt::Display};
#[cfg(not(feature = "dta2json"))]
use std::sync::Arc;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "dta2json"))]
use serde::{Deserializer, Serializer};

use crate::{adq::Adq, context::Context, dta::genre::Genre, equipment::Equipment, id::{HasRuntimeID, RuntimeID, StringHash}, misc::named::HasName, skill::Skill};
#[cfg(not(feature = "dta2json"))]
use crate::dta::vault::VAULT;

#[derive(Debug)]
pub enum CategoryError {
    JsonError(serde_json::Error),
}

impl From<serde_json::Error> for CategoryError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl Display for CategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonError(e) => write!(f, "{:?}", e),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(not(feature = "dta2json"))]
pub struct Category {
    pub name: String,
    #[serde(deserialize_with = "cat_items_deserializer", serialize_with = "cat_items_serializer")]
    pub items: HashMap<String, RuntimeID>,// storage : String/CategoryPayload
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg(feature = "dta2json")]
pub struct Category {
    pub name: String,
    pub items: HashMap<String, CategoryPayload>,
}

#[cfg(not(feature = "dta2json"))]
fn cat_items_deserializer<'de, D>(deserializer: D) -> Result<HashMap<String, RuntimeID>, D::Error>
where D: Deserializer<'de>,
{
    let cat_halp: HashMap<String, CategoryPayload> = Deserialize::deserialize(deserializer)?;
    let mut idmap = HashMap::new();
    for item in cat_halp.iter() {
        let mut lock = VAULT.registry.write().unwrap();
        let rtid = item.1.runtime_id();
        lock.entry(rtid).or_insert(item.1.clone());
        idmap.entry(item.0.clone()).or_insert(rtid);
    }
    Ok(idmap)
}

#[cfg(not(feature = "dta2json"))]
fn cat_items_serializer<S>(items: &HashMap<String, RuntimeID>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    let mut cat_halp = HashMap::new();
    let lock = VAULT.registry.read().unwrap();
    for (key, rtid) in items.iter() {
        cat_halp.entry(key).or_insert(lock.get(rtid).unwrap_or_else(|| panic!("FATAL: RTID {rtid} mysteriously evaporated mid-execution?")));
    }
    cat_halp.serialize(serializer)
}

impl Category {
    pub fn new(name: &str) -> Self {
        Category { name: name.into(), items: HashMap::new() }
    }

    #[cfg(not(feature = "dta2json"))]
    pub fn find(&self, what: &str) -> Option<CategoryPayload> {
        let rtid = self.items.get(what)?;
        let lock = VAULT.registry.read().unwrap();
        lock.get(rtid).cloned()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryPayload {
    Advantage(Adq),
    Bonus(String),
    Counter(String),
    Disadvantage(Adq),
    Equipment(Equipment),
    Genre(Genre),
    Modifier(String),
    Package(Adq),
    Quirk(String),
    Skill(Skill),
}

impl HasRuntimeID for CategoryPayload {
    fn runtime_id(&self) -> RuntimeID {
        match self {
            Self::Advantage(id_owner)    |
            Self::Disadvantage(id_owner) |
            Self::Package(id_owner)
                => id_owner.runtime_id(),

            Self::Bonus(s)    |
            Self::Counter(s)  |
            Self::Modifier(s) |
            Self::Quirk(s)
                => s.as_str().runtime_id(),

            Self::Equipment(e) => e.runtime_id(),

            other => unimplemented!("{other:?} has no RuntimeID! Don't even try!")
        }
    }
}

impl HasName for CategoryPayload {
    fn name(&self) -> &str {
        match self {
            Self::Advantage(v)    |
            Self::Disadvantage(v) |
            Self::Package(v)      => v.name(),
            Self::Bonus(v)     |
            Self::Counter(v)   |
            Self::Modifier(v)  |
            Self::Quirk(v)     => v.as_str(),
            Self::Equipment(v) => v.name(),
            Self::Genre(v) => &v.name,
            Self::Skill(v) => v.name(),
        }
    }
}

impl CategoryPayload {
    /// Derive [Context] from [payload][CategoryPayload].
    /// 
    /// Note that this is by nature of [CategoryPayload] a lossy conversion!
    pub fn as_approx_context(&self) -> Context {
        match self {
            Self::Advantage(_) => Context::Advantage,
            Self::Bonus(_) => Context::Bonus,
            Self::Counter(_) => Context::Counter,
            Self::Disadvantage(_) => Context::Disadvantage,
            Self::Equipment(_) => Context::Equipment,
            Self::Genre(_) => Context::Genre,
            Self::Modifier(_) => Context::Modifier,
            Self::Package(_) => Context::Package,
            Self::Quirk(_) => Context::Quirk,
            Self::Skill(_) => Context::Skill,
        }
    }
}