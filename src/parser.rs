#![allow(clippy::field_reassign_with_default)]

use crate::error::ItemsDatError;
use crate::item::{Item, ItemsDat};
use crate::reader::ByteReader;

const KNOWN_XOR_KEY: &[u8] = b"PBG892FXX982ABC*";

pub fn parse_items_dat(bytes: &[u8]) -> Result<ItemsDat, ItemsDatError> {
    let mut reader = ByteReader::new(bytes);

    let version = reader.read_i16()?;
    let item_count = reader.read_i32()?;

    let mut items: Vec<Item> = Vec::with_capacity(item_count as usize);

    for i in 0..item_count {
        let i = parse_item(&mut reader, version, i as u32)?;
        items.push(i);
    }


    Ok(ItemsDat {
        version,
        item_count,
        items,
    })
}

fn parse_item(reader: &mut ByteReader, version: i16, item_index: u32) -> Result<Item, ItemsDatError> {
    let mut item = Item::default();
    
    item.id = reader.read_u32()?;
    item.properties = reader.read_u16()?;
    item.item_type = reader.read_u8()?;
    item.material = reader.read_u8()?;
    item.name = reader.read_xor_encrypted_string(item_index, KNOWN_XOR_KEY)?;
    item.file_name = reader.read_length_prefixed_string()?;
    item.file_hash = reader.read_u32()?;
    item.visual_type = reader.read_u8()?;
    item.cook_time = reader.read_u32()?;
    item.tex_x = reader.read_u8()?;
    item.tex_y = reader.read_u8()?;
    item.storage_type = reader.read_u8()?;
    item.layer = reader.read_u8()?;
    item.collision_type = reader.read_u8()?;
    item.hardness = reader.read_u8()?;
    item.regen_time = reader.read_u32()?;
    item.clothing_type = reader.read_u8()?;
    item.rarity = reader.read_u16()?;
    item.max_hold = reader.read_u8()?;
    item.alt_file_path = reader.read_length_prefixed_string()?;
    item.alt_file_hash = reader.read_u32()?;
    item.anim_ms = reader.read_u32()?;

    if version >= 4 {
        item.pet_name = Some(reader.read_length_prefixed_string()?);
        item.pet_prefix = Some(reader.read_length_prefixed_string()?);
        item.pet_suffix = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 5 {
        item.pet_ability = Some(reader.read_length_prefixed_string()?);
    }
    
    item.seed_base = reader.read_u8()?;
    item.seed_over = reader.read_u8()?;
    item.tree_base = reader.read_u8()?;
    item.tree_over = reader.read_u8()?;
    item.bg_col = reader.read_u32()?;
    item.fg_col = reader.read_u32()?;
    item.seed1 = reader.read_u16()?;
    item.seed2 = reader.read_u16()?;
    item.bloom_time = reader.read_u32()?;

    if version >= 7 {
        item.anim_type = Some(reader.read_u32()?);
        item.anim_string = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 8 {
        item.anim_tex = Some(reader.read_length_prefixed_string()?);
        item.anim_string2 = Some(reader.read_length_prefixed_string()?);
        item.dlayer1 = Some(reader.read_u32()?);
        item.dlayer2 = Some(reader.read_u32()?);
    }

    if version >= 9 {
        item.properties2 = Some(reader.read_u16()?);
        item.unk = Some(reader.read_n_bytes(62)?);
    }

    if version >= 10 {
        item.tile_range = Some(reader.read_u32()?);
        item.pile_range = Some(reader.read_u32()?);
    }

    if version >= 11 {
        item.custom_punch = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 12 {
        item.unk2 = Some(reader.read_n_bytes(13)?);
    }

    if version >= 13 {
        item.clock_div = Some(reader.read_u32()?);
    }

    if version >= 14 {
        item.parent_id = Some(reader.read_u32()?);
    }

    if version >= 15 {
        item.unk3 = Some(reader.read_n_bytes(25)?);
        item.alt_sit_path = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 16 {
        item.unk4 = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 17 {
        item.unk5 = Some(reader.read_u32()?);
    }

    if version >= 18 {
        item.unk6 = Some(reader.read_u32()?);
    }

    if version >= 19 {
        item.unk7 = Some(reader.read_n_bytes(9)?);
    }

    if version >= 20 {
        item.unk8 = Some(reader.read_u8()?);
    }

    if version >= 21 {
        item.unk9 = Some(reader.read_u8()?);
    }

    if version >= 22 {
        item.item_description = Some(reader.read_length_prefixed_string()?);
    }

    if version >= 23 {
        item.item_info = Some(reader.read_u32()?);
    }
    
    Ok(item)
}
