use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{attrib::{Attribute, AttributeType, HasAttributeValue}, context::Context, gender::Gender, id::{RuntimeID, StringHash}, misc::{category::CategoryPayload, r#const::UNNAMED, costly::HasCost, named::HasName}};

/// PC/NPC container.
#[derive(Debug, Deserialize, Serialize)]
pub struct Ch {
    pub name: String,
    pub gender: Option<Gender>,
    pub st: Attribute,
    pub dx: Attribute,
    pub iq: Attribute,
    pub ht: Attribute,
    extra_hp: i32,
    extra_will: i32,
    extra_per: i32,
    extra_fp: i32,
    extra_speed: i32,
    extra_move: i32,
    #[serde(deserialize_with = "ch_items_deserialize", serialize_with = "ch_items_serialize")]
    pub items: HashMap<Context, HashMap<RuntimeID, CategoryPayload>>,
}

#[derive(Deserialize, Serialize)]
struct ChItemsRTvStorage {
    items: HashMap<Context, HashMap<String, CategoryPayload>>,
}

/// Deserialize String-keyed JSON payload into RuntimeID keyed `items` field.
fn ch_items_deserialize<'de, D>(deserializer: D)
-> Result<HashMap<Context, HashMap<RuntimeID, CategoryPayload>>, D::Error>
where D: Deserializer<'de>,
{
    let jh: ChItemsRTvStorage = Deserialize::deserialize(deserializer)?;
    let mut rtmap = HashMap::new();
    for (ctx, map) in jh.items {
        let mut submap = HashMap::new();
        for (iname, pl) in map.iter() {
            submap.insert(iname.as_str().runtime_id(), pl.clone());
        }
        rtmap.insert(ctx, submap);
    }

    Ok(rtmap)
}

/// Serialize RuntimeID-keyed `items` into String-keyed JSON.
fn ch_items_serialize<S>(
    items: &HashMap<Context, HashMap<RuntimeID, CategoryPayload>>,
    serializer: S
) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    let mut chmap = HashMap::new();
    for (ctx, map) in items {
        let submap = chmap
            .entry(ctx)
            .or_insert(HashMap::new());
        for pl in map.values() {
            submap.entry(pl.name()).or_insert(pl.clone());
        }
    }
    chmap.serialize(serializer)
}

impl Default for Ch {
    fn default() -> Self {
        Self {
            name: UNNAMED.into(),
            gender: None,
            dx: Attribute::default(AttributeType::DX),
            ht: Attribute::default(AttributeType::HT),
            iq: Attribute::default(AttributeType::IQ),
            st: Attribute::default(AttributeType::ST),
            extra_hp: 0,
            extra_will: 0,
            extra_per: 0,
            extra_fp: 0,
            extra_speed: 0,
            extra_move: 0,
            items: HashMap::new(),
        }
    }
}

impl Ch {
    /// Instantiate a blank (or nearly blank) `Ch`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            // note that we do not set gender here. In some genres it's a disadvantage and thus handled separately elsewhere.
            ..Self::default()
        }
    }

    /// Get `Ch`'s **h**it **p**oints (HP).
    pub fn hp(&self) -> i32 {
        self.st.value() + self.extra_hp
    }

    /// Get `Ch`'s **w**ill**p**ower (WP).
    pub fn wp(&self) -> i32 {
        self.iq.value() + self.extra_will
    }

    /// Get `Ch`'s ***per**ception (Per).
    pub fn per(&self) -> i32 {
        self.iq.value() + self.extra_per
    }

    /// Get `Ch`'s **f**atigue **p**oints (FP).
    pub fn fp(&self) -> i32 {
        self.ht.value() + self.extra_fp
    }

    /// Get `Ch`'s basic **speed** score.
    pub fn speed(&self) -> f64 {
        (self.ht.value() + self.dx.value() + self.extra_speed) as f64 / 4.0
    }

    /// Get `Ch`'s basic **move** score (yd/s).
    // `move` is a reserved word and thus `mov()` instead.
    pub fn mov(&self) -> i32 {
        (self.speed() + self.extra_move as f64).trunc() as i32
    }
}

impl HasCost for Ch {
    fn cost(&self) -> f64 {
          self.dx.cost()
        + self.ht.cost()
        + self.iq.cost()
        + self.st.cost()
        + 2.0 * self.extra_hp as f64
        + 5.0 * self.extra_will as f64
        + 5.0 * self.extra_per as f64
        + 3.0 * self.extra_fp as f64
        + 5.0 * self.extra_speed as f64
        + 5.0 * self.extra_move as f64
    }
}

#[cfg(all(test, not(feature = "dta2json")))]
mod ch_tests {
    use crate::{context::Context, test::common_between_tests::{TEST_GENRE_NAME_TL3, prepare_test_environment}};

    use super::Ch;

    #[test]
    fn init_works() {
        let ch = Ch::new("Nameless");
        assert_eq!("Nameless", ch.name);
        assert_eq!(10, ch.st);
    }

    #[test]
    fn speed_works() {
        let mut ch = Ch::new("Nameless");
        assert_eq!(5.0, ch.speed());
        assert_eq!(5, ch.mov());

        ch.dx += 2;
        assert_eq!(5.5, ch.speed());
        assert_eq!(5, ch.mov());

        ch.ht += 2;
        assert_eq!(6.0, ch.speed());
        assert_eq!(6, ch.mov());

        ch.extra_speed = 1;
        assert_eq!(6.25, ch.speed());
        
        ch.extra_move = 1;
        assert_eq!(7, ch.mov());
    }

    #[test]
    fn adding_advantage_works() {
        let mf = prepare_test_environment();
        let genre = mf.find_genre(TEST_GENRE_NAME_TL3)
            .expect(&format!("No genre '{TEST_GENRE_NAME_TL3}' found?!"));

        let ctx = Context::Advantage;
        let name = "Luck";
        let category = "Mental Advantages";
        let item = genre.find(ctx, category, name);
        let Some(item) = item else {panic!("No Luck here!")};
        log::debug!("{item:?}");
    }
}
