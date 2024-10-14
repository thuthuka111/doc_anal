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

#[allow(non_snake_case, unused)]
#[derive(Debug, Iterable, Serialize)]
pub struct Fib {
    pub wIdent: u16,
    pub nFib: u16,
    pub fComplex: bool,
    pub fWhichTblStm: bool,
    pub fcPlcfbtePapx: i32,
    pub lcbPlcfbtePapx: u32,
    pub fcMin: i32,
    pub fcMac: i32,
    /* Lengths for the approprate Text Sections */
    pub ccpText: i32,
    pub ccpFtn: i32,
    pub ccpHdr: i32,
    pub ccpMcr: i32,
    pub ccpAtn: i32,
    pub ccpEdn: i32,
    pub ccpTxbx: i32,
    pub ccpHrdTxbx: i32,
    /// File offset of the STSH (Stylesheet) in the table stream
    pub fcStshf: i32,
    /// Count of bytes of the STSH allocation
    pub lcbStshf: u32,
    /// Offset in Table Stream of Complex file text portion
    pub fcClx: i32,
    /// Length of the fcClx
    pub lcbClx: i32,
    /// Offset in Table Stream of list formation information
    pub fcPlcfLst: i32,
    /// Length of the fcPlcffLst
    pub lcbPlcfLst: u32,
    /// Offset in Table Stream of the List Format Override
    pub fcPlfLfo: i32,
    /// Count of bytes of the PlfLfo
    pub lcbPlfLfo: u32,
    // Add other FIB fields as needed
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
#[derive(Debug)]
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
#[derive(Debug)]
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
