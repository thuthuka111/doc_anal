use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

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
