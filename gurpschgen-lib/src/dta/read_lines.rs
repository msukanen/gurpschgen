use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::Path};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

/// Read lines from `filename`.
///
/// # Returns
/// `Ok`'d buffered line reader, or `Err`.
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<DecodeReaderBytes<File, Vec<u8>>>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    
    // Some DTA files notoriously contain WINDOWS_1252 encoded characters,
    // and those do not comply with UTF-8, making use of a decoder a necessity.
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    Ok(BufReader::new(decoder).lines())
}
