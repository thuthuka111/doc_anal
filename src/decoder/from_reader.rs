use crate::subreader::SubReader;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

use super::from_c_struct::FromCStruct;
pub use super::model::*;

/// A Trait that states that 'Self' is able to be constructed from some reader
pub trait FromReader: Sized {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self>;
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

        reader.seek(SeekFrom::Start(0x02E2))?;
        let fcPlcfLst = reader.read_i32::<LittleEndian>()?;
        let lcbPlcfLst = reader.read_u32::<LittleEndian>()?;
        let fcPlfLfo = reader.read_i32::<LittleEndian>()?;
        let lcbPlfLfo = reader.read_u32::<LittleEndian>()?;

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
            fcPlcfLst,
            lcbPlcfLst,
            fcPlfLfo,
            lcbPlfLfo,
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

        // println!("Number of styles: {}", cstd);

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

            // println!("{:?}\n", stylesheet_std);
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

impl FromReader for LSTs {
    fn from_reader<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        // numbers of LST structures
        let num_ltss = reader.read_u16::<LittleEndian>()?;
        // println!("Number of LTS's: {}", num_ltss);

        // let bytes_left = reader.bytes().count() as i64;
        // println!("Bytes at start: {}", bytes_left);
        // let _ = reader.seek(SeekFrom::Current(-bytes_left))?;

        // make the LSTF
        let mut lsts = Vec::with_capacity(num_ltss.into());
        for _ in 0..num_ltss {
            #[allow(non_snake_case, unused)]
            let LSTF = {
                let lsid = reader.read_i32::<LittleEndian>().unwrap();
                let tplc = reader.read_i32::<LittleEndian>().unwrap();
                // 18 bytes (rgstid[9])
                let mut rgistd_buff = [0 as u8; 18];
                let _ = reader.read(&mut rgistd_buff).unwrap();
                let mut rgistd = [0 as u16; 9];
                for i in 0..9 {
                    rgistd[i] = u16::from_le_bytes([rgistd_buff[2 * i], rgistd_buff[2 * i + 1]]);
                }

                let mut bitfield_buff = reader.read_u8()?;

                let html_compat_flags_bitfield = reader.read_u8()?;

                LSTF {
                    lsid,
                    tplc,
                    rgistd,
                    flagfield: bitfield_buff,
                    compat_flags: html_compat_flags_bitfield,
                }
            };
            // println!("{:#X?}", LSTF);

            lsts.push(LST {
                lstf: LSTF,
                level_styles: Vec::new(),
            });
        }

        // let bytes_left = reader.bytes().count() as i64;
        // println!("Bytes after reading the LSTFs: {}", bytes_left);
        // let _ = reader.seek(SeekFrom::Current(-bytes_left))?;

        for lst in lsts.iter_mut() {
            // the number of LVLF structures to reading depending on fSimpleFlag
            let num_lvlf = if lst.lstf.fSimpleList() { 1 } else { 9 };

            let mut level_styles = Vec::with_capacity(num_ltss.into());

            for _ in 0..num_lvlf {
                #[allow(non_snake_case, unused)]
                let LVLF = {
                    let iStartAt = reader.read_i32::<LittleEndian>().unwrap(); // 4
                    let nfc = reader.read_u8().unwrap(); // 1

                    let bitfield = reader.read_u8().unwrap(); // 1

                    let jc = ((bitfield & 0xC0) >> 6) as u8;
                    let fLegal = (bitfield & 0x20) == 0x20;
                    let fNoRestart = (bitfield & 0x10) == 0x10;
                    let fPrev = (bitfield & 0x08) == 0x08;
                    let fPrevSpace = (bitfield & 0x04) == 0x04;
                    let fWord6 = (bitfield & 0x02) == 0x02;

                    let mut rgbxchNums = [0; 9];
                    reader.read_exact(&mut rgbxchNums).unwrap(); // 9

                    let ixchFollow = reader.read_u8().unwrap(); // 1
                    let dxaSpace = reader.read_i32::<LittleEndian>().unwrap(); // 4
                    let dxaIndent = reader.read_i32::<LittleEndian>().unwrap(); // 4
                    let cbGrpprlPapx = reader.read_u8().unwrap(); // 1
                    let cbGrpprlChpx = reader.read_u8().unwrap(); // 1
                    let ilvlRestartLim = reader.read_u8().unwrap(); // 1
                    let grfhic = reader.read_u8().unwrap(); // 1

                    LVLF {
                        iStartAt,
                        nfc,
                        jc,
                        fLegal,
                        fNoRestart,
                        fPrev,
                        fPrevSpace,
                        fWord6,
                        rgbxchNums,
                        ixchFollow,
                        dxaSpace,
                        dxaIndent,
                        cbGrpprlChpx,
                        cbGrpprlPapx,
                        ilvlRestartLim,
                        grfhic,
                    }
                };

                // println!("{:#X?}", LVLF);
                // println!("{:#?}", LVLF);

                // read cbGrpprlPapx bytes
                let mut grpprl_papx_buffer = vec![0; LVLF.cbGrpprlPapx as usize];
                reader.read_exact(&mut grpprl_papx_buffer).unwrap();

                // read cbGrpprlChpx bytes
                let mut grpprl_chpx_buffer = vec![0; LVLF.cbGrpprlChpx as usize];
                reader.read_exact(&mut grpprl_chpx_buffer).unwrap();


                let number_text = {
                    let length_byte = reader.read_u16::<LittleEndian>().unwrap();
                    // println!("Length of Number Text: {}", length_byte);
                    let mut name_buffer: Vec<u16> = vec![0; length_byte as usize];
                    reader.read_u16_into::<BigEndian>(&mut name_buffer)?;
                    String::from_utf16(&name_buffer).unwrap()
                };

                level_styles.push(LVL {
                    lvlf: LVLF,
                    grpprlChpx: grpprl_chpx_buffer,
                    grpprlPapx: grpprl_papx_buffer,
                    nubmer_text: number_text,
                });
            }

            lst.level_styles = level_styles;
        }

        // calculate and pring number of remaining bytes in buffer
        // let remaining_bytes = reader.bytes().count();
        // println!("Remaining bytes in buffer: {}", remaining_bytes);

        // redundant type lol
        Ok(LSTs {
            num_LSTs: num_ltss,
            LSTs: lsts,
        })
    }
}
