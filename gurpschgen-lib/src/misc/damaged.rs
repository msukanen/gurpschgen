use crate::damage::{Damage, DamageDelivery};

pub trait Damaged {
    fn damage(&self) -> &Vec<Damage>;
    fn max_damage(&self) -> &Option<DamageDelivery>;
}
