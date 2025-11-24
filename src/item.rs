#[derive(Debug)]
pub struct ItemsDat {
    pub version: i16,
    pub item_count: i32,
    pub items: Vec<Item>,
}

/// All names took from [Growtools] as reference.
/// 
/// [Growtools]: https://github.com/xSkriptx/Growtools/blob/5d1aaf76c93e82141a06b74968038daa1a7ed812/pages/dat-decoder.html#L711
#[derive(Debug, Default)]
pub struct Item {
    pub id: u32,
    pub properties: u16,
    pub item_type: u8,
    pub material: u8,
    pub name: String,
    pub file_name: String,
    pub file_hash: u32,
    pub visual_type: u8,
    pub cook_time: u32,
    pub tex_x: u8,
    pub tex_y: u8,
    pub storage_type: u8,
    pub layer: u8,
    pub collision_type: u8,
    pub hardness: u8,
    pub regen_time: u32,
    pub clothing_type: u8,
    pub rarity: u16,
    pub max_hold: u8,
    pub alt_file_path: String,
    pub alt_file_hash: u32,
    pub anim_ms: u32,

    pub pet_name: Option<String>,
    pub pet_prefix: Option<String>,
    pub pet_suffix: Option<String>,

    pub pet_ability: Option<String>,
    
    pub seed_base: u8,
    pub seed_over: u8,
    pub tree_base: u8,
    pub tree_over: u8,
    pub bg_col: u32,
    pub fg_col: u32,
    pub seed1: u16,
    pub seed2: u16,
    pub bloom_time: u32,

    pub anim_type: Option<u32>,
    pub anim_string: Option<String>,

    pub anim_tex: Option<String>,
    pub anim_string2: Option<String>,
    pub dlayer1: Option<u32>,
    pub dlayer2: Option<u32>,

    pub properties2: Option<u16>,
    pub unk: Option<Vec<u8>>,

    pub tile_range: Option<u32>,
    pub pile_range: Option<u32>,

    pub custom_punch: Option<String>,

    pub unk2: Option<Vec<u8>>,

    pub clock_div: Option<u32>,

    pub parent_id: Option<u32>,

    pub unk3: Option<Vec<u8>>,
    pub alt_sit_path: Option<String>,

    pub unk4: Option<String>,

    pub unk5: Option<u32>,

    pub unk6: Option<u32>,

    pub unk7: Option<Vec<u8>>,

    pub unk8: Option<u8>,

    pub unk9: Option<u8>,

    pub item_description: Option<String>,

    pub item_info: Option<u32>,
}
