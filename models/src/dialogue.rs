use binrw::{helpers::until_eof, io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::resref::Resref;
use crate::common::strref::Strref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Dialogue {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[bw(ignore)]
    #[serde(flatten)]
    pub header: DialogueHeader,
    #[bw(ignore)]
    #[br(count=header.count_of_state_tables)]
    pub state_tables: Vec<StateTable>,
    #[bw(ignore)]
    #[br(count=header.count_of_transitions)]
    pub transitions: Vec<Transition>,
    #[bw(ignore)]
    #[br(count=header.count_of_state_triggers)]
    pub state_triggers: Vec<StateTrigger>,
    #[bw(ignore)]
    #[br(count=header.count_of_transition_triggers)]
    pub transition_triggers: Vec<TransitionTrigger>,
    #[bw(ignore)]
    #[br(count=header.count_of_action_tables)]
    pub action_tables: Vec<ActionTable>,
}

impl Model for Dialogue {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Header
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct DialogueHeader {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
    pub count_of_state_tables: u32,
    pub offset_to_state_table: u32,
    pub count_of_transitions: u32,
    pub offset_to_transition_table: u32,
    pub offset_to_state_trigger_table: u32,
    pub count_of_state_triggers: u32,
    pub offset_to_transition_trigger_table: u32,
    pub count_of_transition_triggers: u32,
    pub offset_to_action_table: u32,
    pub count_of_action_tables: u32,
    #[br(count = 4)]
    pub flags: Vec<u8>,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_State
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct StateTable {
    pub actor_response_text: Strref,
    pub index_of_the_first_transition: u32,
    pub count_of_transitions: u32,
    pub index_of_state_trigger: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Transition
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Transition {
    #[br(count = 4)]
    pub flags: Vec<u8>,
    pub player_character_text: Strref,
    pub journal_text: Strref,
    pub index_of_transitions_trigger: u32,
    pub index_of_transitions_action_table: u32,
    pub resource_name: Resref,
    pub index_of_the_next_state: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_StateTrigger
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct StateTrigger {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_TransTrigger
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TransitionTrigger {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm#formDLGV1_Action
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct ActionTable {
    pub offset_to_start_of_file: u32,
    pub length_in_bytes: u32,
}

#[cfg(test)]

mod tests {

    use super::*;
    use binrw::io::{BufReader, Read};
    use std::fs::File;

    #[test]
    fn valid_dialog() {
        let file = File::open("fixtures/mazzy.dlg").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let dialog = Dialogue::new(&buffer);
        assert_eq!(dialog.state_tables.len(), 58);
        assert_eq!(dialog.transitions.len(), 103);
        assert_eq!(dialog.state_triggers.len(), 7);
        assert_eq!(dialog.transition_triggers.len(), 30);
        assert_eq!(dialog.action_tables.len(), 33)
    }
}
