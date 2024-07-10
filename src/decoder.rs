use byteorder::{LittleEndian, ReadBytesExt};
use cfb::CompoundFile;
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

// region: Structs

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
    // Add other FIB fields as needed
}

#[derive(Debug)]
pub struct WordDocument {
    pub fib: Fib,
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

        // println!(
        //     "Text Section (raw bytes): {:?}",
        //     hex::encode_upper(&text_buffer[..])
        // );

        // println!("fxMin in Hex: {:X}", fib.fcMin);

        // Determine which table stream to use
        let table_stream_name = if !fib.fWhichTblStm {
            "0Table"
        } else {
            "1Table"
        };
        let mut table_stream = cfb.open_stream(table_stream_name)?;

        // Read the PlcfbtePapx
        let mut plcfbte_papx_buffer = vec![0; fib.lcbPlcfbtePapx as usize];
        table_stream.seek(SeekFrom::Start(fib.fcPlcfbtePapx as u64))?;
        table_stream.read_exact(&mut plcfbte_papx_buffer)?;

        // println!(
        //     "PlcfbtePapx (raw bytes): {:?}",
        //     hex::encode_upper(&plcfbte_papx_buffer[..])
        // );

        Ok(WordDocument { fib })
    }
}

#[allow(non_snake_case)]
pub fn read_fib<R: Read + Seek>(reader: &mut R) -> io::Result<Fib> {
    reader.seek(SeekFrom::Start(0))?;

    let wIdent = reader.read_u16::<LittleEndian>()?;
    let nFib = reader.read_u16::<LittleEndian>()?;

    reader.seek(SeekFrom::Start(0x0A))?;
    let bitfield = reader.read_u16::<LittleEndian>()?;
    // println!("That bitfield: '{:16b}'", bitfield);
    // println!("Some value eE: '{:16b}'", 0x0200);
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
    })
}
