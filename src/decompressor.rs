use crate::error::ItemsDatError;
use flate2::read::ZlibDecoder;

use std::io::Read;

pub fn decompress_items_dat(compressed_data: &[u8]) -> Result<Vec<u8>, ItemsDatError> {
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut dec = Vec::new();
    
    decoder
        .read_to_end(&mut dec)
        .map_err(|e| ItemsDatError::DecompressionError(e.to_string()))?;
    
    Ok(dec)
}
