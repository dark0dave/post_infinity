use std::rc::Rc;

use binrw::{
    io::{Cursor, SeekFrom},
    BinRead, BinReaderExt, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::resref::Resref;
use crate::common::strref::Strref;
use crate::model::Model;
use crate::tlk::Lookup;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/dlg_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Dialogue {
    #[serde(flatten)]
    pub header: DialogueHeader,
    #[serde(flatten)]
    #[br(count=header.count_of_state_tables, seek_before=SeekFrom::Start(header.offset_to_state_table as u64))]
    pub state_tables: Vec<StateTable>,
    #[serde(flatten)]
    #[br(count=header.count_of_transitions, seek_before=SeekFrom::Start(header.offset_to_transition_table as u64))]
    pub transitions: Vec<Transition>,
    #[serde(flatten)]
    #[br(count=header.count_of_state_triggers, seek_before=SeekFrom::Start(header.offset_to_state_trigger_table as u64))]
    pub state_triggers: Vec<StateTrigger>,
    #[serde(flatten)]
    #[br(count=header.count_of_transition_triggers, seek_before=SeekFrom::Start(header.offset_to_transition_trigger_table as u64))]
    pub transition_triggers: Vec<TransitionTrigger>,
    #[serde(flatten)]
    #[br(count=header.count_of_action_tables, seek_before=SeekFrom::Start(header.offset_to_action_table as u64))]
    pub action_tables: Vec<ActionTable>,
}

impl Model for Dialogue {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
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
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub version: String,
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

        let dialog = Dialogue::new(&buffer);
        assert_eq!(dialog.state_tables.len(), 58);
        assert_eq!(dialog.transitions.len(), 103);
        assert_eq!(dialog.state_triggers.len(), 7);
        assert_eq!(dialog.transition_triggers.len(), 30);
        assert_eq!(dialog.action_tables.len(), 33)
    }
}
