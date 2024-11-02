use std::path::Path;
use std::rc::Rc;

use binrw::{helpers::until_eof, io::Read, io::Seek, BinRead, BinReaderExt, BinWrite};
use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{char_array::CharArray, types::ResourceType},
    from_buffer,
    model::Model,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Save {
    #[serde(skip)]
    #[br(parse_with = until_eof, restore_position)]
    pub original_bytes: Vec<u8>,
    #[br(count = 4)]
    signature: CharArray,
    #[br(count = 4)]
    version: CharArray,
    #[br(parse_with=binrw::helpers::until_eof)]
    pub files: Vec<SavedFile>,
}

impl Save {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Self {
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}", err);
            }
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm#savv1_File
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct SavedFile {
    pub length_of_filename: u32,
    #[br(count=length_of_filename)]
    pub filename: CharArray,
    pub uncompressed_data_length: u32,
    pub compressed_data_length: u32,
    #[br(count=compressed_data_length, restore_position)]
    pub compressed_data: Vec<u8>,
    #[br(count=compressed_data_length, map = |s: Vec<u8>| parse_compressed_data(&s, &filename))]
    #[bw(ignore)]
    #[serde(skip)]
    pub uncompressed_data: Option<Rc<dyn Model>>,
}

fn parse_compressed_data(buff: &[u8], file_name: &CharArray) -> Option<Rc<dyn Model>> {
    let mut d = ZlibDecoder::new(buff);
    let mut buffer = vec![];
    match d.read_to_end(&mut buffer) {
        Ok(_) => {
            let extension = Path::new(&file_name.to_string())
                .extension()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .into_string()
                .unwrap_or_default()
                .replace('\0', "");
            let resource_type = ResourceType::from(extension.as_str());
            from_buffer(&buffer, resource_type)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs::File;

    use super::*;
    use crate::area::Area;
    use binrw::io::BufReader;
    use pretty_assertions::assert_eq;

    #[test]
    fn uncompress_files() {
        let file = File::open("fixtures/BALDUR.SAV").unwrap();
        let mut reader = BufReader::new(file);
        let save = Save::new(&mut reader);
        let model = save.files[0].uncompressed_data.clone().unwrap();
        let ptr = Rc::into_raw(model) as *const Area;
        let area: Area = unsafe { ptr.read() };
        assert_eq!(area.header.last_saved, 47888);
        assert_eq!(area.actors.len(), 5);
        assert_eq!(area.ambients.len(), 2);
    }

    #[test]
    fn read_save() {
        let file = File::open("fixtures/BALDUR.SAV").unwrap();
        let mut reader = BufReader::new(file);
        let save = Save::new(&mut reader);
        assert_eq!(save.files[0].compressed_data_length, 1316);
        assert_eq!(save.files[0].uncompressed_data_length, 9395);
        let file_names = vec![
            "AR0011.are",
            "AR0012.are",
            "AR0013.are",
            "AR0020.are",
            "AR0021.are",
            "AR0022.are",
            "AR0028.are",
            "AR0069.are",
            "AR0070.are",
            "AR0071.are",
            "AR0072.are",
            "AR0084.are",
            "AR0086.are",
            "AR0201.are",
            "AR0202.are",
            "AR0203.are",
            "AR0204.are",
            "AR0205.are",
            "AR0206.are",
            "AR0300.are",
            "AR0301.are",
            "AR0302.are",
            "AR0303.are",
            "AR0304.are",
            "AR0305.are",
            "AR0306.are",
            "AR0307.are",
            "AR0308.are",
            "AR0309.are",
            "AR0310.are",
            "AR0311.are",
            "AR0313.are",
            "AR0314.are",
            "AR0319.are",
            "AR0325.are",
            "AR0326.are",
            "AR0332.are",
            "AR0333.are",
            "AR0334.are",
            "AR0400.are",
            "AR0401.are",
            "AR0402.are",
            "AR0403.are",
            "AR0404.are",
            "AR0405.are",
            "AR0406.are",
            "AR0407.are",
            "AR0409.are",
            "AR0410.are",
            "AR0411.are",
            "AR0412.are",
            "AR0413.are",
            "AR0414.are",
            "AR0415.are",
            "AR0416.are",
            "AR0419.are",
            "AR0420.are",
            "AR0500.are",
            "AR0501.are",
            "AR0502.are",
            "AR0503.are",
            "AR0507.are",
            "AR0508.are",
            "AR0509.are",
            "AR0510.are",
            "AR0511.are",
            "AR0513.are",
            "AR0514.are",
            "AR0516.are",
            "AR0517.are",
            "AR0518.are",
            "AR0520.are",
            "AR0522.are",
            "AR0523.are",
            "AR0600.are",
            "AR0601.are",
            "AR0602.are",
            "AR0603.are",
            "AR0604.are",
            "AR0605.are",
            "AR0606.are",
            "AR0607.are",
            "AR0700.are",
            "AR0701.are",
            "AR0702.are",
            "AR0704.are",
            "AR0705.are",
            "AR0711.are",
            "AR0800.are",
            "AR0801.are",
            "AR0803.are",
            "AR0804.are",
            "AR0805.are",
            "AR0806.are",
            "AR0808.are",
            "AR0809.are",
            "AR0810.are",
            "AR0812.are",
            "AR0813.are",
            "AR0900.are",
            "AR0902.are",
            "AR0903.are",
            "AR0904.are",
            "AR0906.are",
            "AR0907.are",
            "AR1000.are",
            "AR1001.are",
            "AR1002.are",
            "AR1006.are",
            "AR1009.are",
            "AR1100.are",
            "AR1101.are",
            "AR1102.are",
            "AR1103.are",
            "AR1104.are",
            "AR1105.are",
            "AR1106.are",
            "AR1200.are",
            "AR1201.are",
            "AR1202.are",
            "AR1203.are",
            "AR1204.are",
            "AR1300.are",
            "AR1301.are",
            "AR1302.are",
            "AR1303.are",
            "AR1304.are",
            "AR1306.are",
            "AR1307.are",
            "AR1400.are",
            "AR1401.are",
            "AR1402.are",
            "AR1403.are",
            "AR1404.are",
            "AR1500.are",
            "AR1503.are",
            "AR1507.are",
            "AR1508.are",
            "AR1510.are",
            "AR1511.are",
            "AR1512.are",
            "AR1513.are",
            "AR1514.are",
            "AR1515.are",
            "AR1516.are",
            "AR1600.are",
            "AR1601.are",
            "AR1602.are",
            "AR1603.are",
            "AR1604.are",
            "AR1605.are",
            "AR1606.are",
            "AR1607.are",
            "AR1608.are",
            "AR1609.are",
            "AR1610.are",
            "AR1611.are",
            "AR1612.are",
            "AR1613.are",
            "AR1900.are",
            "AR1901.are",
            "AR1902.are",
            "AR1904.are",
            "AR1905.are",
            "AR2000.are",
            "AR2006.are",
            "AR2007.are",
            "AR2008.are",
            "AR2009.are",
            "AR2011.are",
            "AR2012.are",
            "AR2013.are",
            "AR2014.are",
            "AR2015.are",
            "AR2016.are",
            "AR2017.are",
            "AR2100.are",
            "AR2101.are",
            "AR2102.are",
            "AR2200.are",
            "AR2201.are",
            "AR2202.are",
            "AR2203.are",
            "AR2204.are",
            "AR2205.are",
            "AR2206.are",
            "AR2207.are",
            "AR2208.are",
            "AR2210.are",
            "AR2300.are",
            "AR2400.are",
            "AR2401.are",
            "AR2402.are",
            "AR2500.are",
            "AR3000.are",
            "AR3001.are",
            "AR3003.are",
            "AR3005.are",
            "AR3009.are",
            "AR3010.are",
            "AR3011.are",
            "AR3016.are",
            "BAG01.sto",
            "BAG02.sto",
            "BAG03B.sto",
            "BAG03D.sto",
            "BAG04.sto",
            "BAG05.sto",
            "BAG06B.sto",
            "BERNARD.sto",
            "BERNARD2.sto",
            "BINNKEEP.sto",
            "BMTHIEF.sto",
            "BSHOP02.sto",
            "CDAEMERC.sto",
            "CDBAG04.sto",
            "CROBAR01.sto",
            "DEFAULT.toh",
            "DMARK.sto",
            "DOGHMA.sto",
            "DSHOP01.sto",
            "DSHOP02.sto",
            "FFBART.sto",
            "GARLENA.sto",
            "GORCH.sto",
            "JAYES.sto",
            "KPCHAP01.sto",
            "MERCHANT.sto",
            "OH4000.are",
            "OH4010.are",
            "OH4100.are",
            "OH4101.are",
            "OH6000.are",
            "OH6010.are",
            "OH6100.are",
            "OH6200.are",
            "OH6300.are",
            "OHNGDUKE.sto",
            "OHNPOTME.sto",
            "OHNSCRME.sto",
            "OHNWANME.sto",
            "PPINN01.sto",
            "PPSTOR01.sto",
            "PPUMB01.sto",
            "RIBALD.sto",
            "RIBALD3.sto",
            "ROGER.sto",
            "RR#001.are",
            "RR#MARIN.sto",
            "SAHPR1.sto",
            "SCROLLS.sto",
            "SHOP03.sto",
            "SHOP06.sto",
            "SHOP07.sto",
            "SHOP08.sto",
            "SLSHOP01.sto",
            "SLSHOP02.sto",
            "TANALLY1.sto",
            "TEMSUP.sto",
            "TEMTALOS.sto",
            "THUMB.sto",
            "TRCAR04.sto",
            "TRMER01.sto",
            "TRMER02.sto",
            "UDDROW22.sto",
            "UDDROW23.sto",
            "UDDROW24.sto",
            "UDDROW25.sto",
            "UDDUER01.sto",
            "UDSVIR04.sto",
            "UDSVIR05.sto",
            "UHINN01.sto",
            "UHMER01.sto",
            "UHMER02.sto",
            "UHMER03.sto",
            "WALLACE.sto",
            "WINNKEEP.sto",
            "WMART1.sto",
            "WMART2.sto",
            "WORLDMAP.wmp",
        ];
        for (i, file) in save.files.iter().enumerate() {
            assert_eq!(
                file.compressed_data.len(),
                file.compressed_data_length as usize
            );
            assert_eq!(file.filename.to_string().replace("\0", ""), file_names[i]);
        }
    }
}
