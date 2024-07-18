use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

use crate::subreader::SubReader;

/// Trait to indicate that a struct can be built from its C representation in bytes
pub trait FromCStruct {
    /// The size of the C struct in bytes
    fn c_size() -> usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

/// A Trait that states that 'Self' is able to be constructed from some reader
pub trait FromReader: Sized {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self>;
}

// region: Structs

#[allow(non_snake_case)]
pub struct _STSHI {
    // Style sheet information strucure
    cstd: u16,
    cbSTDBaseInFile: u16,
    fStdStylenamesWritten: u16,
    stiMaxWhenSaved: u16,
    istdMaxFixedWhenSaved: u16,
    nVerBuiltInNamesWhenSaved: u16,
    rgftcStandardChpStsh: [u16; 0x0A], // 10
    cbLSD: u16,
    mpstilsd: [u16; 0x0A], // 10
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
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
    // Add other FIB fields as needed
}

#[allow(non_snake_case, unused)]
#[derive(Debug)]
pub struct STD {
    /// Invariant style identifier
    sti: u16,
    /// Spare field for any temporary use
    fScratch: bool,
    /// PHEs of all text with this style are wrong
    fInvalHeight: bool,
    /// UPEs have been generated
    fHasUpe: bool,
    /// std has been mass-copied
    fMassCopy: bool,
    /// Style kind
    stk: u16,
    /// Base style
    istdBase: u16,
    /// Number of UPXs (and UPEs)
    cupx: u16,
    /// Next style
    istdNext: u16,
    /// Offset to end of upx's, start of upe's
    bchUpe: u16,
    /// Auto redefine style when appropriate
    fAutoRedef: bool,
    /// Hidden from UI?
    fHidden: bool,
    /// Style already has valid sprmCRgLidX_80 in it
    f97LidsSet: bool,
    /// Copied the lid from sprmCRgLidX into sprmCRgLidX_80
    fCopyLang: bool,
    /// HTML Threading compose style
    fPersonalCompose: bool,
    /// HTML Threading reply style
    fPersonalReply: bool,
    /// HTML Threading - another user's personal style
    fPersonal: bool,
    /// Do not export this style to HTML/CSS
    fNoHtmlExport: bool,
    /// Do not show this style in long style lists
    fSemiHidden: bool,
    /// Locked style?
    fLocked: bool,
    /// Style is used by a word feature, e.g. footnote
    fInternalUse: bool,
    /// Unused bits
    unused_bits: u16,
    /// Is this style linked to another?
    istdLink: u16,
    /// 4 Spare bits
    fSpare: u16,
    /// Marks during merge which doc's style changed
    rsid: i32,
    /// Used temporarily during HTML export
    iftcHtml: u16,
    /// Unused bits
    unused: u16,
    /// Sub-names are separated by chDelimStyle
    xstzName: String,
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
#[derive(Debug)]
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
    rgfc: Vec<i32>,
    rgstruct: Vec<T>,
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
#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub struct DocumentSummaryInfoStream {
    codepage: Option<NormalPropertyType>,
    category: Option<NormalPropertyType>,
    presformat: Option<NormalPropertyType>,
    bytecount: Option<NormalPropertyType>,
    linecount: Option<NormalPropertyType>,
    paracount: Option<NormalPropertyType>,
    slidecount: Option<NormalPropertyType>,
    notecount: Option<NormalPropertyType>,
    hiddencount: Option<NormalPropertyType>,
    mmclipcount: Option<NormalPropertyType>,
    scale: Option<NormalPropertyType>,
    headingpair: Option<NormalPropertyType>,
    docparts: Option<NormalPropertyType>,
    manager: Option<NormalPropertyType>,
    company: Option<NormalPropertyType>,
    linksdirty: Option<NormalPropertyType>,
    chars_with_spaces: Option<NormalPropertyType>,
    sharedoc: Option<NormalPropertyType>,
    linkbase: Option<NormalPropertyType>,
    hlinks: Option<NormalPropertyType>,
    hlinkschanged: Option<NormalPropertyType>,
    version: Option<NormalPropertyType>,
    digsig: Option<NormalPropertyType>,
    content_type: Option<NormalPropertyType>,
    content_status: Option<NormalPropertyType>,
    language: Option<NormalPropertyType>,
    doc_version: Option<NormalPropertyType>,

    custom_property_dict: Vec<(String, NormalPropertyType)>,
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

impl FromReader for Fib {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        reader.seek(SeekFrom::Start(0))?;

        let wIdent = reader.read_u16::<LittleEndian>()?;
        let nFib = reader.read_u16::<LittleEndian>()?;

        reader.seek(SeekFrom::Start(0x0A))?;
        let bitfield = reader.read_u16::<LittleEndian>()?;
        let fComplex = (bitfield & 0x0004) == 0x0004;
        let fWhichTblStm = (bitfield & 0x200) == 0x200;

        reader.seek(SeekFrom::Start(0x0102))?; // Skip to get to fcPlcfbtePapx
        let fcPlcfbtePapx = reader.read_i32::<LittleEndian>()?;
        let lcbPlcfbtePapx = reader.read_u32::<LittleEndian>()?;

        reader.seek(SeekFrom::Start(0x0018))?;
        let fcMin = reader.read_i32::<LittleEndian>()?;
        let fcMac = reader.read_i32::<LittleEndian>()?;

        reader.seek(SeekFrom::Start(0x004C))?;
        let ccpText = reader.read_i32::<LittleEndian>()?;
        let ccpFtn = reader.read_i32::<LittleEndian>()?;
        let ccpHdd = reader.read_i32::<LittleEndian>()?;
        let ccpMcr = reader.read_i32::<LittleEndian>()?;
        let ccpAtn = reader.read_i32::<LittleEndian>()?;
        let ccpEdn = reader.read_i32::<LittleEndian>()?;
        let ccpTxbx = reader.read_i32::<LittleEndian>()?;
        let ccpHrdTxbx = reader.read_i32::<LittleEndian>()?;
        println!(
            "current cursor position: {}",
            reader.seek(SeekFrom::Current(0))?
        );

        reader.seek(SeekFrom::Start(0x00A2))?;
        let fcStshf = reader.read_i32::<LittleEndian>()?;
        let lcbStshf = reader.read_u32::<LittleEndian>()?;

        reader.seek(SeekFrom::Start(0x01A2))?;
        let fcClx = reader.read_i32::<LittleEndian>()?;
        let lcbClx = reader.read_i32::<LittleEndian>()?;

        Ok(Fib {
            wIdent,
            nFib,
            fComplex,
            fWhichTblStm,
            fcPlcfbtePapx,
            lcbPlcfbtePapx,
            fcMin,
            fcMac,
            ccpText,
            ccpFtn,
            ccpHdr: ccpHdd,
            ccpMcr,
            ccpAtn,
            ccpEdn,
            ccpTxbx,
            ccpHrdTxbx,
            fcStshf,
            lcbStshf,
            fcClx,
            lcbClx,
        })
    }
}

impl FromReader for SHSHI {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(stsh_buffer: &mut R) -> io::Result<Self> {
        // read first 2 bytes for size of the STSHI structure
        let cbStshi = stsh_buffer.read_u16::<LittleEndian>()?;
        let mut stshi_buffer = vec![0; cbStshi as usize];
        stsh_buffer.read_exact(&mut stshi_buffer)?;

        let mut stshi_buffer = BufReader::new(Cursor::new(stshi_buffer));

        let cstd = stshi_buffer.read_u16::<LittleEndian>()?;
        let cbSTDBaseInFile = stshi_buffer.read_u16::<LittleEndian>()?;
        let some_flags = stshi_buffer.read_u16::<LittleEndian>()?;
        let fStdStylenamesWritten = (some_flags & 0x80) == 0x80;
        let stiMaxWhenSaved = stshi_buffer.read_u16::<LittleEndian>()?;
        let istdMaxFixedWhenSaved = stshi_buffer.read_u16::<LittleEndian>()?;
        let nVerBuiltInNamesWhenSaved = stshi_buffer.read_u16::<LittleEndian>()?;

        // Reading in the styles
        let mut styles = Vec::with_capacity(cstd as usize);

        println!("Number of styles: {}", cstd);

        for _ in 0..cstd {
            // size of following STD structure
            let cbStd = stsh_buffer.read_u16::<LittleEndian>()?;
            if cbStd == 0 {
                continue;
            }
            let mut std_buffer = vec![0; cbStd as usize];
            stsh_buffer.read_exact(&mut std_buffer)?;
            let mut std_buffer = BufReader::new(Cursor::new(std_buffer));

            let stylesheet_std = STD::from_reader(&mut std_buffer)?;

            println!("{:?}\n", stylesheet_std);
            let _remaining_buff = std_buffer.fill_buf()?;
            // println!("Remaining buffer: {}", hex::encode_upper(remaining_buff));
            // assert!(remaining_buff.len() == 0, "Buffer not fully read");

            styles.push(stylesheet_std);
        }

        Ok(SHSHI {
            cstd,
            cbSTDBaseInFile,
            fStdStylenamesWritten,
            stiMaxWhenSaved,
            istdMaxFixedWhenSaved,
            nVerBuiltInNamesWhenSaved,
            styles,
        })
    }
}

impl FromReader for PLCF<PCD> {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        // let grpprls = vec![];
        // NOTE: this currently does not support reading the Grppr byte which starts with one
        // Hence break if encountered
        // To rememdy this, if 1 is encountered then you gotta keep going untill the first page byte is a 2
        let first_byte = reader.read_u8()?; // clxt(2)
        assert_eq!(first_byte, 2);

        let lcb = reader.read_u32::<LittleEndian>()?;
        let mut plcfpcd_buff: Vec<u8> = vec![0; lcb as usize];
        reader.read_exact(&mut plcfpcd_buff)?;

        let plcf = PLCF::<PCD>::from_bytes(&plcfpcd_buff);
        // let remaining_buff = reader.fill_buf()?;
        // assert_eq!(remaining_buff.len(), 0);

        // Ok((grpprls, plcf))
        Ok(plcf)
    }
}

impl FromReader for STD {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(std_buffer: &mut R) -> io::Result<Self> {
        let mut bitfield: u16;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let sti = bitfield & 0x0FFF;
        let fScratch = (bitfield & 0x8000) == 0x8000;
        let fInvalHeight = (bitfield & 0x4000) == 0x4000;
        let fHasUpe = (bitfield & 0x2000) == 0x2000;
        let fMassCopy = (bitfield & 0x1000) == 0x1000;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let stk = bitfield & 0xF000;
        let istdBase = bitfield & 0x0FFF;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let cupx = bitfield & 0xF000;
        let istdNext = bitfield & 0x0FFF;

        let bchUpe = std_buffer.read_u16::<LittleEndian>()?;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let fAutoRedef = (bitfield & 0x8000) == 0x8000;
        let fHidden = (bitfield & 0x4000) == 0x4000;
        let f97LidsSet = (bitfield & 0x2000) == 0x2000;
        let fCopyLang = (bitfield & 0x1000) == 0x1000;
        let fPersonalCompose = (bitfield & 0x0800) == 0x0800;
        let fPersonalReply = (bitfield & 0x0400) == 0x0400;
        let fPersonal = (bitfield & 0x0200) == 0x0200;
        let fNoHtmlExport = (bitfield & 0x0100) == 0x0100;
        let fSemiHidden = (bitfield & 0x0080) == 0x0080;
        let fLocked = (bitfield & 0x0040) == 0x0040;
        let fInternalUse = (bitfield & 0x0020) == 0x0020;
        let _ = bitfield & 0x001F;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let istdLink = bitfield & 0x0FFF;
        let fSpare = bitfield & 0xF000;

        let rsid = std_buffer.read_i32::<LittleEndian>()?;

        bitfield = std_buffer.read_u16::<LittleEndian>()?;
        let iftcHtml = bitfield & 0x0007;
        let _ = bitfield & 0xFFF8;

        let xstzName = {
            let length_byte = std_buffer.read_u8()?;
            let mut name_buffer: Vec<u16> = vec![0; length_byte as usize];
            std_buffer.read_u16_into::<BigEndian>(&mut name_buffer)?;
            let _ = std_buffer.read_u8()?; // Null Terminator
            String::from_utf16(&name_buffer).unwrap()
        };

        Ok(STD {
            sti,
            fScratch,
            fInvalHeight,
            fHasUpe,
            fMassCopy,
            stk,
            istdBase,
            cupx,
            istdNext,
            bchUpe,
            fAutoRedef,
            fHidden,
            f97LidsSet,
            fCopyLang,
            fPersonalCompose,
            fPersonalReply,
            fPersonal,
            fNoHtmlExport,
            fSemiHidden,
            fLocked,
            fInternalUse,
            unused_bits: 0,
            istdLink,
            fSpare,
            rsid,
            iftcHtml,
            unused: 0,
            xstzName,
        })
    }
}

impl FromReader for DocumentSummaryInfoStream {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let property_set_stream = PropertySetStream::from_reader(reader)?;

        let mut codepage = None;
        let mut category = None;
        let mut presformat = None;
        let mut bytecount = None;
        let mut linecount = None;
        let mut paracount = None;
        let mut slidecount = None;
        let mut notecount = None;
        let mut hiddencount = None;
        let mut mmclipcount = None;
        let mut scale = None;
        let mut headingpair = None;
        let mut docparts = None;
        let mut manager = None;
        let mut company = None;
        let mut linksdirty = None;
        let mut chars_with_spaces = None;
        let mut sharedoc = None;
        let mut linkbase = None;
        let mut hlinks = None;
        let mut hlinkschanged = None;
        let mut version = None;
        let mut digsig = None;
        let mut content_type = None;
        let mut content_status = None;
        let mut language = None;
        let mut doc_version = None;

        for (prop_ident_and_offset, property_type) in property_set_stream.propertySets[0]
            .rgProperties
            .iter()
            .zip(property_set_stream.propertySets[0].properties.iter())
        {
            let property_type = match property_type {
                PropertyType::NormalPropertyType(property_type) => property_type,
                _ => {
                    // This should not happen
                    continue;
                }
            };
            let something = &prop_ident_and_offset.propertyIdentifier;
            match something {
                PropertyIdentifier::Normal(prop_id) => match prop_id {
                    0x0001 => codepage = Some(property_type.clone()),
                    0x0002 => category = Some(property_type.clone()),
                    0x0003 => presformat = Some(property_type.clone()),
                    0x0004 => bytecount = Some(property_type.clone()),
                    0x0005 => linecount = Some(property_type.clone()),
                    0x0006 => paracount = Some(property_type.clone()),
                    0x0007 => slidecount = Some(property_type.clone()),
                    0x0008 => notecount = Some(property_type.clone()),
                    0x0009 => hiddencount = Some(property_type.clone()),
                    0x000A => mmclipcount = Some(property_type.clone()),
                    0x000B => scale = Some(property_type.clone()),
                    0x000C => headingpair = Some(property_type.clone()),
                    0x000D => docparts = Some(property_type.clone()),
                    0x000E => manager = Some(property_type.clone()),
                    0x000F => company = Some(property_type.clone()),
                    0x0010 => linksdirty = Some(property_type.clone()),
                    0x0011 => chars_with_spaces = Some(property_type.clone()),
                    0x0013 => sharedoc = Some(property_type.clone()),
                    0x0014 => linkbase = Some(property_type.clone()),
                    0x0015 => hlinks = Some(property_type.clone()),
                    0x0016 => hlinkschanged = Some(property_type.clone()),
                    0x0017 => version = Some(property_type.clone()),
                    0x0018 => digsig = Some(property_type.clone()),
                    0x001A => content_type = Some(property_type.clone()),
                    0x001B => content_status = Some(property_type.clone()),
                    0x001C => language = Some(property_type.clone()),
                    0x001D => doc_version = Some(property_type.clone()),
                    _ => {}
                },
                PropertyIdentifier::CodePageProperty => {}
                _ => {
                    panic!("Bad property type identifer in DocumentSummaryInfoStream")
                }
            }
        }

        // Doing User Defined Properties if there are
        let custom_property_dict: Vec<(String, NormalPropertyType)> = match property_set_stream
            .propertySets
            .get(1)
        {
            Some(property_set) => match &property_set.properties[0] {
                PropertyType::DictionaryPropertyType(dictionary) => {
                    let mut custom_property_dict: Vec<(String, NormalPropertyType)> = Vec::new();
                    let mut dict_item_to_rgid_n_offset_index = Vec::new();

                    for (name, dict_def_prop_id) in &dictionary.dictionary {
                        for (index, prop_ident_and_offset) in
                            property_set.rgProperties.iter().enumerate()
                        {
                            match prop_ident_and_offset.propertyIdentifier {
                                PropertyIdentifier::Normal(prop_id) => {
                                    if prop_id == *dict_def_prop_id {
                                        dict_item_to_rgid_n_offset_index
                                            .push((name.clone(), index - 1)); // Short term fix
                                    }
                                }
                                _ => {}
                            };
                        }
                    }

                    for (name, index) in dict_item_to_rgid_n_offset_index {
                        let property_type = &property_set.properties[index];
                        let property_type = match property_type {
                            PropertyType::NormalPropertyType(property_type) => property_type,
                            _ => {
                                panic!("Bad property type identifer in DocumentSummaryInfoStream")
                            }
                        };
                        custom_property_dict.push((name, property_type.clone()));
                    }
                    custom_property_dict
                }
                _ => {
                    panic!("Bad property type identifer in DocumentSummaryInfoStream")
                }
            },
            None => Vec::new(),
        };

        Ok(DocumentSummaryInfoStream {
            codepage,
            category,
            presformat,
            bytecount,
            linecount,
            paracount,
            slidecount,
            notecount,
            hiddencount,
            mmclipcount,
            scale,
            headingpair,
            docparts,
            manager,
            company,
            linksdirty,
            chars_with_spaces,
            sharedoc,
            linkbase,
            hlinks,
            hlinkschanged,
            version,
            digsig,
            content_type,
            content_status,
            language,
            doc_version,
            custom_property_dict,
        })
    }
}

impl FromReader for SummaryInformation {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let property_set_stream = PropertySetStream::from_reader(reader)?;

        let mut title = None;
        let mut subject = None;
        let mut author = None;
        let mut keywords = None;
        let mut comments = None;
        let mut template = None;
        let mut last_author = None;
        let mut revision_number = None;
        let mut application_name = None;
        let mut edit_time = None;
        let mut last_printed = None;
        let mut create_dtm = None;
        let mut last_save_dtm = None;
        let mut page_count = None;
        let mut word_count = None;
        let mut char_count = None;
        let mut doc_security = None;

        for (prop_ident_and_offset, property_type) in property_set_stream.propertySets[0]
            .rgProperties
            .iter()
            .zip(property_set_stream.propertySets[0].properties.iter())
        {
            let property_type = match property_type {
                PropertyType::NormalPropertyType(property_type) => property_type,
                _ => {
                    // This should not happen
                    continue;
                }
            };

            let something = &prop_ident_and_offset.propertyIdentifier;
            match something {
                PropertyIdentifier::Normal(prop_id) => match prop_id {
                    0x0002 => title = Some(property_type.clone()),
                    0x0003 => subject = Some(property_type.clone()),
                    0x0004 => author = Some(property_type.clone()),
                    0x0005 => keywords = Some(property_type.clone()),
                    0x0006 => comments = Some(property_type.clone()),
                    0x0007 => template = Some(property_type.clone()),
                    0x0008 => last_author = Some(property_type.clone()),
                    0x0009 => revision_number = Some(property_type.clone()),
                    0x0012 => application_name = Some(property_type.clone()),
                    0x000A => edit_time = Some(property_type.clone()),
                    0x000B => last_printed = Some(property_type.clone()),
                    0x000C => create_dtm = Some(property_type.clone()),
                    0x000D => last_save_dtm = Some(property_type.clone()),
                    0x000E => page_count = Some(property_type.clone()),
                    0x000F => word_count = Some(property_type.clone()),
                    0x0010 => char_count = Some(property_type.clone()),
                    0x0013 => doc_security = Some(property_type.clone()),
                    _ => {}
                },
                PropertyIdentifier::CodePageProperty => {}
                _ => {
                    panic!("Bad property type identifer in SummaryInformation")
                }
            }
        }

        Ok(SummaryInformation {
            title: title,
            subject: subject,
            author: author,
            keywords: keywords,
            comments: comments,
            template: template,
            lastAuthor: last_author,
            revisionNumber: revision_number,
            applicationName: application_name,
            editTime: edit_time,
            lastPrinted: last_printed,
            create_dtm: create_dtm,
            lastSave_dtm: last_save_dtm,
            pageCount: page_count,
            wordCount: word_count,
            charCount: char_count,
            doc_security: doc_security,
        })
    }
}

impl FromReader for PropertySetStreamStart {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let byteorder = reader.read_u16::<LittleEndian>()?;
        let version = reader.read_u16::<LittleEndian>()?;

        let os_major_minor_bitfield = reader.read_u16::<BigEndian>()?;
        let OSMajorVersion = (os_major_minor_bitfield & 0xFF00) as u8;
        let OSMinorVersion = (os_major_minor_bitfield & 0x00FF) as u8;
        let OSType = reader.read_u16::<LittleEndian>()?; // should be 2

        let mut applicationClsid = [0 as u8; 16];
        reader.read_exact(&mut applicationClsid)?;
        let cSections = reader.read_u32::<LittleEndian>()?; // should be 0s only

        let mut rg_offsets = Vec::with_capacity(cSections as usize);

        // Reading the types and the offsets for the PropertySets
        for _ in 0..cSections {
            let mut buffer_bytes = [0 as u8; 20];
            reader.read_exact(&mut buffer_bytes)?;

            let rgIdOffset = RgIdOffset::from_bytes(&buffer_bytes);
            rg_offsets.push(rgIdOffset);
        }

        Ok(PropertySetStreamStart {
            byteOrder: byteorder,
            version,
            OSMajorVersion,
            OSMinorVersion,
            OSType,
            applicationClsid,
            num_property_sets: cSections,
            rgIdOffset: rg_offsets,
        })
    }
}

impl FromReader for PropertySetStream {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let property_stream_set_start = PropertySetStreamStart::from_reader(reader)?;

        let mut property_sets =
            Vec::with_capacity(property_stream_set_start.num_property_sets as usize);

        for id_n_offset in &property_stream_set_start.rgIdOffset {
            // new reader that has a base (start) at the section offset
            let offset = id_n_offset.sectionOffset as u64;
            let mut subreader = SubReader::new(reader, offset)?;
            let property_set = PropertySet::from_reader(&mut subreader)?;

            property_sets.push(property_set);
        }

        Ok(PropertySetStream {
            propertySetStreamVals: property_stream_set_start,
            propertySets: property_sets,
        })
    }
}

impl FromReader for PropertySet {
    #[allow(non_snake_case)]
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let cb = reader.read_u32::<LittleEndian>()?;
        let cProperties = reader.read_u32::<LittleEndian>()?;

        let mut rgProperties = Vec::with_capacity(cProperties as usize);

        for _ in 0..cProperties {
            // Consider using an impl Constructor for this
            let propertyIdentifier =
                PropertyIdentifier::from_u32(reader.read_u32::<LittleEndian>()?);

            let propertyOffset = reader.read_u32::<LittleEndian>()?;
            rgProperties.push(PropertyIdentifierAndOffset {
                propertyIdentifier,
                propertyOffset,
            });
        }

        let mut _properties = Vec::with_capacity(cProperties as usize);

        for val in &rgProperties {
            let offset = val.propertyOffset as u64;
            reader.seek(SeekFrom::Start(offset))?;

            match val.propertyIdentifier {
                PropertyIdentifier::Normal(_) => {
                    let normal = NormalPropertyType::from_reader(reader)?;
                    _properties.push(PropertyType::NormalPropertyType(normal));
                }
                PropertyIdentifier::DictionaryProperty => {
                    let dictionary = DictionaryPropertyType::from_reader(reader)?;
                    _properties.push(PropertyType::DictionaryPropertyType(dictionary));
                }
                PropertyIdentifier::CodePageProperty => {
                    // Not implemented
                    // let property = T::from_reader(reader)?;
                    // _properties.push(property);
                }
                PropertyIdentifier::LocaleProperty => {
                    // Not implemented
                    // let property = T::from_reader(reader)?;
                    // _properties.push(property);
                }
                PropertyIdentifier::BehaviourProperty => {
                    // Not implemented
                    // let property = T::from_reader(reader)?;
                    // _properties.push(property);
                }
            }
        }

        Ok(PropertySet {
            cb,
            cProperties,
            rgProperties,
            properties: _properties,
        })
    }
}

impl FromReader for NormalPropertyType {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let type_val = reader.read_u16::<LittleEndian>()?;
        let _padding = reader.read_u16::<LittleEndian>()?;

        match type_val {
            0x0000 => Ok(NormalPropertyType::VT_EMPTY),
            0x0001 => Ok(NormalPropertyType::VT_NULL),
            0x0002 => Ok(NormalPropertyType::VT_I2(
                reader.read_i16::<LittleEndian>()?,
            )),
            0x0003 => Ok(NormalPropertyType::VT_I4(
                reader.read_i32::<LittleEndian>()?,
            )),
            0x0004 => Ok(NormalPropertyType::VT_R4(
                reader.read_f32::<LittleEndian>()?,
            )),
            0x0005 => Ok(NormalPropertyType::VT_R8(
                reader.read_f64::<LittleEndian>()?,
            )),
            0x0006 => Ok(NormalPropertyType::VT_CY(
                reader.read_i64::<LittleEndian>()?,
            )),
            0x0007 => Ok(NormalPropertyType::VT_DATE(reader.read_f64::<BigEndian>()?)),
            0x0008 => {
                // this function is incorrect, does not check the value of CP_WINUNICODE (0x04B0) to see what this is
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut u16_buffer = vec![0; length as usize / 2];
                for i in 0..length as usize / 2 {
                    u16_buffer[i] = u16::from_be_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
                }
                Ok(NormalPropertyType::VT_BSTR(
                    String::from_utf16(&u16_buffer).unwrap(),
                ))
            }
            0x000A => Ok(NormalPropertyType::VT_ERROR(
                reader.read_u32::<LittleEndian>()?,
            )),
            0x000B => Ok(NormalPropertyType::VT_BOOL(
                reader.read_u16::<BigEndian>()? != 0,
            )),
            0x000E => Ok(NormalPropertyType::VT_DECIMAL(
                reader.read_f64::<LittleEndian>()?,
            )), // not properly interpreted
            0x0010 => Ok(NormalPropertyType::VT_I1(reader.read_i8()?)),
            0x0011 => Ok(NormalPropertyType::VT_UI1(reader.read_u8()?)),
            0x0012 => Ok(NormalPropertyType::VT_UI2(
                reader.read_u16::<LittleEndian>()?,
            )),
            0x0013 => Ok(NormalPropertyType::VT_UI4(
                reader.read_u32::<LittleEndian>()?,
            )),
            0x0014 => Ok(NormalPropertyType::VT_I8(
                reader.read_i64::<LittleEndian>()?,
            )),
            0x0015 => Ok(NormalPropertyType::VT_UI8(
                reader.read_u64::<LittleEndian>()?,
            )),
            0x0016 => Ok(NormalPropertyType::VT_INT(
                reader.read_i32::<LittleEndian>()?,
            )),
            0x0017 => Ok(NormalPropertyType::VT_UINT(
                reader.read_u32::<LittleEndian>()?,
            )),
            0x001E => {
                // this function is incorrect, does not check the value of CP_WINUNICODE (0x04B0) to see what this is
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                Ok(NormalPropertyType::VT_LPSTR(
                    String::from_utf8(buffer).unwrap(),
                ))
            }
            0x001F => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut u16_buffer = vec![0; length as usize / 2];
                for i in 0..length as usize / 2 {
                    u16_buffer[i] = u16::from_be_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
                }
                Ok(NormalPropertyType::VT_LPWSTR(
                    String::from_utf16(&u16_buffer).unwrap(),
                ))
            }
            0x0040 => Ok(NormalPropertyType::VT_FILETIME(
                reader.read_u64::<LittleEndian>()?,
            )),
            0x0041 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                Ok(NormalPropertyType::VT_BLOB(buffer))
            }
            0x0042 => Ok(NormalPropertyType::VT_STREAM(
                reader.read_u64::<LittleEndian>()?,
            )), // Not properly interpreted
            0x0043 => Ok(NormalPropertyType::VT_STORAGE(
                reader.read_u64::<LittleEndian>()?,
            )), // Not properly interpreted
            0x0044 => Ok(NormalPropertyType::VT_STREAMED_OBJECT(
                reader.read_u64::<LittleEndian>()?,
            )), // Not properly interpreted
            0x0045 => Ok(NormalPropertyType::VT_STORED_OBJECT(
                reader.read_u64::<LittleEndian>()?,
            )), // Not properly interpreted
            0x0046 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                Ok(NormalPropertyType::VT_BLOB_OBJECT(buffer))
            }
            0x0047 => Ok(NormalPropertyType::VT_CF(
                reader.read_u32::<LittleEndian>()?,
            )), // Not properly interpreted
            0x0048 => {
                let mut buffer = [0; 16];
                reader.read_exact(&mut buffer)?;
                Ok(NormalPropertyType::VT_CLSID(buffer))
            }
            0x1002 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 2);
                for i in 0..length as usize / 2 {
                    values.push(i16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_I2(values))
            }
            0x1003 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 4);
                for i in 0..length as usize / 4 {
                    values.push(i32::from_le_bytes([
                        buffer[i * 4],
                        buffer[i * 4 + 1],
                        buffer[i * 4 + 2],
                        buffer[i * 4 + 3],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_I4(values))
            }
            0x1004 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 4);
                for i in 0..length as usize / 4 {
                    values.push(f32::from_le_bytes([
                        buffer[i * 4],
                        buffer[i * 4 + 1],
                        buffer[i * 4 + 2],
                        buffer[i * 4 + 3],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_R4(values))
            }
            0x1005 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 8);
                for i in 0..length as usize / 8 {
                    values.push(f64::from_le_bytes([
                        buffer[i * 8],
                        buffer[i * 8 + 1],
                        buffer[i * 8 + 2],
                        buffer[i * 8 + 3],
                        buffer[i * 8 + 4],
                        buffer[i * 8 + 5],
                        buffer[i * 8 + 6],
                        buffer[i * 8 + 7],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_R8(values))
            }
            0x1006 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 8);
                for i in 0..length as usize / 8 {
                    values.push(i64::from_le_bytes([
                        buffer[i * 8],
                        buffer[i * 8 + 1],
                        buffer[i * 8 + 2],
                        buffer[i * 8 + 3],
                        buffer[i * 8 + 4],
                        buffer[i * 8 + 5],
                        buffer[i * 8 + 6],
                        buffer[i * 8 + 7],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_CY(values))
            }
            0x100B => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 2);
                for i in 0..length as usize / 2 {
                    values.push(u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]) != 0);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_BOOL(values))
            }
            0x100C => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut values = Vec::with_capacity(length as usize);
                for _ in 0..length as usize {
                    let property_val = NormalPropertyType::from_reader(reader)?;
                    values.push(property_val);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_VARIANT(values))
            }
            0x1010 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize);
                for i in 0..length as usize {
                    values.push(buffer[i] as i8);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_I1(values))
            }
            0x1011 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize);
                for i in 0..length as usize {
                    values.push(buffer[i]);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_UI1(values))
            }
            0x1012 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 2);
                for i in 0..length as usize / 2 {
                    let value = u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
                    values.push(value);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_UI2(values))
            }
            0x1013 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 4);
                for i in 0..length as usize / 4 {
                    values.push(u32::from_le_bytes([
                        buffer[i * 4],
                        buffer[i * 4 + 1],
                        buffer[i * 4 + 2],
                        buffer[i * 4 + 3],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_UI4(values))
            }
            0x1014 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 8);
                for i in 0..length as usize / 8 {
                    values.push(i64::from_le_bytes([
                        buffer[i * 8],
                        buffer[i * 8 + 1],
                        buffer[i * 8 + 2],
                        buffer[i * 8 + 3],
                        buffer[i * 8 + 4],
                        buffer[i * 8 + 5],
                        buffer[i * 8 + 6],
                        buffer[i * 8 + 7],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_I8(values))
            }
            0x1015 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 8);
                for i in 0..length as usize / 8 {
                    values.push(u64::from_le_bytes([
                        buffer[i * 8],
                        buffer[i * 8 + 1],
                        buffer[i * 8 + 2],
                        buffer[i * 8 + 3],
                        buffer[i * 8 + 4],
                        buffer[i * 8 + 5],
                        buffer[i * 8 + 6],
                        buffer[i * 8 + 7],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_UI8(values))
            }
            0x101E => {
                let _length = reader.read_u32::<LittleEndian>()?;

                let mut strings = Vec::with_capacity(_length as usize);
                // reading a vector of VT_LPSTRs
                for _ in 0.._length {
                    let length = reader.read_u32::<LittleEndian>()?;
                    let mut buffer = vec![0; length as usize];
                    reader.read_exact(&mut buffer)?;
                    let mut u16_buffer = vec![0; length as usize / 2];
                    for i in 0..length as usize / 2 {
                        u16_buffer[i] = u16::from_be_bytes([buffer[i * 2], buffer[i * 2 + 1]]);
                    }
                    strings.push(String::from_utf16(&u16_buffer).unwrap());
                }

                Ok(NormalPropertyType::VT_VECTOR_VT_LPSTR(strings))
            }
            0x101F => {
                let num_strings = reader.read_u32::<LittleEndian>()?;
                let mut values = Vec::with_capacity(num_strings as usize);
                for _ in 0..num_strings {
                    let length = reader.read_u32::<LittleEndian>()?;
                    let mut buffer = vec![0; length as usize];
                    reader.read_exact(&mut buffer)?;
                    values.push(String::from_utf8(buffer).unwrap());
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_LPWSTR(values))
            }
            0x1040 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 8);
                for i in 0..length as usize / 8 {
                    values.push(u64::from_le_bytes([
                        buffer[i * 8],
                        buffer[i * 8 + 1],
                        buffer[i * 8 + 2],
                        buffer[i * 8 + 3],
                        buffer[i * 8 + 4],
                        buffer[i * 8 + 5],
                        buffer[i * 8 + 6],
                        buffer[i * 8 + 7],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_FILETIME(values))
            }
            0x1047 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut buffer = vec![0; length as usize];
                reader.read_exact(&mut buffer)?;
                let mut values = Vec::with_capacity(length as usize / 4);
                for i in 0..length as usize / 4 {
                    values.push(u32::from_le_bytes([
                        buffer[i * 4],
                        buffer[i * 4 + 1],
                        buffer[i * 4 + 2],
                        buffer[i * 4 + 3],
                    ]));
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_CF(values))
            }
            0x1048 => {
                let length = reader.read_u32::<LittleEndian>()?;
                let mut clsids = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    let mut buffer = [0; 16];
                    reader.read_exact(&mut buffer)?;
                    clsids.push(buffer);
                }
                Ok(NormalPropertyType::VT_VECTOR_VT_CLSID(clsids))
            }
            _ => Ok(NormalPropertyType::Unknown(type_val)),
        }
    }
}

impl FromReader for DictionaryPropertyType {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let c_entries = reader.read_u32::<LittleEndian>()?;

        let mut dict_map: Vec<(String, u32)> = Vec::with_capacity(c_entries as usize);

        for _ in 0..c_entries {
            // this should really depend on the CodePage property
            let property_id = reader.read_u32::<LittleEndian>()?;

            let length = reader.read_u32::<LittleEndian>()?;

            let mut buffer = vec![0; length as usize];
            reader.read_exact(&mut buffer)?;

            let key = String::from_utf8(buffer).unwrap();
            dict_map.push((key, property_id));
        }

        Ok(DictionaryPropertyType {
            dictionary: dict_map,
        })
    }
}

impl<T: FromCStruct> PLCF<T> {
    fn from_bytes(bytes: &[u8]) -> Self {
        let cb = bytes.len();
        #[allow(non_snake_case)]
        let iMac = (cb - 4) / (4 + T::c_size());

        // Make first array of the Plex
        let mut rgfc = Vec::with_capacity(iMac + 1);
        for i in 0..=iMac {
            let offset = i * 4;
            let fc = i32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            rgfc.push(fc);
        }

        // Make second array structure[T] of the Plex
        let mut rgstruct = Vec::with_capacity(iMac);
        let regstruct_offset = 4 * (iMac + 1);
        for i in 0..iMac {
            let offset = regstruct_offset + i * T::c_size();
            let struct_bytes = &bytes[offset..offset + T::c_size()];
            let struct_val = T::from_bytes(struct_bytes);
            rgstruct.push(struct_val);
        }

        PLCF { rgfc, rgstruct }
    }
}

impl FromCStruct for PCD {
    fn c_size() -> usize {
        8
    }

    #[allow(non_snake_case)]
    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 8);
        let bitfield = u16::from_be_bytes([bytes[0], bytes[1]]);

        let fNoParaLast = (bitfield & 0x8000) == 0x8000;
        let fPaphNil = (bitfield & 0x4000) == 0x4000;
        let fDirty = (bitfield & 0x2000) == 0x2000;
        let _ = bitfield & 0x1000;
        let fn_val = bitfield & 0x0FFF; // Dont know if this correct

        let fc = i32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        let prm = u16::from_be_bytes([bytes[6], bytes[7]]); // Unprocessed

        PCD {
            fNoParaLast,
            rest_of_bitfield: [fPaphNil, fDirty, false],
            fn_val,
            fc,
            prm,
        }
    }
}

impl FromCStruct for RgIdOffset {
    fn c_size() -> usize {
        20
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 20);
        let mut format_id = [0 as u8; 16];
        format_id.copy_from_slice(&bytes[0..16]);
        let section_offset = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);

        RgIdOffset {
            formatID: format_id,
            sectionOffset: section_offset,
        }
    }
}

impl PropertyIdentifier {
    fn from_u32(val: u32) -> Self {
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
