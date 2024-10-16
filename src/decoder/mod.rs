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
            if fib.fcPlcfLst == 0 {
                // there is no LST
                LSTs {
                    num_LSTs: 0,
                    LSTs: vec![],
                }
            } else {
                let distance_to_plf_lfo = fib.fcPlfLfo - fib.fcPlcfLst;

                let mut list_table_buffer = vec![0; distance_to_plf_lfo as usize];
                table_stream.seek(SeekFrom::Start(fib.fcPlcfLst as u64))?;
                table_stream.read_exact(&mut list_table_buffer)?;
                dbg!(fib.fcPlcfLst);
                dbg!(fib.lcbPlcfLst);
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
            }
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

    pub fn get_logical_structures(&self) -> Vec<Structure> {
        let fib = Structure::from("Fib", &self.fib);
        let stylesheet = Structure::from("StyleSheet", &self.stylesheet);
        let peice_tables = Structure::from("Piece Tables", &self.piece_table);
        let document_summary_information_stream = Structure::from(
            "Document Summary Information",
            &self.document_summary_information_stream,
        );
        let summary_information_stream =
            Structure::from("Summary Information", &self.summary_information);

        vec![
            fib,
            stylesheet,
            peice_tables,
            document_summary_information_stream,
            summary_information_stream,
        ]
    }

    pub fn to_json_logical(&self) -> JsonValue {
        JsonValue::from(self.get_logical_structures())
    }

    pub fn get_physical_sructures(&self) -> Vec<PhysicalStructure> {
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

        let main_text_section = PhysicalStructure::from_reader_range(
            &mut word_doc_stream,
            fib.fcMin as u64,
            fib.fcMin as u64 * fib.ccpText as u64,
            "Table Stream",
        )
        .description("first character to alst character of the regular text section");
        output.push(main_text_section);

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
                &mut table_stream,
                footnote_run.0,
                footnote_run.0 + footnote_run.1,
                "WordDocument",
            )
            .description("Footnote Reference Structure"),
        );

        output
    }

    /// Returns an array of the json value used fo rthe physical Table analysis in the frontend
    pub fn to_json_physical(&self) -> JsonValue {
        self.get_physical_sructures().into()
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

    pub fn compare_to_physical(&self, other_word_doc: &WordDocument) -> JsonValue {
        let reference_physical_strucutres = self.get_physical_sructures();
        let other_physical_strucutres = other_word_doc.get_physical_sructures();

        let difference_indeces = reference_physical_strucutres
            .iter()
            .zip(other_physical_strucutres.iter())
            .map(|(ref_structure, other_structure)| {
                let differences = compute_physical_differences(
                    ref_structure.bytes.clone(),
                    other_structure.bytes.clone(),
                );
                ComparisonPhysicalStructure {
                    ref_structure,
                    comp_structure: other_structure,
                    difference_indices: differences,
                }
            })
            .collect::<Vec<_>>();

        difference_indeces.into()
    }

    pub fn compare_to_logical(&self, other_word_doc: &WordDocument) -> JsonValue {
        let reference_logical_structures = self.get_logical_structures();
        let other_logical_strucutres = other_word_doc.get_logical_structures();

        let mut logical_structures: Vec<ComparisonLogicalStructure> = Vec::new();

        for (ref_structure, other_structure) in reference_logical_structures
            .iter()
            .zip(other_logical_strucutres.iter())
        {
            assert_eq!(ref_structure.name, other_structure.name);
            let bool_arr = compute_structure_item_differences(
                &ref_structure.structure,
                &other_structure.structure,
            );
            let substructure_arr = match (
                ref_structure.substructs.as_ref(),
                other_structure.substructs.as_ref(),
            ) {
                (Some(ref_substructures), Some(other_substructures)) => {
                    compute_subsctructure_differences(ref_substructures, other_substructures)
                }
                (None, Some(other_substructures)) => {
                    let mut substructure_arr = Vec::new();
                    for other_substructure in other_substructures.iter() {
                        substructure_arr.push(ComparisonLogicalStructure {
                            ref_structure: None,
                            comp_structure: Some(other_substructure),
                            structure_differences: vec![false; other_substructure.structure.len()],
                            substructure_differences: Vec::new(),
                        });
                    }
                    substructure_arr
                }
                (Some(ref_substructures), None) => {
                    let mut substructure_arr = Vec::new();
                    for ref_substructure in ref_substructures.iter() {
                        substructure_arr.push(ComparisonLogicalStructure {
                            ref_structure: Some(ref_substructure),
                            comp_structure: None,
                            structure_differences: vec![false; ref_substructure.structure.len()],
                            substructure_differences: Vec::new(),
                        });
                    }
                    substructure_arr
                }
                (None, None) => Vec::new(),
            };

            logical_structures.push(ComparisonLogicalStructure {
                ref_structure: Some(other_structure),
                comp_structure: Some(ref_structure),
                structure_differences: bool_arr,
                substructure_differences: substructure_arr,
            });
        }

        logical_structures.into()
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

/// Read the text Section of the document
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

fn compute_physical_differences(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<(usize, usize)> {
    // map to reflect the nibbles
    let mut nibble_vec_1 = Vec::with_capacity(vec1.len() * 2);
    let mut nibble_vec_2 = Vec::with_capacity(vec2.len() * 2);

    for byte in vec1.iter() {
        nibble_vec_1.push(byte >> 4);
        nibble_vec_1.push(byte & 0x0F);
    }
    for byte in vec2.iter() {
        nibble_vec_2.push(byte >> 4);
        nibble_vec_2.push(byte & 0x0F);
    }

    let vec1 = nibble_vec_1;
    let vec2 = nibble_vec_2;

    let mut differences = Vec::new();
    let mut start: Option<usize> = None;

    for (i, (&byte1, &byte2)) in vec1.iter().zip(vec2.iter()).enumerate() {
        if byte1 != byte2 {
            // If start is None, this is the beginning of a new difference
            if start.is_none() {
                start = Some(i);
            }
        } else {
            // If the bytes are the same and we are tracking a difference, close the range
            if let Some(s) = start {
                differences.push((s, i));
                start = None;
            }
        }
    }

    // compute difference in size of the two vectors and use for the last difference
    if vec1.len() > vec2.len() {
        differences.push((vec2.len(), vec1.len()));
    } else if vec2.len() > vec1.len() {
        differences.push((vec1.len(), vec2.len()));
    }

    // Handle the case where the last part of the vectors differs
    if let Some(s) = start {
        differences.push((s, vec1.len()));
    }

    differences
}

fn compute_structure_item_differences(
    items: &Vec<StructureItem>,
    other_items: &Vec<StructureItem>,
) -> Vec<bool> {
    let mut differences = Vec::new();

    let mut items_iter = items.iter();
    let mut other_items_iter = other_items.iter();

    while let (Some(item), Some(other_item)) = (items_iter.next(), other_items_iter.next()) {
        assert_eq!(
            item.name, other_item.name,
            "Structure items should really be the same"
        );
        if item.value == other_item.value {
            differences.push(false);
        } else {
            differences.push(true);
        }
    }

    differences
}

fn compute_subsctructure_differences<'a, 'b>(
    substructures: &'a Vec<Structure>,
    other_substructures: &'b Vec<Structure>,
) -> Vec<ComparisonLogicalStructure<'a, 'b>> {
    let mut differences = Vec::new();

    let mut compared_other_structures = Vec::new();

    for substructure in substructures.iter() {
        let other_structure_matching_name = other_substructures
            .iter()
            .enumerate()
            .find(|(_, s)| s.name == substructure.name);
        match other_structure_matching_name {
            Some((index, other_structure)) => {
                let structure_differeces = compute_structure_item_differences(
                    &substructure.structure,
                    &other_structure.structure,
                );

                differences.push(ComparisonLogicalStructure {
                    ref_structure: Some(&substructure),
                    comp_structure: Some(&other_structure),
                    structure_differences: structure_differeces,
                    substructure_differences: Vec::new(),
                });
                compared_other_structures.push(index);
            }
            None => {
                differences.push(ComparisonLogicalStructure {
                    ref_structure: Some(substructure),
                    comp_structure: None,
                    structure_differences: vec![false; substructure.structure.len()],
                    substructure_differences: Vec::new(),
                });
            }
        }
    }

    for (_, other_structure) in other_substructures
        .iter()
        .enumerate()
        .filter(|(i, _)| !compared_other_structures.contains(i))
    {
        differences.push(ComparisonLogicalStructure {
            ref_structure: None,
            comp_structure: Some(other_structure),
            structure_differences: vec![false; other_structure.structure.len()],
            substructure_differences: Vec::new(),
        });
    }

    differences
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_physical_compute_differences() {
        let vec1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let vec2 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10];
        let differences = compute_physical_differences(vec1, vec2);
        assert_eq!(differences, vec![(9, 10)]);
    }

    #[test]
    fn test_logical_differences() {
        let test_structure = Structure {
            name: "STHI".to_string(),
            structure: vec![
                StructureItem {
                    name: "cstd".to_string(),
                    value: "0x00000001".to_string(),
                    description: None,
                },
                StructureItem {
                    name: "cbSTDBaseInX".to_string(),
                    value: "0x00000000".to_string(),
                    description: None,
                },
                StructureItem {
                    name: "cbSTDBaseInY".to_string(),
                    value: "0x00000000".to_string(),
                    description: None,
                },
            ],
            substructs: None,
        };

        let test_other_strucure = Structure {
            name: "STHI".to_string(),
            structure: vec![
                StructureItem {
                    name: "cstd".to_string(),
                    value: "0x00000001".to_string(),
                    description: None,
                },
                StructureItem {
                    name: "cbSTDBaseInX".to_string(),
                    value: "0x00000000".to_string(),
                    description: None,
                },
                StructureItem {
                    name: "cbSTDBaseInY".to_string(),
                    value: "0x00000001".to_string(),
                    description: None,
                },
            ],
            substructs: None,
        };

        let differences = compute_structure_item_differences(
            &test_structure.structure,
            &test_other_strucure.structure,
        );
        assert_eq!(differences, vec![false, false, true]);
    }

    #[test]
    fn test_substructure_logical() {
        let styles_1 = vec![
            Structure {
                name: "Normal,DO NOT USE,n".to_string(),
                structure: vec![
                    StructureItem {
                        name: "std".to_string(),
                        value: "0".to_string(),
                        description: None,
                    },
                    StructureItem {
                        name: "fScratch".to_string(),
                        value: "false".to_string(),
                        description: None,
                    },
                ],
                substructs: None,
            },
            Structure {
                name: "Heading 1,H1".to_string(),
                structure: vec![
                    StructureItem {
                        name: "std".to_string(),
                        value: "1".to_string(),
                        description: None,
                    },
                    StructureItem {
                        name: "fScratch".to_string(),
                        value: "false".to_string(),
                        description: None,
                    },
                ],
                substructs: None,
            },
        ];

        let styles_2 = vec![
            Structure {
                name: "Normal,DO NOT USE,n".to_string(),
                structure: vec![
                    StructureItem {
                        name: "std".to_string(),
                        value: "65".to_string(),
                        description: None,
                    },
                    StructureItem {
                        name: "fScratch".to_string(),
                        value: "false".to_string(),
                        description: None,
                    },
                ],
                substructs: None,
            },
            Structure {
                name: "Heading 2,H2".to_string(),
                structure: vec![
                    StructureItem {
                        name: "std".to_string(),
                        value: "2".to_string(),
                        description: None,
                    },
                    StructureItem {
                        name: "fScratch".to_string(),
                        value: "false".to_string(),
                        description: None,
                    },
                ],
                substructs: None,
            },
        ];

        let differences = compute_subsctructure_differences(&styles_1, &styles_2);
        assert_eq!(differences.len(), 3);
        assert_eq!(differences[0].structure_differences, vec![true, false]);

        assert!(differences[1].ref_structure.is_some());
        assert!(differences[1].comp_structure.is_none());

        assert!(differences[2].ref_structure.is_none());
        assert!(differences[2].comp_structure.is_some());
    }

    #[test]
    fn test_files() {
        let file_1 = File::open("C:\\Users\\Thuthuka\\Downloads\\test1.doc").unwrap();
        let file_2 = File::open("C:\\Users\\Thuthuka\\Downloads\\wordpeg.doc").unwrap();

        let test1 = WordDocument::read_file(file_1).unwrap();
        let wordpeg = WordDocument::read_file(file_2).unwrap();

        // let _ = test1.compare_to_physical(&wordpeg);
        // let _ = test1.compare_to_logical(&wordpeg);
        // let _ = wordpeg.compare_to_logical(&test1);
        let _ = wordpeg.compare_to_physical(&test1);
    }
}
