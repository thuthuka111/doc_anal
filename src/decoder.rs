use cfb::CompoundFile;
use json::JsonValue;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    vec,
};

use crate::model::*;

#[allow(unused)]
#[derive(Debug)]
pub struct WordDocument {
    pub cfb: CompoundFile<File>,
    pub fib: Fib,
    pub stylesheet: SHSHI,
    pub piece_table: PLCF<PCD>,
    pub document_summary_information_stream: DocumentSummaryInfoStream,
    pub summary_information: SummaryInformation,
}

// endregion: Structs

impl WordDocument {
    pub fn read_file(file: File) -> io::Result<Self> {
        let mut cfb = CompoundFile::open(file)?;
        let mut word_doc_stream = cfb.open_stream("WordDocument")?;

        let document_summary_information_stream = {
            let mut doc_sum_info_stream = cfb.open_stream("\x05DocumentSummaryInformation")?;

            let doc_sum_info = DocumentSummaryInfoStream::from_reader(&mut doc_sum_info_stream)?;
            doc_sum_info
        };

        let summary_information = {
            let mut summary_info_stream = cfb.open_stream("\x05SummaryInformation")?;

            let data = [
                0xFE, 0xFF, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0xE0, 0x85, 0x9F, 0xF2, 0xF9, 0x4F, 0x68, 0x10, 0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27,
                0xB3, 0xD9, 0x30, 0x00, 0x00, 0x00, 0x8C, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x98, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xA0, 0x00,
                0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0xB8, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
                0xC4, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xD0, 0x00, 0x00, 0x00, 0x06, 0x00,
                0x00, 0x00, 0xDC, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0xE8, 0x00, 0x00, 0x00,
                0x08, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x10, 0x01,
                0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x1C, 0x01, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00,
                0x3C, 0x01, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x48, 0x01, 0x00, 0x00, 0x0C, 0x00,
                0x00, 0x00, 0x54, 0x01, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0x60, 0x01, 0x00, 0x00,
                0x0E, 0x00, 0x00, 0x00, 0x6C, 0x01, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x74, 0x01,
                0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x7C, 0x01, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
                0x84, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xE4, 0x04, 0x00, 0x00, 0x1E, 0x00,
                0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x4A, 0x6F, 0x65, 0x27, 0x73, 0x20, 0x64, 0x6F,
                0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x04, 0x00,
                0x00, 0x00, 0x4A, 0x6F, 0x62, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
                0x4A, 0x6F, 0x65, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x1E, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x4E, 0x6F, 0x72, 0x6D, 0x61, 0x6C,
                0x2E, 0x64, 0x6F, 0x74, 0x6D, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00,
                0x43, 0x6F, 0x72, 0x6E, 0x65, 0x6C, 0x69, 0x75, 0x73, 0x00, 0x00, 0x00, 0x1E, 0x00,
                0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x36, 0x36, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00,
                0x18, 0x00, 0x00, 0x00, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20,
                0x4F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x20, 0x57, 0x6F, 0x72, 0x64, 0x00, 0x00, 0x00,
                0x40, 0x00, 0x00, 0x00, 0x00, 0x6E, 0xD9, 0xA2, 0x42, 0x00, 0x00, 0x00, 0x40, 0x00,
                0x00, 0x00, 0x00, 0x16, 0xD0, 0xA1, 0x4E, 0x8E, 0xC6, 0x01, 0x40, 0x00, 0x00, 0x00,
                0x00, 0x1C, 0xF2, 0xD5, 0x2A, 0xCE, 0xC6, 0x01, 0x40, 0x00, 0x00, 0x00, 0x00, 0x3C,
                0xDC, 0x73, 0xDD, 0x80, 0xC8, 0x01, 0x03, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00,
                0x03, 0x00, 0x00, 0x00, 0xE5, 0x0D, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x38, 0x4F,
                0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];

            let mut _summary_info_stream = Cursor::new(data);
            let summary_info = SummaryInformation::from_reader(&mut summary_info_stream)?;
            summary_info
        };

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
            // println!("{:#?}", complex_part.1);

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

        // Read the List Tables
        // let list_tables = {
        //     let mut list_table_buffer = vec![0; fib.lcbPlcfLst as usize];
        //     table_stream.seek(SeekFrom::Start(fib.fcPlcfLst as u64))?;
        //     table_stream.read_exact(&mut list_table_buffer)?;

        //     let mut list_table_buffer = BufReader::new(Cursor::new(list_table_buffer));
        //     // Dont do this witha `from_reader`
        //     let list_tables = LSTs::from_reader(&mut list_table_buffer)?;

        //     list_tables
        // };

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
            document_summary_information_stream,
            summary_information,
        })
    }

    pub fn to_json(&self) -> JsonValue {
        let fib = Structure::from("Fib", &self.fib);
        let stylesheet = Structure::from("StyleSheet", &self.stylesheet);

        JsonValue::from(vec![fib, stylesheet])
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
