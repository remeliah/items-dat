//! A library to deserialize Growtopia's items.dat files.
//! 
//! ## Usage
//! 
//! ```
//! use items_dat::ItemsDat;
//! 
//! let items_dat = ItemsDat::from_path("resources/items.dat").unwrap();
//! let first_item = &items_dat.items[0];
//! println!("items.dat version: {}", items_dat.version);
//! println!("Number of items: {}", items_dat.item_count);
//! println!("First item: {first_item:?}");
//! ```
//! 

mod error;
mod item;
mod parser;
mod reader;
mod decompressor;

pub use error::ItemsDatError;
pub use item::{Item, ItemsDat};
pub use decompressor::decompress_items_dat;

use std::fs::File;
use std::io::Read;
use std::path::Path;

impl ItemsDat {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ItemsDatError> {
        parser::parse_items_dat(bytes)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ItemsDatError> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        Self::from_bytes(&buf)
    }

    pub fn from_compressed_file<P: AsRef<Path>>(path: P) -> Result<Self, ItemsDatError> {
        let c = std::fs::read(path)?;
        let dec = decompress_items_dat(&c)?;

        Self::from_bytes(&dec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn empty_bytes_fails() {
        let items_dat = ItemsDat::from_bytes(&[]);
        assert!(items_dat.is_err());
    }
    
    #[test]
    fn from_file() {
        let items_dat = ItemsDat::from_path("resources/items.dat").unwrap();

        assert_eq!(items_dat.version, 22);
        assert_eq!(items_dat.item_count, 15796);
    }

    #[test]
    fn from_bytes() {
        let items_dat = ItemsDat::from_bytes(
            include_bytes!("../resources/items.dat")
        ).unwrap();

        assert_eq!(items_dat.version, 22);
        assert_eq!(items_dat.item_count, 15796);
    }
}
