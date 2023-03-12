use std::rc::Rc;

use serde::Serialize;

use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};
use crate::{
    common::{fixed_char_array::FixedCharSlice, header::Header},
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm
#[derive(Debug, Serialize)]
pub struct Store {
    pub store_header: StoreHeader,
    pub items_for_sale: Vec<ItemsForSale>,
    pub drinks_for_sale: Vec<DrinksForSale>,
    pub cures_for_sale: Vec<CuresForSale>,
    pub items_purchased_here: Vec<ItemsPurchasedHere>,
}

impl Model for Store {
    fn new(buffer: &[u8]) -> Self {
        let store_header = copy_buff_to_struct::<StoreHeader>(buffer, 0);

        let start = usize::try_from(store_header.offset_to_items_for_sale_section).unwrap_or(0);
        let count = usize::try_from(store_header.count_of_items_for_sale_section).unwrap_or(0);
        let items_for_sale = copy_transmute_buff::<ItemsForSale>(buffer, start, count);

        let start = usize::try_from(store_header.offset_to_drinks_section).unwrap_or(0);
        let count = usize::try_from(store_header.count_of_drinks_section).unwrap_or(0);
        let drinks_for_sale = copy_transmute_buff::<DrinksForSale>(buffer, start, count);

        let start = usize::try_from(store_header.offset_to_cures_section).unwrap_or(0);
        let count = usize::try_from(store_header.count_of_cures_section).unwrap_or(0);
        let cures_for_sale = copy_transmute_buff::<CuresForSale>(buffer, start, count);

        let start = usize::try_from(store_header.offset_to_items_purchased_section).unwrap_or(0);
        let count =
            usize::try_from(store_header.count_of_items_in_items_purchased_section).unwrap_or(0);
        let items_purchased_here = copy_transmute_buff::<ItemsPurchasedHere>(buffer, start, count);
        Self {
            store_header,
            items_for_sale,
            drinks_for_sale,
            cures_for_sale,
            items_purchased_here,
        }
    }
    fn create_as_box(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct StoreHeader {
    pub header: Header,
    //  (0=Store, 1=Tavern, 2=Inn, 3=Temple, 5=Container)
    pub store_type: i32,
    pub name: FixedCharSlice<4>,
    pub flags: i32,
    pub sell_price_markup: i32,
    pub buy_price_markup: i32,
    pub depreciation_rate: i32,
    pub chance_of_steal_failure: i16,
    pub capacity: i16,
    _unknown1: [i8; 8],
    pub offset_to_items_purchased_section: i32,
    pub count_of_items_in_items_purchased_section: i32,
    pub offset_to_items_for_sale_section: i32,
    pub count_of_items_for_sale_section: i32,
    pub lore: i32,
    pub id_price: i32,
    pub rumours_tavern: [i8; 8],
    pub offset_to_drinks_section: i32,
    pub count_of_drinks_section: i32,
    pub rumours_temple: [i8; 8],
    pub room_flags: i32,
    pub price_of_a_peasant_room: i32,
    pub price_of_a_merchant_room: i32,
    pub price_of_a_noble_room: i32,
    pub price_of_a_royal_room: i32,
    pub offset_to_cures_section: i32,
    pub count_of_cures_section: i32,
    _unknown2: [i8; 32],
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Sale
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct ItemsForSale {
    pub filename_of_item: [i8; 8],
    pub item_expiration_time: i16,
    pub quantity_charges_1: i16,
    pub quantity_charges_2: i16,
    pub quantity_charges_3: i16,
    pub flags: i32,
    pub amount_of_this_item_in_stock: i32,
    //  (0=limited stock, 1=infinite stock)
    pub infinite_supply_flag: i32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Drink
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct DrinksForSale {
    pub rumour_resource: [i8; 8],
    pub drink_name: FixedCharSlice<4>,
    pub drink_price: i32,
    pub alcoholic_strength: i32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sto_v1.htm#storv1_0_Cure
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct CuresForSale {
    pub filename_of_spell: [i8; 8],
    pub spell_price: i32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct ItemsPurchasedHere(i32);
