use regex::Regex;

/**
 Container for advantages, disadvantages and quirks.
 */
#[derive(Debug, Clone)]
pub struct Adq {
    name: String,
    initial_cost: i32,
    cost_increment: i32,
    max_level: i32,
    bonus_mods: Vec<String>,
    given: Vec<String>,
    mod_groups: Vec<String>,
}

impl From<(&str, &str)> for Adq {
    /**
     Advantages and Disadvantages have the form:
     
     name;
     initial cost/cost increment; max no. LEVELS; bonus mods; given ads/disads/skills; modifier groups used
    */
    fn from(value: (&str, &str)) -> Self {
        let rx = Regex::new(r"^\s*((?<c1>\d+)\s*/\s*(?<c2>\d+)|(?<c3>\d+))(\s*;\s*((?<maxlvl>\d+)?(\s*;\s*((?<bonus>[^;]*)(\s*;\s*((?<given>[^;]*)(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap();
        let name = String::from(value.0);
        if let Some(caps) = rx.captures(value.1) {
            let initial_cost;
            let mut cost_increment = 0;
            let mut max_level = 1;
            let mut bonus_mods = vec![];
            let mut given = vec![];
            let mut mod_groups = vec![];

            // Let's deal with (c1/c2)|(c3) regexes first.
            if let Some(cap) = caps.name("c1") {
                // Note that c1 & c2 capture at once and so we can just unwrap c2 instead of specifically checking for it.
                initial_cost = cap.as_str().parse::<i32>().unwrap();
                cost_increment = caps.name("c2").unwrap().as_str().parse::<i32>().unwrap();
            } else if let Some(cap) = caps.name("c3") {
                initial_cost = cap.as_str().parse::<i32>().unwrap();
            } else {
                panic!("FATAL: cost not defined in {:?}", value.1)
            }

            // Got max level defined?
            if let Some(cap) = caps.name("maxlvl") {
                max_level = cap.as_str().parse::<i32>().unwrap();
            }

            if let Some(cap) = caps.name("bonus") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        bonus_mods.push(x.to_string())
                    }
                }
            }

            if let Some(cap) = caps.name("given") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        given.push(x.trim().to_string())
                    }
                }
            }

            if let Some(cap) = caps.name("modgr") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        mod_groups.push(x.trim().to_string())
                    }
                }
            }

            Adq {
                name,
                initial_cost,
                cost_increment,
                max_level,
                bonus_mods,
                given,
                mod_groups,
            }
        } else {
            panic!("FATAL: malformed ADQ: {:?}", value.1)
        }
    }
}

#[cfg(test)]
mod adq_tests {
    use super::Adq;

    #[test]
    fn adq_is_constructed_from_short_real_data() {
        let data = "10/5; 2";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
    }

    #[test]
    fn adq_is_constructed_from_partial_real_data() {
        let data = "10/5; 2;;Gluttony, Mohican";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
        assert_eq!(2, adq.given.len());
    }

    #[test]
    fn adq_is_constructed_from_full_real_data() {
        let data = "10/5; 2;;Gluttony, Mohican; Toxifiers, Motorists, Woke";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
        assert_eq!(2, adq.given.len());
        assert_eq!(3, adq.mod_groups.len());
    }

    #[test]
    fn adq_is_constructed_from_mixed_and_extra_data() {
        let data = "10/5; 2;;, Mohican; Toxifiers, Motorists, Woke;Bongo";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
        assert_eq!(1, adq.given.len());
        assert_eq!(3, adq.mod_groups.len());
    }
}
