use binrw::{helpers::until_eof, io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::resref::Resref;
use crate::common::strref::Strref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Store {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[serde(flatten)]
    pub header: StoreHeader,
    #[br(count=header.count_of_items_for_sale_section)]
    pub items_for_sale: Vec<ItemsForSale>,
    #[br(count=header.count_of_drinks_section)]
    pub drinks_for_sale: Vec<DrinksForSale>,
    #[br(count=header.count_of_cures_section)]
    pub cures_for_sale: Vec<CuresForSale>,
    #[br(count=header.count_of_items_in_items_purchased_section)]
    pub items_purchased_here: Vec<u32>,
}

impl Model for Store {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct StoreHeader {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
    //  (0=Store, 1=Tavern, 2=Inn, 3=Temple, 5=Container)
    pub store_type: u32,
    pub name: Strref,
    pub flags: u32,
    pub sell_price_markup: u32,
    pub buy_price_markup: u32,
    pub depreciation_rate: u32,
    pub chance_of_steal_failure: u16,
    pub capacity: u16,
    #[serde(skip)]
    #[br(count = 8)]
    _unknown1: Vec<u8>,
    pub offset_to_items_purchased_section: u32,
    pub count_of_items_in_items_purchased_section: u32,
    pub offset_to_items_for_sale_section: u32,
    pub count_of_items_for_sale_section: u32,
    pub lore: u32,
    pub id_price: u32,
    pub rumours_tavern: Resref,
    pub offset_to_drinks_section: u32,
    pub count_of_drinks_section: u32,
    pub rumours_temple: Resref,
    pub room_flags: u32,
    pub price_of_a_peasant_room: u32,
    pub price_of_a_merchant_room: u32,
    pub price_of_a_noble_room: u32,
    pub price_of_a_royal_room: u32,
    pub offset_to_cures_section: u32,
    pub count_of_cures_section: u32,
    #[serde(skip)]
    #[br(count = 36)]
    _unknown2: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Sale
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ItemsForSale {
    pub filename_of_item: Resref,
    pub item_expiration_time: u16,
    pub quantity_charges_1: u16,
    pub quantity_charges_2: u16,
    pub quantity_charges_3: u16,
    pub flags: u32,
    pub amount_of_this_item_in_stock: u32,
    //  (0=limited stock, 1=infinite stock)
    pub infinite_supply_flag: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Drink
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct DrinksForSale {
    pub rumour_resource: Resref,
    pub drink_name: Strref,
    pub drink_price: u32,
    pub alcoholic_strength: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Cure
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct CuresForSale {
    pub filename_of_spell: Resref,
    pub spell_price: u32,
}
