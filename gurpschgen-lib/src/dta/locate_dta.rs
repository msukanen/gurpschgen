use std::{env, path::Path};

/**
 Locate DTA/GEN etc. file(s) and change current working directory to there.

 **Panics** if suitable directory not found.
 */
pub fn locate_dta(verbose: bool) {
    // where the datafiles might be lurking?
    let possible_dta_location = [
        "./datafiles",
        "./.dta",
        "./dta2json/datafiles",
        "../datafiles",
        "../.dta",
        "../dta2json/datafiles",
        "../dtafiles",
        "./dta",
        "../dta",
    ];

    let mut found_dtas = false;
    // Scan around - break early.
    for path in possible_dta_location {
        if env::set_current_dir(&Path::new(path)).is_ok() {
            // Found! We want to live there, too.
            let cwd = env::current_dir().unwrap();
            if verbose {println!("DTA found in {}", cwd.display());}
            found_dtas = true;
            break;
        }
    }

    if !found_dtas {
        panic!("We could not locate .dta/.gen (or other such) file(s) in any (internally) specified potential locations!")
    }
}
