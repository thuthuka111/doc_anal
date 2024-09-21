use cfb::CompoundFile;
use from_reader::FromReader;
use json::JsonValue;
pub use model::*;
use std::{
    cell::RefCell,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    vec,
};

mod from_c_struct;
mod from_reader;
mod model;
mod to_structure;

#[allow(unused)]
#[derive(Debug)]
pub struct WordDocument {
    pub cfb: RefCell<CompoundFile<File>>,
    pub fib: Fib,
    pub stylesheet: SHSHI,
    pub piece_table: PLCF<PCD>,
    pub list_tables: LSTs,
    pub table_stream_name: String,
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
        let list_tables = {
            // making a set of bytes to read and write from
            // let list_table_buffer = vec![0u8; 71];
            // let mut list_table_buffer = Cursor::new(list_table_buffer);

            // // iStartAt
            // list_table_buffer.write(&1i32.to_le_bytes())?;

            // // nfc
            // list_table_buffer.write(&0x02u8.to_le_bytes())?;

            // // bitfield
            // list_table_buffer.write(&0u8.to_le_bytes())?;

            // // rgbxchNums
            // list_table_buffer.write(&1u8.to_le_bytes())?;
            // for _ in 0..8 {
            //     list_table_buffer.write(&0u8.to_le_bytes())?;
            // }
            // // ixchFollow0x03
            // list_table_buffer.write(&0x03u8.to_le_bytes())?;
            // list_table_buffer.write(&0i32.to_le_bytes())?;
            // list_table_buffer.write(&0u32.to_le_bytes())?;
            // list_table_buffer.write(&0x0Du8.to_le_bytes())?;
            // list_table_buffer.write(&0x18u8.to_le_bytes())?;
            // // ilvlRestartLim
            // list_table_buffer.write(&0x00u8.to_le_bytes())?;
            // list_table_buffer.write(&0u8.to_le_bytes())?;

            // for _ in 0..0x18 {
            //     list_table_buffer.write(&0u8.to_le_bytes())?;
            // }

            // for _ in 0..0xD {
            //     list_table_buffer.write(&0u8.to_le_bytes())?;
            // }
            // //"\0x0000." as bytes
            // list_table_buffer.write(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?;

            // // Second LVL

            // // putting curser back to the start
            // list_table_buffer.seek(SeekFrom::Start(0))?;

            // something has gone awry here picbile the length of the PLcfLSt does not incude the LVL structures that come afterwards
            // calculating this to at least not run in to the next section, as the lcbPlcfLst is too small
            let distance_to_plf_lfo = fib.fcPlfLfo - fib.fcPlcfLst;

            let mut list_table_buffer = vec![0; distance_to_plf_lfo as usize];
            table_stream.seek(SeekFrom::Start(fib.fcPlcfLst as u64))?;
            table_stream.read_exact(&mut list_table_buffer)?;
            dbg!(fib.fcPlcfLst);
            dbg!(fib.lcbPlcfLst);
            // dbg!((fib.fcPlfLfo, fib.lcbPlfLfo));
            println!(
                "bytes between fcPlcfLst and fcPlfLfo: {}",
                fib.fcPlfLfo - fib.fcPlcfLst
            );
            // println!("bytes remaining in table stream: {}", table_stream.bytes().count());

            let mut list_table_buffer = BufReader::new(Cursor::new(list_table_buffer));
            // Dont do this witha `from_reader`
            let list_tables = LSTs::from_reader(&mut list_table_buffer)?;
            // todo!();

            // println!("List Tables: {:#?}", list_tables);
            list_tables
        };

        // Read sttbListNames if there are any

        // Read the LFO records (List Format Override) if any

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
            cfb: RefCell::new(cfb),
            fib,
            stylesheet,
            piece_table,
            table_stream_name: table_stream_name.to_string(),
            list_tables,
            document_summary_information_stream,
            summary_information,
        })
    }

    pub fn to_json_logical(&self) -> JsonValue {
        let fib = Structure::from("Fib", &self.fib);
        let stylesheet = Structure::from("StyleSheet", &self.stylesheet);
        let document_summary_information_stream = Structure::from(
            "Document Summary Information Stream",
            &self.document_summary_information_stream,
        );

        JsonValue::from(vec![fib, stylesheet, document_summary_information_stream])
    }

    /// Returns an array of the json value used fo rthe physical Table analysis in the frontend
    pub fn to_json_physical(&self) -> JsonValue {
        let fib = &self.fib;

        let mut output = Vec::new();
        let mut word_doc_stream = self.cfb.borrow_mut().open_stream("WordDocument").unwrap();

        let fib_header_bytes =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 0, 72, "WordDocument")
                .description(
                    "Fib Header bytes; conttains varaibles like product versionand word version",
                )
                .structure_name("Fib 1997 Header");
        let next_fib_section =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 72, 402, "WordDocument")
                .description("variables ccpText - lcbWss of the Fib")
                .structure_name("Fib 1997 cont.");
        let next_fib_section_2 =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 402, 898, "WordDocument")
                .description("variables fcDop - lcbSttbfUser of the Fib. End of Word 97 Fib")
                .structure_name("Fib 1997 cont.");
        let word_2000_defs =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 898, 1034, "WordDocument")
                .description("Word 2000 definitions. Variables fcPlcTch - lcbPlcfpgp")
                .structure_name("Fib 2000 ext.");
        let word_2002_defs =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 1034, 1242, "WordDocument")
                .description("Word 2002 definitions. Variables fcPlcfuim - lcbPlcflvcMixedXP")
                .structure_name("Fib 2002 ext.");
        let word_2003_defs =
            PhysicalStructure::from_reader_range(&mut word_doc_stream, 1242, 1258, "WordDocument")
                .description("Word 2003 definitions. Variables fcHplxsdr - cQuickSavesNew")
                .structure_name("Fib 2003 ext.");

        output.extend(vec![
            fib_header_bytes,
            next_fib_section,
            next_fib_section_2,
            word_2000_defs,
            word_2002_defs,
            word_2003_defs,
        ]); // Consider annotating these all as FIB as `(partial FIB)`

        let mut table_stream = self
            .cfb
            .borrow_mut()
            .open_stream(&self.table_stream_name)
            .unwrap();

        let text_section = PhysicalStructure::from_reader_range(
            &mut table_stream,
            fib.fcMin as u64,
            fib.fcMin as u64,
            "Table Stream",
        )
        .description("first character to alst character of the regular text section");
        output.push(text_section);

        let stylesheet = PhysicalStructure::from_reader_range(
            &mut table_stream,
            fib.fcStshf as u64,
            (fib.fcStshf + fib.lcbStshf as i32) as u64,
            "Table Stream",
        )
        .description("STSHI(Stylesheet) structure");
        output.push(stylesheet);

        let footnote_run = Self::get_offset_and_count(&mut word_doc_stream, 0x00AA);
        output.push(
            PhysicalStructure::from_reader_range(
                &mut word_doc_stream,
                footnote_run.0,
                footnote_run.0 + footnote_run.1,
                "WordDocument",
            )
            .description("Footnote Reference Structure"),
        );

        output.into()
    }

    /// helper function to read undecoded fib variables from the Word document stream
    /// ## Note
    /// assumes that first 4 bytes read is the fcOffset and the next 4 bytes is the count of bytes
    fn get_offset_and_count<R: Read + Seek>(reader: &mut R, fc_offset: usize) -> (u64, u64) {
        let mut buffer = [0u8; 8];
        reader.seek(SeekFrom::Start(fc_offset as u64)).unwrap();
        reader.read_exact(&mut buffer).unwrap();
        let fc_offset = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let count = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        (fc_offset as u64, count as u64)
    }

    pub fn print_cfb_structure(&self) {
        let cfb = self.cfb.borrow();
        let entries = cfb.walk();
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
