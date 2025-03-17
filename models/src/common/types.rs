// #![allow(dead_code, unused_variables)]

use std::{error::Error, path::Path};

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

// https://gibberlings3.github.io/iesdp/file_formats/general.htm#FileFormats
#[derive(Debug, Copy, Clone, Hash, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
#[brw(little, repr = u16)]
#[repr(u16)]
pub enum ResourceType {
    NotFound = 0x0000,
    FileTypeBmp = 0x0001,
    FileTypeMve = 0x0002,
    FileTypeWav = 0x0004,
    FileTypeWfx = 0x0005,
    FileTypePlt = 0x0006,
    FileTypeBam = 0x03e8,
    FileTypeWed = 0x03e9,
    FileTypeChu = 0x03ea,
    FileTypeTi = 0x03eb,
    FileTypeMos = 0x03ec,
    FileTypeItm = 0x03ed,
    FileTypeSpl = 0x03ee,
    FileTypeBcs = 0x03ef,
    FileTypeIds = 0x03f0,
    FileTypeCre = 0x03f1,
    FileTypeAre = 0x03f2,
    FileTypeDlg = 0x03f3,
    FileType2da = 0x03f4,
    FileTypeGam = 0x03f5,
    FileTypeSto = 0x03f6,
    FileTypeWmap = 0x03f7,
    //FileTypeChr1 = 0x03f8,
    FileTypeEff = 0x03f8,
    FileTypeBs = 0x03f9,
    FileTypeChr = 0x03fa,
    FileTypeVvc = 0x03fb,
    FileTypeVef = 0x03fc,
    FileTypePro = 0x03fd,
    FileTypeBio = 0x03fe,
    FileTypeWbm = 0x03ff,
    FileTypeFnt = 0x0400,
    FileTypeGui = 0x0402,
    FileTypeSql = 0x0403,
    FileTypePvrz = 0x0404,
    FileTypeGlsl = 0x0405,
    FileTypeTlk = 0x0407,
    FileTypeMenu = 0x0408,
    FileTypeMenu2 = 0x0409,
    FileTypeTtf = 0x040a,
    FileTypePng = 0x040b,
    FileTypeBah = 0x044c,
    FileTypeIni = 0x0802,
    FileTypeSrc = 0x0803,
    // Here our invented FileTypes:
    FileTypeKey = 0x1000,
    FileTypeBiff = 0x1001,
    FileTypeSave = 0x1002,
}

impl TryFrom<&Path> for ResourceType {
    type Error = Box<dyn Error>;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let extension = value.extension().ok_or("Path has no extension")?;
        Ok(Self::from(
            extension.to_str().ok_or("Could not convert to string")?,
        ))
    }
}

impl From<&str> for ResourceType {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "bmp" => ResourceType::FileTypeBmp,
            "mve" => ResourceType::FileTypeMve,
            "wav" => ResourceType::FileTypeWav,
            "wfx" => ResourceType::FileTypeWfx,
            "plt" => ResourceType::FileTypePlt,
            "bam" => ResourceType::FileTypeBam,
            "wed" => ResourceType::FileTypeWed,
            "chu" => ResourceType::FileTypeChu,
            "ti" => ResourceType::FileTypeTi,
            "mos" => ResourceType::FileTypeMos,
            "itm" => ResourceType::FileTypeItm,
            "spl" => ResourceType::FileTypeSpl,
            "bcs" => ResourceType::FileTypeBcs,
            "ids" => ResourceType::FileTypeIds,
            "cre" => ResourceType::FileTypeCre,
            "are" => ResourceType::FileTypeAre,
            "dlg" => ResourceType::FileTypeDlg,
            "2da" => ResourceType::FileType2da,
            "gam" => ResourceType::FileTypeGam,
            "sto" => ResourceType::FileTypeSto,
            "wmp" => ResourceType::FileTypeWmap,
            "eff" => ResourceType::FileTypeEff,
            "bs" => ResourceType::FileTypeBs,
            "chr" => ResourceType::FileTypeChr,
            "vvc" => ResourceType::FileTypeVvc,
            "vef" => ResourceType::FileTypeVef,
            "pro" => ResourceType::FileTypePro,
            "bio" => ResourceType::FileTypeBio,
            "wbm" => ResourceType::FileTypeWbm,
            "fnt" => ResourceType::FileTypeFnt,
            "gui" => ResourceType::FileTypeGui,
            "sql" => ResourceType::FileTypeSql,
            "pvrz" => ResourceType::FileTypePvrz,
            "glsl" => ResourceType::FileTypeGlsl,
            "tlk" => ResourceType::FileTypeTlk,
            "menu" => ResourceType::FileTypeMenu,
            "ttf" => ResourceType::FileTypeTtf,
            "png" => ResourceType::FileTypePng,
            "bah" => ResourceType::FileTypeBah,
            "ini" => ResourceType::FileTypeIni,
            "src" => ResourceType::FileTypeSrc,
            // Here our invented FileTypes:
            "key" => ResourceType::FileTypeKey,
            "bif" => ResourceType::FileTypeBiff,
            "sav" => ResourceType::FileTypeSave,
            _ => ResourceType::NotFound,
        }
    }
}
