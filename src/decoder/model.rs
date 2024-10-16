use std::io::{Read, Seek, SeekFrom};

use json::{object, JsonValue};
use serde_derive::Serialize;
use struct_iterable::Iterable;
use ts_rs::TS;

use super::{from_c_struct::FromCStruct, to_structure::ToStructure};

// region: Structs

/// A Struct that is used for storing the information of a structure
/// within a document for the logical analysis
#[derive(Debug, TS)]
#[ts(export)]
pub struct Structure {
    pub name: String,
    pub structure: Vec<StructureItem>,
    pub substructs: Option<Vec<Structure>>,
}

/// A Struct that is used for storing the information of a field within a structure
#[derive(Debug, TS)]
pub struct StructureItem {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

impl Structure {
    pub fn from<T: ToStructure>(name: &str, item: &T) -> Self {
        Structure {
            name: name.to_string(),
            structure: item.structure_items(),
            substructs: item.substructures(),
        }
    }
}

impl From<&Structure> for JsonValue {
    fn from(structure: &Structure) -> JsonValue {
        let mut items_structure = vec![];
        for item in &structure.structure {
            let mut item_json = object! {};
            item_json["name"] = item.name.clone().into();
            item_json["value"] = item.value.clone().into();
            if let Some(description) = &item.description {
                item_json["description"] = description.clone().into();
            }
            items_structure.push(item_json);
        }

        let substructs = if let Some(substructs) = &structure.substructs {
            let mut substructs_structure: Vec<&Structure> = vec![];
            for substruct in substructs {
                substructs_structure.push(substruct.into());
            }
            Some(substructs_structure)
        } else {
            None
        };

        object! {
            name: structure.name.clone(),
            structure: items_structure,
            substructs: substructs,
        }
    }
}

impl Into<JsonValue> for Structure {
    fn into(self) -> JsonValue {
        JsonValue::from(&self)
    }
}

/// storing the physical bytes of a certain section of the document
#[derive(Debug, TS)]
#[ts(export)]
pub struct PhysicalStructure {
    pub stream_name: String,
    pub structure_name: Option<String>,
    // the below will be made into a string for TS
    pub bytes: Vec<u8>,
    pub start_index: u64,
    pub end_index: u64,
    pub description: Option<String>,
}

impl PhysicalStructure {
    pub fn from_reader_range<T: Read + Seek>(
        reader: &mut T,
        start: u64,
        end: u64,
        stream_name: &str,
    ) -> Self {
        let mut bytes = vec![];
        reader.seek(SeekFrom::Start(start)).unwrap();
        reader.take(end - start).read_to_end(&mut bytes).unwrap();

        PhysicalStructure {
            stream_name: stream_name.to_string(),
            bytes,
            start_index: start,
            end_index: end,
            description: None,
            structure_name: None,
        }
    }

    pub fn structure_name(mut self, name: &str) -> Self {
        self.structure_name = Some(name.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

impl From<&PhysicalStructure> for JsonValue {
    fn from(value: &PhysicalStructure) -> Self {
        let hex_string = hex::encode_upper(&value.bytes);

        object! {
            stream_name: value.stream_name.clone(),
            structure_name: value.structure_name.clone(),
            bytes: hex_string,
            start_index: value.start_index,
            end_index: value.end_index,
            description: value.description.clone(),
        }
    }
}

impl Into<JsonValue> for PhysicalStructure {
    fn into(self) -> JsonValue {
        JsonValue::from(&self)
    }
}

#[derive(Debug, TS)]
#[ts(export)]
pub struct ComparisonPhysicalStructure<'a, 'b> {
    pub ref_structure: &'b PhysicalStructure,
    pub comp_structure: &'a PhysicalStructure,
    pub difference_indices: Vec<(usize, usize)>,
}

impl<'a, 'b> From<&ComparisonPhysicalStructure<'a, 'b>> for JsonValue {
    fn from(value: &ComparisonPhysicalStructure) -> Self {
        let mut difference_indices: Vec<JsonValue> = vec![];
        for (ref_index, comp_index) in &value.difference_indices {
            difference_indices.push(vec![*ref_index, *comp_index].into());
        }

        object! {
            ref_structure: JsonValue::from(value.ref_structure),
            comp_structure: JsonValue::from(value.comp_structure),
            difference_indices: difference_indices,
        }
    }
}

impl<'a, 'b> Into<JsonValue> for ComparisonPhysicalStructure<'a, 'b> {
    fn into(self) -> JsonValue {
        JsonValue::from(&self)
    }
}

#[derive(Debug, TS)]
#[ts(export)]
pub struct ComparisonLogicalStructure<'a, 'b> {
    pub ref_structure: Option<&'a Structure>,
    pub comp_structure: Option<&'b Structure>,
    pub structure_differences: Vec<bool>,
    pub substructure_differences: Vec<ComparisonLogicalStructure<'a, 'b>>,
}

impl<'a, 'b> From<&ComparisonLogicalStructure<'a, 'b>> for JsonValue {
    fn from(value: &ComparisonLogicalStructure) -> Self {
        let mut structure_differences: Vec<JsonValue> = vec![];
        for difference in &value.structure_differences {
            structure_differences.push(difference.clone().into());
        }

        let mut substructure_differences: Vec<JsonValue> = vec![];
        for substructure in &value.substructure_differences {
            substructure_differences.push(substructure.into());
        }

        object! {
            ref_structure: JsonValue::from(value.ref_structure),
            comp_structure: JsonValue::from(value.comp_structure),
            structure_differences: structure_differences,
            substructure_differences: substructure_differences,
        }
    }
}

impl<'a, 'b> Into<JsonValue> for ComparisonLogicalStructure<'a, 'b> {
    fn into(self) -> JsonValue {
        JsonValue::from(&self)
    }
}

#[allow(non_snake_case)]
/// Style sheet information strucure
pub struct _STSHI {
    pub cstd: u16,
    pub cbSTDBaseInFile: u16,
    pub fStdStylenamesWritten: u16,
    pub stiMaxWhenSaved: u16,
    pub istdMaxFixedWhenSaved: u16,
    pub nVerBuiltInNamesWhenSaved: u16,
    pub rgftcStandardChpStsh: [u16; 0x0A], // 10
    pub cbLSD: u16,
    pub mpstilsd: [u16; 0x0A], // 10
}

pub struct Bytes {
    pub bytes: Vec<u8>,
}

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Bytes { bytes }
    }

    pub fn from_u16(value: u16) -> Self {
        Bytes {
            bytes: value.to_le_bytes().to_vec(),
        }
    }

    pub fn from_i32(value: i32) -> Self {
        Bytes {
            bytes: value.to_le_bytes().to_vec(),
        }
    }

    pub fn from_u32(value: u32) -> Self {
        Bytes {
            bytes: value.to_le_bytes().to_vec(),
        }
    }
}

// auto fmt as hex::encode
impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode_upper(&self.bytes))
    }
}

// impl Serialzie for Bytes
impl serde::Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("0x{}", hex::encode_upper(&self.bytes)).serialize(serializer)
    }
}


#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct Fib {
    pub wIdent: Bytes,
    pub nFib: u16,
    pub nProduct: u16,
    pub Lid: u16,
    pub pnNext: i16,
    pub fDot: bool,
    pub fGlsy: bool,
    pub fComplex: bool,
    pub fHasPic: bool,
    pub cQuickSaves: u8, // more like a u4
    pub fEncrypted: bool,
    pub fWhichTblStm: bool,
    pub fReadOnlyRecommended: bool,
    pub fWriteReservation: bool,
    pub fExtChar: bool,
    pub fLoadOverride: bool,
    pub fFarEast: bool,
    pub fCrypto: bool,
    pub nFibBack: u16,
    pub lKey: Bytes,
    pub Envr: u8,
    pub fMac: bool,
    pub fEmptySpecial: bool,
    pub fLoadOverridePage: bool,
    pub fFutureSavedUndo: bool,
    pub fWord97Saved: bool,
    pub fSpare0: [bool; 3],
    pub Chs: u16,
    pub chsTables: u16,
    pub fcMin: i32,
    pub fcMac: i32,
    pub Csw: u16,
    /* Beginning of the array of shorts */
    pub wMagicCreated: Bytes,
    pub wMagicRevised: Bytes,
    pub wMagicCreatedPrivate: Bytes,
    pub wMagicRevisedPrivate: Bytes,
    pub pnfbpChpFirst_W6: i16,
    pub pnChpFirst_W6: i16,
    pub cpnBteChp_W6: i16,
    pub pnFbpPapFirst_W6: i16,
    pub pnPapFirst_W6: i16,
    pub cpnBtePap_W6: i16,
    pub pnFbpLvcFirst_W6: i16,
    pub pnLvcFirst_W6: i16,
    pub cpnBteLvc_W6: i16,
    pub lidFE: i16,
    pub Clw: u16,
    /* Beginning of array of longs */
    pub cbMac: i32,
    pub lProductCreated: Bytes,
    pub lProductRevised: Bytes,
    /* Lengths for the approprate Text Sections */
    pub ccpText: i32,
    pub ccpFtn: i32,
    pub ccpHdr: i32,
    pub ccpMcr: i32,
    pub ccpAtn: i32,
    pub ccpEdn: i32,
    pub ccpTxbx: i32,
    pub ccpHrdTxbx: i32,
    pub pnFbpChpFirst: i32,
    pub pnChpFirst: i32,
    pub cpnBteChp: i32,
    pub pnFbpPapFirst: i32,
    pub pnPapFirst: i32,
    pub cpnBtePap: i32,
    pub pnFbpLvcFirst: i32,
    pub pnLvcFirst: i32,
    pub cpnBteLvc: i32,
    pub fcIslandFirst: i32,
    pub fcIslandLim: i32,
    pub Cfclcb: u16,
    /* Begining of FC/LCB pairs */
    pub fcStshfOrig: i32,
    pub lcbStshfOrig: u32,
    pub fcStshf: i32,
    /// Count of bytes of the STSH allocation
    pub lcbStshf: u32,
    pub fcPlcffndRef: i32,
    pub lcbPlcffndRef: u32,
    pub fcPlcffndTxt: i32,
    pub lcbPlcffndTxt: u32,
    pub fcPlcfandRef: i32,
    pub lcbPlcfandRef: u32,
    pub fcPlcfandTxt: i32,
    pub lcbPlcfandTxt: u32,
    pub fcPlcfsed: i32,
    pub lcbPlcfsed: u32,
    pub fcPlcpad: i32,
    pub lcbPlcpad: u32,
    pub fcPlcfphe: i32,
    pub lcbPlcfphe: u32,
    pub fcSttbfglsy: i32,
    pub lcbSttbfglsy: u32,
    pub fcPlcfglsy: i32,
    pub lcbPlcfglsy: u32,
    pub fcPlcfhdd: i32,
    pub lcbPlcfhdd: u32,
    pub fcPlcfbteChpx: i32,
    pub lcbPlcfbteChpx: u32,
    /// File offset of the STSH (Stylesheet) in the table stream
    pub fcPlcfbtePapx: i32,
    pub lcbPlcfbtePapx: u32,
    pub fcPlcfsea: i32,
    pub lcbPlcfsea: u32,
    pub fcsttbfffn: i32,
    pub lcbsttbfffn: u32,
    pub fcPlcffldMom: i32,
    pub lcbPlcffldMom: u32,
    pub fcPlcffldHdr: i32,
    pub lcbPlcffldHdr: u32,
    pub fcPlcffldFtn: i32,
    pub lcbPlcffldFtn: u32,
    pub fcPlcffldAtn: i32,
    pub lcbPlcffldAtn: u32,
    pub fcPlcffldMcr: i32,
    pub lcbPlcffldMcr: u32,
    pub fcSttbfbkmk: i32,
    pub lcbSttbfbkmk: u32,
    pub fcPlcfbkf: i32,
    pub lcbPlcfbkf: u32,
    pub fcPlcfbkl: i32,
    pub lcbPlcfbkl: u32,
    pub fcCmds: i32,
    pub lcbCmds: u32,
    pub fcPlcmcr: i32,
    pub lcbPlcmcr: u32,
    pub fcSttbfmcr: i32,
    pub lcbSttbfmcr: u32,
    pub fcPrDrvr: i32,
    pub lcbPrDrvr: u32,
    pub fcPrEnvPort: i32,
    pub lcbPrEnvPort: u32,
    pub fcPrEnvLand: i32,
    pub lcbPrEnvLand: u32,
    pub fcWss: i32,
    pub lcbWss: u32,
    pub fcDop: i32,
    pub lcbDop: u32,
    pub fcSttbfAssoc: i32,
    pub lcbSttbfAssoc: u32,
    /// Offset in Table Stream of Complex file text portion
    pub fcClx: i32,
    /// Length of the fcClx
    pub lcbClx: i32,
    /// Offset in Table Stream of list formation information
    pub fcPlcfpgdFtn: i32,
    pub lcbPlcfpgdFtn: u32,
    pub fcAutosaveSource: i32,
    pub lcbAutosaveSource: u32,
    pub fcGrpXstAtnOwners: i32,
    pub lcbGrpXstAtnOwners: u32,
    pub fcSttbfAtnBkmk: i32,
    pub lcbSttbfAtnBkmk: u32,
    pub fcPlcdoaMom: i32,
    pub lcbPlcdoaMom: u32,
    pub fcPlcdoaHdr: i32,
    pub lcbPlcdoaHdr: u32,
    pub fcPlcspaMom: i32,
    pub lcbPlcspaMom: u32,
    pub fcPlcspaHdr: i32,
    pub lcbPlcspaHdr: u32,
    pub fcPlcfAtnbkf: i32,
    pub lcbPlcfAtnbkf: u32,
    pub fcPlcfAtnbkl: i32,
    pub lcbPlcfAtnbkl: u32,
    pub fcPms: i32,
    pub lcbPms: u32,
    pub fcFormFldSttbs: i32,
    pub lcbFormFldSttbs: u32,
    pub fcPlcfendRef: i32,
    pub lcbPlcfendRef: u32,
    pub fcPlcfendTxt: i32,
    pub lcbPlcfendTxt: u32,
    pub fcPlcffldEdn: i32,
    pub lcbPlcffldEdn: u32,
    pub fcPlcfpgdEdn: i32,
    pub lcbPlcfpgdEdn: u32,
    pub fcDggInfo: i32,
    pub lcbDggInfo: u32,
    pub fcSttbfRMark: i32,
    pub lcbSttbfRMark: u32,
    pub fcSttbfCaption: i32,
    pub lcbSttbfCaption: u32,
    pub fcSttbfAutoCaption: i32,
    pub lcbSttbfAutoCaption: u32,
    pub fcPlcfWkb: i32,
    pub lcbPlcfWkb: u32,
    pub fcPlcfSpl: i32,
    pub lcbPlcfSpl: u32,
    pub fcPlcftxbxTxt: i32,
    pub lcbPlcftxbxTxt: u32,
    pub fcPlcffldTxbx: i32,
    pub lcbPlcffldTxbx: u32,
    pub fcPlcfhdrtxbxTxt: i32,
    pub lcbPlcfhdrtxbxTxt: u32,
    pub fcPlcffldHdrTxbx: i32,
    pub lcbPlcffldHdrTxbx: u32,
    pub fcStwUser: i32,
    pub lcbStwUser: u32,
    pub fcSttbTtmbd: i32,
    pub lcbSttbTtmbd: u32,
    pub fcCookieData: i32,
    pub lcbCookieData: u32,
    pub fcPgdMotherOldOld: FCPGDOLD,
    pub fcpgdFtnOldOld: FCPGDOLD,
    pub fcpgdEdnOldOld: FCPGDOLD,
    pub fcSttbfIntlFld: i32,
    pub lcbSttbfIntlFld: u32,
    pub fcRouteSlip: i32,
    pub lcbRouteSlip: u32,
    pub fcSttbSavedBy: i32,
    pub lcbSttbSavedBy: u32,
    pub fcSttbFnm: i32,
    pub lcbSttbFnm: u32,
    pub fcPlcfLst: i32,
    /// Length of the fcPlcffLst
    pub lcbPlcfLst: u32,
    /// Offset in Table Stream of the List Format Override
    pub fcPlfLfo: i32,
    /// Count of bytes of the PlfLfo
    pub lcbPlfLfo: u32,
    pub fcPlcftxbxBkd: i32,
    pub lcbPlcftxbxBkd: u32,
    pub fcPlcftxbxHdrBkd: i32,
    pub lcbPlcftxbxHdrBkd: u32,
    pub fcDocUndoWord9: i32,
    pub lcbDocUndoWord9: u32,
    pub fcRgbUse: i32,
    pub lcbRgbUse: u32,
    pub fcUsp: i32,
    pub lcbUsp: u32,
    pub fcUskf: i32,
    pub lcbUskf: u32,
    pub fcPlcupcRgbUse: i32,
    pub lcbPlcupcRgbUse: u32,
    pub fcPlcupcUsp: i32,
    pub lcbPlcupcUsp: u32,
    pub fcSttbGlsyStyle: i32,
    pub lcbSttbGlsyStyle: u32,
    pub fcPlgosl: i32,
    pub lcbPlgosl: u32,
    pub fcPlcocx: i32,
    pub lcbPlcocx: u32,
    pub fcPlcfBteLvc: i32,
    pub lcbPlcfBteLvc: u32,
    // FILETIME
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
    pub fcPlcfLvcPre10: i32,
    pub lcbPlcfLvcPre10: u32,
    pub fcPlcfAsumy: i32,
    pub lcbPlcfAsumy: u32,
    pub fcPlcfGram: i32,
    pub lcbPlcfGram: u32,
    pub fcSttbListNames: i32,
    pub lcbSttbListNames: u32,
    pub fcSttbfUssr: i32,
    pub lcbSttbfUssr: u32,
    pub fcPlcfTch: i32,
    pub lcbPlcfTch: u32,
    pub fcRmdfThreading: i32,
    pub lcbRmdfThreading: u32,
    pub fcMid: i32,
    pub lcbMid: u32,
    pub fcSttbRgtplc: i32,
    pub lcbSttbRgtplc: u32,
    pub fcMsoEnvelope: i32,
    pub lcbMsoEnvelope: u32,
    pub fcPlcfLad: i32,
    pub lcbPlcfLad: u32,
    pub fcRgDofr: i32,
    pub lcbRgDofr: u32,
    pub fcPlcosl: i32,
    pub lcbPlcosl: u32,
    pub fcPlcfCookieOld: i32,
    pub lcbPlcfCookieOld: u32,
    pub fcPgdMotherOld: FCPGDOLD,    
    pub fcpgdFtnOld: FCPGDOLD,
    pub fcpgdEdnOld: FCPGDOLD,
    pub fcUnused1: i32,
    pub lcbUnused1: u32,
    pub fcPlcfPgp: i32,
    pub lcbPlcfPgp: u32,
    pub fcPlcfuim: i32,
    pub lcbPlcfuim: u32,
    pub fcPlfguidUim: i32,
    pub lcbPlfguidUim: u32,
    pub fcAtrdExtra: i32,
    pub lcbAtrdExtra: u32,
    pub fcPlrsid: i32,
    pub lcbPlrsid: u32,
    pub fcSttbfBkmkFactoid: i32,
    pub lcbSttbfBkmkFactoid: u32,
    pub fcPlcfBkfFactoid: i32,
    pub lcbPlcfBkfFactoid: u32,
    pub fcPlcfcookie: i32,
    pub lcbPlcfcookie: u32,
    pub fcPlcfBklFactoid: i32,
    pub lcbPlcfBklFactoid: u32,
    pub fcFactoidData: i32,
    pub lcbFactoidData: u32,
    pub fcDocUndo: i32,
    pub lcbDocUndo: u32,
    pub fcSttbfBkmkFcc: i32,
    pub lcbSttbfBkmkFcc: u32,
    pub fcPlcfBkfFcc: i32,
    pub lcbPlcfBkfFcc: u32,
    pub fcPlcfBklFcc: i32,
    pub lcbPlcfBklFcc: u32,
    pub fcSttbfbkmkBPRepairs: i32,
    pub lcbSttbfbkmkBPRepairs: u32,
    pub fcPlcfbkfBPRepairs: i32,
    pub lcbPlcfbkfBPRepairs: u32,
    pub fcPmsNew: i32,
    pub lcbPmsNew: u32,
    pub fcODSO: i32,
    pub lcbODSO: u32,
    pub fcPlcfpmiOldXP: i32,
    pub lcbPlcfpmiOldXP: u32,
    pub fcPlcfpmiNewXP: i32,
    pub lcbPlcfpmiNewXP: u32,
    pub fcPlcfpmiMixedXP: i32,
    pub lcbPlcfpmiMixedXP: u32,
    pub fcEncryptedProps: i32,
    pub lcbEncryptedProps: u32,
    pub fcPlcffactoid: i32,
    pub lcbPlcffactoid: u32,
    pub fcPlcflvcOldXP: i32,
    pub lcbPlcflvcOldXP: u32,
    pub fcPlcflvcNewXP: i32,
    pub lcbPlcflvcNewXP: u32,
    pub fcPlcflvcMixedXP: i32,
    pub lcbPlcflvcMixedXP: u32,
    pub fcHplxsdr: i32,
    pub lcbHplxsdr: u32,
    pub fcSttbfBkmkSdt: i32,
    pub lcbSttbfBkmkSdt: u32,
    pub fcPlcfBkfSdt: i32,
    pub lcbPlcfBkfSdt: u32,
    pub fcPlcBlkSdt: i32,
    pub lcbPlcBlkSdt: u32,
    pub fcCustomXForm: i32,
    pub lcbCustomXForm: u32,
    pub fcSttbfBkmkProt: i32,
    pub lcbSttbfBkmkProt: u32,
    pub fcPlcfBkfProt: i32,
    pub lcbPlcfBkfProt: u32,
    pub fcPlcfBklProt: i32,
    pub lcbPlcfBklProt: u32,
    pub fcSttbProtUser: i32,
    pub lcbSttbProtUser: u32,
    pub fcPlcftpc: i32,
    pub lcbPlcftpc: u32,
    pub fcPlcfpmiOld: i32,
    pub lcbPlcfpmiOld: u32,
    pub fcPlcfpmiOldInline: i32,
    pub lcbPlcfpmiOldInline: u32,
    pub fcPlcfpmiNew: i32,
    pub lcbPlcfpmiNew: u32,
    pub fcPlcfpmiNewInline: i32,
    pub lcbPlcfpmiNewInline: u32,
    pub fcPlcfvcOld: i32,
    pub lcbPlcfvcOld: u32,
    pub fcPlcfvcOldInline: i32,
    pub lcbPlcfvcOldInline: u32,
    pub fcPlcflvcNew: i32,
    pub lcbPlcflvcNew: u32,
    pub fcPlcflvcNewInline: i32,
    pub lcbPlcflvcNewInline: u32,
    pub fcpgdMother: FCPGDOLD,
    pub fcpgdFtn: FCPGDOLD,
    pub fcpgdEdn: FCPGDOLD,
    pub fcAfd: i32,
    pub lcbAfd: u32,
    pub cswNew: u16,
    pub actualNFib: u16,
    pub cQuickSavesNew: u16,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Serialize)]
pub struct FCPGDOLD {
    pub fcPgd: i32,
    pub lcbPgd: u32,
    pub fcBkd: i32,
    pub lcbBkd: u32,
}

/// List Tables
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct LSTs {
    pub num_LSTs: u16,
    pub LSTs: Vec<LST>,
}

// contains formatting propertues which apply to the entire list
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct LSTF {
    pub lsid: i32,
    pub tplc: i32,
    pub rgistd: [u16; 9],
    pub flagfield: u8,
    pub compat_flags: u8,
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct LVLF {
    pub iStartAt: i32,
    pub nfc: u8,
    pub jc: u8,
    pub fLegal: bool,
    pub fNoRestart: bool,
    pub fPrev: bool,
    pub fPrevSpace: bool,
    pub fWord6: bool,
    pub rgbxchNums: [u8; 9],
    pub ixchFollow: u8,
    pub dxaSpace: i32,
    pub dxaIndent: i32,
    pub cbGrpprlChpx: u8,
    pub cbGrpprlPapx: u8,
    pub ilvlRestartLim: u8,
    pub grfhic: u8,
}

/// The formatting for a single level of a list
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct LVL {
    pub lvlf: LVLF,
    pub grpprlChpx: Vec<u8>,
    pub grpprlPapx: Vec<u8>,
    pub nubmer_text: String,
}

/// List Table
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct LST {
    pub lstf: LSTF,
    pub level_styles: Vec<LVL>,
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Serialize, Iterable)]
pub struct STD {
    /// Invariant style identifier
    pub sti: u16,
    /// Spare field for any temporary use
    pub fScratch: bool,
    /// PHEs of all text with this style are wrong
    pub fInvalHeight: bool,
    /// UPEs have been generated
    pub fHasUpe: bool,
    /// std has been mass-copied
    pub fMassCopy: bool,
    /// Style kind
    pub stk: u16,
    /// Base style
    pub istdBase: u16,
    /// Number of UPXs (and UPEs)
    pub cupx: u16,
    /// Next style
    pub istdNext: u16,
    /// Offset to end of upx's, start of upe's
    pub bchUpe: u16,
    /// Auto redefine style when appropriate
    pub fAutoRedef: bool,
    /// Hidden from UI?
    pub fHidden: bool,
    /// Style already has valid sprmCRgLidX_80 in it
    pub f97LidsSet: bool,
    /// Copied the lid from sprmCRgLidX into sprmCRgLidX_80
    pub fCopyLang: bool,
    /// HTML Threading compose style
    pub fPersonalCompose: bool,
    /// HTML Threading reply style
    pub fPersonalReply: bool,
    /// HTML Threading - another user's personal style
    pub fPersonal: bool,
    /// pub Do not export this style to HTML/CSS
    pub fNoHtmlExport: bool,
    /// Do not show this style in long style lists
    pub fSemiHidden: bool,
    /// Locked style?
    pub fLocked: bool,
    /// Style is used by a word feature, e.g. footnote
    pub fInternalUse: bool,
    /// Unused bits
    pub unused_bits: u16,
    /// Is this style linked to another?
    pub istdLink: u16,
    /// 4 Spare bits
    pub fSpare: u16,
    /// Marks during merge which doc's style changed
    pub rsid: i32,
    /// Used temporarily during HTML export
    pub iftcHtml: u16,
    /// Unused bits
    pub unused: u16,
    /// Sub-names are separated by chDelimStyle
    pub xstzName: String,
    // grupx and grupe arrays are omitted as they are variable length
}

/// Peice Descriptor (PCD)
#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct PCD {
    pub fNoParaLast: bool,
    pub rest_of_bitfield: [bool; 3],
    pub fn_val: u16,
    pub fc: i32,
    pub prm: u16,
}
#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct SHSHI {
    pub cstd: u16,
    pub cbSTDBaseInFile: u16,
    /// Are build-in stylenames stored
    pub fStdStylenamesWritten: bool,
    /* in the c struct xtra flags are stored in unnamed padding*/
    /// Max sti known when this file was written
    pub stiMaxWhenSaved: u16,
    /// Number of fixed-index istds there are
    pub istdMaxFixedWhenSaved: u16,
    /// Current version of build-in stylenames
    pub nVerBuiltInNamesWhenSaved: u16,
    /* 3 more fields to do */
    pub styles: Vec<STD>,
}

/// Represents the piece table (plcfpcd)
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct PLCF<T: FromCStruct> {
    pub rgfc: Vec<i32>,
    pub rgstruct: Vec<T>,
}

// The first few bytes of the PropertySetStream
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct PropertySetStreamStart {
    pub byteOrder: u16,
    pub version: u16,
    pub OSMajorVersion: u8,
    pub OSMinorVersion: u8,
    pub OSType: u16,
    pub applicationClsid: [u8; 16],
    pub num_property_sets: u32,
    pub rgIdOffset: Vec<RgIdOffset>,
}

#[derive(Debug, Clone)]
pub struct DictionaryPropertyType {
    pub dictionary: Vec<(String, u32)>,
}

#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Serialize)]
pub enum NormalPropertyType {
    VT_EMPTY,
    VT_NULL,
    VT_I2(i16),
    VT_I4(i32),
    VT_R4(f32),
    VT_R8(f64),
    VT_CY(i64),
    VT_DATE(f64),
    VT_BSTR(String),
    VT_ERROR(u32),
    VT_BOOL(bool),
    VT_DECIMAL(f64), // not properly interpreted
    VT_I1(i8),
    VT_UI1(u8),
    VT_UI2(u16),
    VT_UI4(u32),
    VT_I8(i64),
    VT_UI8(u64),
    VT_INT(i32),
    VT_UINT(u32),
    VT_LPSTR(String),
    VT_LPWSTR(String),
    VT_FILETIME(u64),
    VT_BLOB(Vec<u8>),
    VT_STREAM(u64),          // Not properly interpreted
    VT_STORAGE(u64),         // Not properly interpreted
    VT_STREAMED_OBJECT(u64), // Not properly interpreted
    VT_STORED_OBJECT(u64),   // Not properly interpreted
    VT_BLOB_OBJECT(Vec<u8>),
    VT_CF(u32), // Not properly interpreted
    VT_CLSID([u8; 16]),
    VT_VECTOR_VT_I2(Vec<i16>),
    VT_VECTOR_VT_I4(Vec<i32>),
    VT_VECTOR_VT_R4(Vec<f32>),
    VT_VECTOR_VT_R8(Vec<f64>),
    VT_VECTOR_VT_CY(Vec<i64>),
    VT_VECTOR_VT_BOOL(Vec<bool>),
    VT_VECTOR_VT_VARIANT(Vec<NormalPropertyType>),
    VT_VECTOR_VT_I1(Vec<i8>),
    VT_VECTOR_VT_UI1(Vec<u8>),
    VT_VECTOR_VT_UI2(Vec<u16>),
    VT_VECTOR_VT_UI4(Vec<u32>),
    VT_VECTOR_VT_I8(Vec<i64>),
    VT_VECTOR_VT_UI8(Vec<u64>),
    VT_VECTOR_VT_LPSTR(Vec<String>),
    VT_VECTOR_VT_LPWSTR(Vec<String>),
    VT_VECTOR_VT_FILETIME(Vec<u64>),
    VT_VECTOR_VT_CF(Vec<u32>),
    VT_VECTOR_VT_CLSID(Vec<[u8; 16]>),
    VT_ARRAY_VT_I2(Vec<i16>),
    VT_ARRAY_VT_I4(Vec<i32>),
    VT_ARRAY_VT_R4(Vec<f32>),
    VT_ARRAY_VT_R8(Vec<f64>),
    VT_ARRAY_VT_CY(Vec<i64>),
    VT_ARRAY_VT_DATE(Vec<f64>),
    VT_ARRAY_VT_BSTR(Vec<String>),
    VT_ARRAY_VT_ERROR(Vec<i32>),
    VT_ARRAY_VT_BOOL(Vec<bool>),
    VT_ARRAY_VT_VARIANT(Vec<NormalPropertyType>),
    VT_ARRAY_VT_DECIMAL(Vec<f64>),
    VT_ARRAY_VT_I1(Vec<i8>),
    VT_ARRAY_VT_UI1(Vec<u8>),
    VT_ARRAY_VT_UI2(Vec<u16>),
    VT_ARRAY_VT_UI4(Vec<u32>),
    VT_ARRAY_VT_INT(Vec<i32>),
    VT_ARRAY_VT_UINT(Vec<u32>),
    Unknown(u16), // Something has gone wrong
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct DocumentSummaryInfoStream {
    pub codepage: Option<NormalPropertyType>,
    pub category: Option<NormalPropertyType>,
    pub presformat: Option<NormalPropertyType>,
    pub bytecount: Option<NormalPropertyType>,
    pub linecount: Option<NormalPropertyType>,
    pub paracount: Option<NormalPropertyType>,
    pub slidecount: Option<NormalPropertyType>,
    pub notecount: Option<NormalPropertyType>,
    pub hiddencount: Option<NormalPropertyType>,
    pub mmclipcount: Option<NormalPropertyType>,
    pub scale: Option<NormalPropertyType>,
    pub headingpair: Option<NormalPropertyType>,
    pub docparts: Option<NormalPropertyType>,
    pub manager: Option<NormalPropertyType>,
    pub company: Option<NormalPropertyType>,
    pub linksdirty: Option<NormalPropertyType>,
    pub chars_with_spaces: Option<NormalPropertyType>,
    pub sharedoc: Option<NormalPropertyType>,
    pub linkbase: Option<NormalPropertyType>,
    pub hlinks: Option<NormalPropertyType>,
    pub hlinkschanged: Option<NormalPropertyType>,
    pub version: Option<NormalPropertyType>,
    pub digsig: Option<NormalPropertyType>,
    pub content_type: Option<NormalPropertyType>,
    pub content_status: Option<NormalPropertyType>,
    pub language: Option<NormalPropertyType>,
    pub doc_version: Option<NormalPropertyType>,
    pub custom_property_dict: Vec<(String, NormalPropertyType)>,
}

// Specification at: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleps/f7933d28-2cc4-4b36-bc23-8861cbcd37c4
// Spec help at: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleps/3f9119dc-faa2-4bb9-af95-5cf128fa5fbd
#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct SummaryInformation {
    pub title: Option<NormalPropertyType>,
    pub subject: Option<NormalPropertyType>,
    pub author: Option<NormalPropertyType>,
    pub keywords: Option<NormalPropertyType>,
    pub comments: Option<NormalPropertyType>,
    pub template: Option<NormalPropertyType>,
    pub lastAuthor: Option<NormalPropertyType>,
    pub revisionNumber: Option<NormalPropertyType>,
    pub applicationName: Option<NormalPropertyType>,
    pub editTime: Option<NormalPropertyType>,
    pub lastPrinted: Option<NormalPropertyType>,
    pub create_dtm: Option<NormalPropertyType>,
    pub lastSave_dtm: Option<NormalPropertyType>,
    pub pageCount: Option<NormalPropertyType>,
    pub wordCount: Option<NormalPropertyType>,
    pub charCount: Option<NormalPropertyType>,
    pub doc_security: Option<NormalPropertyType>,
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct RgIdOffset {
    pub formatID: [u8; 16],
    pub sectionOffset: u32,
}
#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub enum PropertyIdentifier {
    Normal(u32),
    DictionaryProperty,
    CodePageProperty,
    LocaleProperty,
    BehaviourProperty,
}

#[derive(Debug, Clone)]
pub enum PropertyType {
    NormalPropertyType(NormalPropertyType),
    DictionaryPropertyType(DictionaryPropertyType),
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct PropertyIdentifierAndOffset {
    pub propertyIdentifier: PropertyIdentifier,
    pub propertyOffset: u32,
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct PropertySetStream {
    pub propertySetStreamVals: PropertySetStreamStart,
    pub propertySets: Vec<PropertySet>,
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct PropertySet {
    pub cb: u32,
    pub cProperties: u32,
    pub rgProperties: Vec<PropertyIdentifierAndOffset>,
    pub properties: Vec<PropertyType>,
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct RgProps {
    pub propID: u32,
    pub propOffset: u32,
}

impl PropertyIdentifier {
    pub fn from_u32(val: u32) -> Self {
        match val {
            0x00000002..=0x7FFFFFFF => PropertyIdentifier::Normal(val),
            0x00000000 => PropertyIdentifier::DictionaryProperty,
            0x00000001 => PropertyIdentifier::CodePageProperty,
            0x80000000 => PropertyIdentifier::LocaleProperty,
            0x80000003 => PropertyIdentifier::BehaviourProperty,
            _ => panic!("Invalid property Identifyer used"),
        }
    }
}

impl DictionaryPropertyType {
    pub fn get_name(&self, id: u32) -> Option<&String> {
        for (name, prop_id) in &self.dictionary {
            if *prop_id == id {
                return Some(name);
            }
        }
        None
    }
}

#[allow(non_snake_case, unused)]
impl LSTF {
    pub fn fSimpleList(&self) -> bool {
        self.flagfield & 0x01 == 0x01
    }

    pub fn fHybridList(&self) -> bool {
        self.flagfield & 0x10 == 0x10
    }
}
