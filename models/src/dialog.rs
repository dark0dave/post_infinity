use std::rc::Rc;

use serde::Serialize;

use crate::common::fixed_char_array::FixedCharSlice;
use crate::model::Model;
use crate::resources::utils::{copy_buff_to_struct, copy_transmute_buff};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm
#[derive(Debug, Serialize)]
pub struct Dialog {
    pub header: DialogHeader,
    pub state_tables: Vec<StateTable>,
    pub transitions: Vec<Transition>,
    pub state_triggers: Vec<StateTrigger>,
    pub transition_triggers: Vec<TransitionTrigger>,
    pub action_tables: Vec<ActionTable>,
}

impl Model for Dialog {
    fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<DialogHeader>(buffer, 0);

        let start = usize::try_from(header.offset_to_state_table).unwrap_or(0);
        let count = usize::try_from(header.count_of_state_tables).unwrap_or(0);
        let state_tables = copy_transmute_buff::<StateTable>(buffer, start, count);

        let start = usize::try_from(header.offset_to_transition_table).unwrap_or(0);
        let count = usize::try_from(header.count_of_transitions).unwrap_or(0);
        let transitions = copy_transmute_buff::<Transition>(buffer, start, count);

        let start = usize::try_from(header.offset_to_state_trigger_table).unwrap_or(0);
        let count = usize::try_from(header.count_of_state_triggers).unwrap_or(0);
        let state_triggers = copy_transmute_buff::<StateTrigger>(buffer, start, count);

        let start = usize::try_from(header.offset_to_transition_trigger_table).unwrap_or(0);
        let count = usize::try_from(header.count_of_transition_triggers).unwrap_or(0);
        let transition_triggers = copy_transmute_buff::<TransitionTrigger>(buffer, start, count);

        let start = usize::try_from(header.offset_to_action_table).unwrap_or(0);
        let count = usize::try_from(header.count_of_action_tables).unwrap_or(0);
        let action_tables = copy_transmute_buff::<ActionTable>(buffer, start, count);

        Self {
            header,
            state_tables,
            transitions,
            state_triggers,
            transition_triggers,
            action_tables,
        }
    }

    fn create_as_box(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Header
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct DialogHeader {
    pub signature: FixedCharSlice<4>,
    pub version: FixedCharSlice<4>,
    pub count_of_state_tables: i32,
    pub offset_to_state_table: i32,
    pub count_of_transitions: i32,
    pub offset_to_transition_table: i32,
    pub offset_to_state_trigger_table: i32,
    pub count_of_state_triggers: i32,
    pub offset_to_transition_trigger_table: i32,
    pub count_of_transition_triggers: i32,
    pub offset_to_action_table: i32,
    pub count_of_action_tables: i32,
    pub flags: FixedCharSlice<4>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_State
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct StateTable {
    pub actor_response_text: FixedCharSlice<4>,
    pub index_of_the_first_transition: u32,
    pub count_of_transitions: u32,
    pub index_of_state_trigger: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Transition
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct Transition {
    pub flags: FixedCharSlice<4>,
    pub player_character_text: FixedCharSlice<4>,
    pub journal_text: FixedCharSlice<4>,
    pub index_of_transitions_trigger: u32,
    pub index_of_transitions_action_table: u32,
    pub resource_name: FixedCharSlice<8>,
    pub index_of_the_next_state: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_StateTrigger
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct StateTrigger {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_TransTrigger
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct TransitionTrigger {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Action
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct ActionTable {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

#[cfg(test)]

mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_dialog() {
        let file = File::open("fixtures/mazzy.dlg").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let dialog = Dialog::new(&buffer);
        assert_eq!(dialog.state_tables.len(), 58);
        assert_eq!(dialog.transitions.len(), 103);
        assert_eq!(dialog.state_triggers.len(), 7);
        assert_eq!(dialog.transition_triggers.len(), 30);
        assert_eq!(dialog.action_tables.len(), 33)
    }
}
