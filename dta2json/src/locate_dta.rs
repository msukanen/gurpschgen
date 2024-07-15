use std::{env, path::Path};

/**
 Locate .DTA/.GEN file(s) and change current working directory there.

 **Panics** if suitable directory not found.
 */
pub(crate) fn locate_dta() {
    // where the datafiles might be lurking?
    let possible_dta_location = [
        "./datafiles",
        "./.dta",
        "./dta2json/datafiles",
        "../datafiles",
        "../.dta",
        "../dta2json/datafiles",
    ];
    let mut found_dtas = false;
    for path in possible_dta_location {
        if env::set_current_dir(&Path::new(path)).is_ok() {
            let cwd = env::current_dir().unwrap();
            println!("DTA found in {}", cwd.display());
            found_dtas = true;
            break;
        }
    }
    if !found_dtas {
        panic!("We could not locate .dta/.gen file(s) in any (internally) specified potential locations!")
    }
}
