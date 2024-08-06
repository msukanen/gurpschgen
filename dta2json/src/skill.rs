use gurpschgen_lib::skill::{Skill, SkillDefault};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::skillroot::skillroot_from_str;

pub(crate) static RX_SIMPLE: Lazy<Regex> = Lazy::new(||Regex::new(r"^(?:\s*(?<anything>[^;]+))").unwrap());

pub(crate) fn skill_from_tuple(value: (&str, &str)) -> Skill {
    static RX_DEF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<name>.+)(?<def>[-+]\d+)\s*$)").unwrap());
    static RX_MAS_DEF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?<what>.+)(?<mode>[-+*/])(?<val>\d+[.]?\d+)\s*$)").unwrap());
    static RX_TL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:TL)").unwrap());
    static RX_GBONUS: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:(?<bv1>[-+]\d+)\s+(?<bname1>\w+.*))|(?:(?<bname2>.+)(?<bv2>[-+]\d+)\s*$))").unwrap());
    static RX_GIVES: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<what>[^@]+)@(?<val>\d+))").unwrap());
    
    let name = value.0.trim();
    let mut base = None;
    let mut defaults = vec![];
    let mut affected_by_bonuses = vec![];
    let mut tl_dependant = false;
    let mut increases_counters = vec![];
    let mut gives_bonuses = vec![];
    let mut gives = vec![];

    for (index, x) in value.1.split(";").into_iter().enumerate() {
        let x = x.trim();
        if x.is_empty() { continue; }
        
        match index {
            // Skill base.
            0 => {
                base = skillroot_from_str(x.trim()).into();
                if let Some(_) = RX_TL.captures(x) {
                    tl_dependant = true
                }
            },
            // Sort out skill default(s).
            1 => {
                let ds = x.split(",");
                for d in ds {
                    if let Some(x) = RX_DEF.captures(d) {
                        let v = if let Some(def) = x.name("def") {
                            def.as_str().parse::<i32>().unwrap()
                        } else {0};
                        defaults.push(SkillDefault::Add { at: x.name("name").unwrap().as_str().trim().to_string(), val: v })
                    } else if let Some(x) = RX_MAS_DEF.captures(d) {
                        let n = x.name("what").unwrap().as_str().trim();
                        let v = x.name("val").unwrap().as_str();
                        defaults.push(match x.name("mode").unwrap().as_str() {
                            "/" => SkillDefault::Div { at: n.to_string(), val: v.parse::<f64>().unwrap() },
                            _   => SkillDefault::Mul { at: n.to_string(), val: v.parse::<f64>().unwrap() }
                        })
                    } else {
                        defaults.push(SkillDefault::Add { at: d.trim().to_string(), val: 0 })
                    }
                }
            },
            2 => {
                let gs = x.split(",");
                for g in gs {
                    if let Some(x) = RX_GIVES.captures(g) {
                        gives.push((
                            x.name("what").unwrap().as_str().trim().to_string(),
                            x.name("val").unwrap().as_str().parse::<i32>().unwrap()
                        ))
                    }
                }
            },
            // Bonuses affecting the skill.
            3 => {
                let bs = x.split(",");
                for b in bs {
                    let b = b.trim();
                    if !b.is_empty() {
                        affected_by_bonuses.push(b.to_string())
                    }
                }
            },
            // Counters the skill counts as...
            4|6 => {//TODO: is 6 actually for real, or is 
                    //    Storm (G) → M/H; ; Unus. Background:Grimoire Spells@1, Rain@12, Hail@12; ; Weather Water-Spells Count; ; Weather Air-Spells Count
                    // a bugged entry in original DTA?
                let cs = x.split(",");
                for c in cs {
                    let c = c.trim();
                    if !c.is_empty() {
                        increases_counters.push(c.to_string())
                    }
                }
            },
            // Bonuses what the skill gives...
            5 => {
                let bs = x.split(",");
                for b in bs {
                    if let Some(x) = RX_GBONUS.captures(b) {
                        if let Some(b) = x.name("bname1") {
                            let Some(v) = x.name("bv1") else {panic!("FATAL: regex fail?!")};
                            gives_bonuses.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
                        } else /*bname2*/{
                            let Some(b) = x.name("bname2") else {panic!("FATAL: regex fail?!")};
                            let Some(v) = x.name("bv2") else {panic!("FATAL: regex fail?!")};
                            gives_bonuses.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
                        }
                    } else {
                        todo!("FATAL: unrecognized 5th for {}: \"{b}\"", value.0)
                    }
                }
            }
            _=> todo!("{x}")
        }
    }

    Skill { rank: 0,
        name: name.to_string(),
        base: base.unwrap(),
        defaults, affected_by_bonuses,
        tl_dependant, increases_counters,
        gives_bonuses, gives,
    }
}

#[cfg(test)]
mod skill_tests {
    use gurpschgen_lib::{misc::named::Named, skill::{DifficultyRating, Skill, SkillDefault, SkillRoot, Stat}};

    use crate::skill::skill_from_tuple;

    #[test]
    fn very_basics_stat_wrong() {
        let data = ("<test>", "M/H(ST); Alchemy+0, Digity-2, Dignus B +3");
        let sk = skill_from_tuple(data);
        assert_ne!(SkillRoot::M { stat: Stat::ST, diff: DifficultyRating::H }, sk.base);
    }

    #[test]
    fn more_complex() {
        let data = ("Karate", "P/H; Karate Art-3, Karate Sport-3; ; +Melee Weapon Bonus; ; +5 Punching Damage Bonus, Kicking Damage Bonus +5");
        let sk = skill_from_tuple(data);
        assert_eq!(vec![
            SkillDefault::Add { at: "Karate Art".to_string(), val: -3 },
            SkillDefault::Add { at: "Karate Sport".to_string(), val: -3 }], sk.defaults);
        assert_eq!(vec![("Punching Damage Bonus".to_string(), 5), ("Kicking Damage Bonus".to_string(), 5)], sk.gives_bonuses);
    }

    #[test]
    fn lengthy_line_gives_works() {
        let data = ("INT", "M/E; IQ-0; Acting@13, Acrobatics@11, Administration@13, Blowpipe@12, Carousing@13, Computer Operation@14, Computer Programming@11, Criminology@11, Cryptanalysis@10, Dancing@10, Demolition@11, Detect Lies@13, Diagnosis@9, Diplomacy@13, Disguise@11, Electronics Operation: Communications@12, Electronics Operation: Security Systems@14, Escape@10, Explosive Ordnance Disposal@10, Fast Draw: Pistol@13, Fast Draw: Knife@11, Fast-Talk@13, First Aid@12, Forensics@10, Forgery@12, Gesture@13, Guns: Pistol@15, Guns: Submachine Gun@11, Holdout@13, Intelligence Analysis@13, Interrogation@12, Judo@11, Knife@10, Lockpicking@10, Motorcycle@10, Photography@11, Pickpocket@10, Poisons@9, Research@13, Sex Appeal@12, Shadowing@13, Shortsword@9, SIGINT Collection and Jamming@10, Sign Language@12, Skiing@9, Stealth@12, Streetwise@12, Swimming@13, Throwing@9, Tracking@10, Traffic Analysis@12, Traps@11;");
        let sk = skill_from_tuple(data);
        let mut found = false;
        for x in sk.gives {
            if x.0.eq("Computer Programming") && x.1.eq(&11) {
                found = true;
                break;
            }
        }
        assert_eq!(true, found);
    }

    #[test]
    fn defaults_work_without_explicit_value_given() {
        let data = ("Beam Weapons: Lasers", "P/E, TL; DX-4, Beam Weapons: Electrolasers-4, Beam Weapons: Blasters-4, Beam Weapons: Flamers-4, Beam Weapons: Sonic-4, Beam Weapons: Neural-4, Beam Weapons: Force Beams; ; +High IQ Guns Bonus");
        let sk = skill_from_tuple(data);
        assert_eq!(vec![
            SkillDefault::Add { at: "DX".to_string(), val: -4 },
            SkillDefault::Add { at: "Beam Weapons: Electrolasers".to_string(), val: -4},
            SkillDefault::Add { at: "Beam Weapons: Blasters".to_string(), val: -4},
            SkillDefault::Add { at: "Beam Weapons: Flamers".to_string(), val: -4},
            SkillDefault::Add { at: "Beam Weapons: Sonic".to_string(), val: -4},
            SkillDefault::Add { at: "Beam Weapons: Neural".to_string(), val: -4},
            SkillDefault::Add { at: "Beam Weapons: Force Beams".to_string(), val: 0}], sk.defaults);
    }

    #[test]
    fn serde_stat_works() {
        let stat = vec![SkillRoot::P { stat: Stat::HT, diff: DifficultyRating::H }, SkillRoot::MA { diff: DifficultyRating::A }];
        let json = serde_json::to_string(&stat).unwrap();
        println!("{json}");
    }

    #[test]
    fn serde_skill_works() {
        let sk = Skill {
            name: "Sinking".to_string(),
            rank: 2,
            base: SkillRoot::P { stat: Stat::ST, diff: DifficultyRating::E },
            defaults: vec![SkillDefault::Add { at: "Swimming".to_string(), val: 2 }],
            affected_by_bonuses: vec!["Overweight".to_string()],
            tl_dependant: false,
            increases_counters: vec![],
            gives: vec![],
            gives_bonuses: vec![],
        };
        let json = serde_json::to_string(&sk).unwrap();
        println!("{json}");
        let sk: Skill = serde_json::from_str(&json).unwrap();
        assert_eq!("Sinking".to_string(), sk.name());
        assert_eq!(SkillRoot::P { stat: Stat::ST, diff: DifficultyRating::E }, sk.base);
    }
}
