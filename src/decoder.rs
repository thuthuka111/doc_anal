use cfb::CompoundFile;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
};

use crate::model::*;

#[allow(unused)]
#[derive(Debug)]
pub struct WordDocument {
    cfb: CompoundFile<File>,
    pub fib: Fib,
    pub stylesheet: SHSHI,
    pub piece_table: PLCF<PCD>,
}

// endregion: Structs

impl WordDocument {
    pub fn read_file(file: File) -> io::Result<Self> {
        let mut cfb = CompoundFile::open(file)?;
        let mut word_doc_stream = cfb.open_stream("WordDocument")?;

        let fib = Fib::from_reader(&mut word_doc_stream)?;
        println!("{:?}", fib);

        // Determine which table stream to use
        let table_stream_name = if !fib.fWhichTblStm {
            "0Table"
        } else {
            "1Table"
        };
        let mut table_stream = cfb.open_stream(table_stream_name)?;

        // Read the complex part of the document
        let (_grppls, piece_table) = {
            let mut complex_buff = vec![0; fib.lcbClx as usize];
            table_stream.seek(SeekFrom::Start(fib.fcClx as u64))?;
            table_stream.read_exact(&mut complex_buff)?;

            let mut complex_buff = Cursor::new(complex_buff);
            let complex_part = (vec![()], PLCF::<PCD>::from_reader(&mut complex_buff)?);
            println!("{:#?}", complex_part.1);

            complex_part
        };

        // Read the Stylesheet
        let stylesheet = {
            // NOTE: potentially break if nFib is less than 67 as STSHI format is different

            let mut stsh_buffer = vec![0; fib.lcbStshf as usize];
            table_stream.seek(SeekFrom::Start(fib.fcStshf as u64))?;
            table_stream.read_exact(&mut stsh_buffer)?;
            println!("Lenght of STSH: {}", stsh_buffer.len());

            let mut stsh_buffer = BufReader::new(Cursor::new(stsh_buffer));
            let stylesheet = SHSHI::from_reader(&mut stsh_buffer)?;

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
        Ok(WordDocument {
            cfb,
            fib,
            stylesheet,
            piece_table,
        })
    }

    pub fn print_cfb_structure(&self) {
        let entries = self.cfb.walk();
        for entry in entries {
            if entry.is_stream() {
                println!("Stream: {}", entry.name());
            } else {
                println!("Storage: {}", entry.name());
            }
        }
    } 

}

/// Read teh text Section of the document
#[allow(non_snake_case)]
fn _read_text<R: Read + Seek>(fib: &Fib, reader: &mut R) -> io::Result<()> {
    if !fib.fComplex {
        let mut main_text = vec![0; fib.ccpText as usize];
        reader.seek(SeekFrom::Start(fib.fcMin as u64))?;
        reader.read_exact(&mut main_text)?;

        let mut footnote = vec![0; fib.ccpFtn as usize];
        reader.seek(SeekFrom::Start((fib.fcMin + fib.ccpText) as u64))?;
        reader.read_exact(&mut footnote)?;

        let mut header = vec![0; fib.ccpHdr as usize];
        reader.seek(SeekFrom::Start(
            (fib.fcMin + fib.ccpText + fib.ccpFtn) as u64,
        ))?;
        reader.read_exact(&mut header)?;

        let mut annotation = vec![0; fib.ccpAtn as usize];
        reader.seek(SeekFrom::Start(
            (fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr) as u64,
        ))?;
        reader.read_exact(&mut annotation)?;

        let endnote_length = fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpAtn
            - (fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpEdn);
        let mut endnote = vec![0; endnote_length as usize];
        reader.seek(SeekFrom::Start(
            (fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpAtn) as u64,
        ))?;
        reader.read_exact(&mut endnote)?;

        let textbox_length =
            fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpAtn + fib.ccpEdn
                - (fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpEdn + fib.ccpTxbx);
        let mut textbox = vec![0; textbox_length as usize];
        reader.seek(SeekFrom::Start(
            (fib.fcMin + fib.ccpText + fib.ccpFtn + fib.ccpHdr + fib.ccpAtn + fib.ccpEdn) as u64,
        ))?;
        reader.read_exact(&mut textbox)?;

        let header_textbox_length = fib.fcMin
            + fib.ccpText
            + fib.ccpFtn
            + fib.ccpHdr
            + fib.ccpAtn
            + fib.ccpEdn
            + fib.ccpTxbx
            - (fib.fcMin
                + fib.ccpText
                + fib.ccpFtn
                + fib.ccpHdr
                + fib.ccpEdn
                + fib.ccpTxbx
                + fib.ccpHrdTxbx);
        let mut header_textbox = vec![0; header_textbox_length as usize];
        reader.seek(SeekFrom::Start(
            (fib.fcMin
                + fib.ccpText
                + fib.ccpFtn
                + fib.ccpHdr
                + fib.ccpAtn
                + fib.ccpEdn
                + fib.ccpTxbx) as u64,
        ))?;
        reader.read_exact(&mut header_textbox)?;

        todo!();
    } else {
        todo!();
    }
}

