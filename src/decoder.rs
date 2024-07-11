use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use cfb::CompoundFile;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
};

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
    pub ccpText: i32,
    /// File offset of the STSH (Stylesheet) in the table stream
    pub fcStshf: i32,
    /// Count of bytes of the STSH allocation
    pub lcbStshf: u32,
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

#[allow(unused)]
#[derive(Debug)]
pub struct WordDocument {
    pub fib: Fib,
    pub stylesheet: SHSHI,
    // pub text: Vec<u8>,
}

// endregion: Structs

impl WordDocument {
    pub fn read_file(file: File) -> io::Result<Self> {
        let mut cfb = CompoundFile::open(file)?;
        let mut word_doc_stream = cfb.open_stream("WordDocument")?;

        let fib = read_fib(&mut word_doc_stream)?;

        let mut text_buffer = vec![0; fib.ccpText as usize];
        word_doc_stream.seek(SeekFrom::Start(fib.fcMin as u64))?;
        word_doc_stream.read_exact(&mut text_buffer)?;

        // Text Section Una
        println!(
            "Text Section (raw bytes): {:?}",
            hex::encode_upper(&text_buffer[..])
        );

        // Determine which table stream to use
        let table_stream_name = if !fib.fWhichTblStm {
            "0Table"
        } else {
            "1Table"
        };
        let mut table_stream = cfb.open_stream(table_stream_name)?;

        // Read the Stylesheet
        let stylesheet = {
            // NOTE: potentially break if nFib is less than 67 as STSHI format is different

            let mut stsh_buffer = vec![0; fib.lcbStshf as usize];
            table_stream.seek(SeekFrom::Start(fib.fcStshf as u64))?;
            table_stream.read_exact(&mut stsh_buffer)?;
            println!("Lenght of STSH: {}", stsh_buffer.len());

            let mut stsh_buffer = BufReader::new(Cursor::new(stsh_buffer));
            let stylesheet = read_stylesheet(&mut stsh_buffer)?;

            let left_bytes = stsh_buffer.fill_buf()?;
            assert_eq!(left_bytes.len(), 0);

            stylesheet
        };

        // Read the PlcfbtePapx
        {
            let mut plcfbte_papx_buffer = vec![0; fib.lcbPlcfbtePapx as usize];
            table_stream.seek(SeekFrom::Start(fib.fcPlcfbtePapx as u64))?;
            table_stream.read_exact(&mut plcfbte_papx_buffer)?;
            // println!(
            //     "PlcfbtePapx (raw bytes): {:?}",
            //     hex::encode_upper(&plcfbte_papx_buffer[..])
            // );
        }
        Ok(WordDocument { fib, stylesheet })
    }
}

#[allow(non_snake_case)]
fn read_fib<R: Read + Seek>(reader: &mut R) -> io::Result<Fib> {
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

    reader.seek(SeekFrom::Start(0x00A2))?;
    let fcStshf = reader.read_i32::<LittleEndian>()?;
    let lcbStshf = reader.read_u32::<LittleEndian>()?;

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
        fcStshf,
        lcbStshf,
    })
}

#[allow(non_snake_case)]
fn read_stylesheet<R: Read + Seek>(stsh_buffer: &mut R) -> io::Result<SHSHI> {
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

        let stylesheet_std = read_stylesheet_std(&mut std_buffer)?;

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

#[allow(non_snake_case)]
fn read_stylesheet_std<R: Read + Seek>(std_buffer: &mut R) -> io::Result<STD> {
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
