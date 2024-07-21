/**
 General damage types + embedded delivery method.
 */
#[derive(Debug, Clone)]
pub enum Damage {
    Cut(DamageDelivery),
    Cr(DamageDelivery),
    Imp(DamageDelivery),
}

/**
 Some common damage delivery methods.
 */
#[derive(Debug, Clone)]
pub enum DamageDelivery {
    /**
     **Fixed**: num dice & modifier. Guns and other weapons that have relatively stable/fixed dmg model.
     */
    Fixed(i32, i32),
    /**
     **Sw**ing: based on ST. Embedded modifier.
     */
    Sw(i32),
    /**
     **Thr**ust: based on ST. Embedded modifier.
     */
    Thr(i32),
}
