use json::{object, JsonValue};
use struct_iterable::Iterable;

use super::model::*;

/// A Trait that states that 'Self' can be made into a `Structure`
pub trait ToStructure {
    /// A function for annotation name of fields in the structure with their description
    fn descriptions() -> JsonValue;
    /// A function which processes the struct and returns a JsonValue of the name and value of the fields
    fn structure_items(&self) -> Vec<StructureItem>;
    /// A function used to make the substructures, if any, of the struct
    fn substructures(&self) -> Option<Vec<Structure>>;
}

impl ToStructure for Fib {
    fn descriptions() -> JsonValue {
        let fields = vec![
            ("wIdent", "Magic Number"),
            (
                "nFib",
                "FIB version written. Should be >= 101 for post Word 6.0 windows files",
            ),
            ("nProduct", "Product version written by"),
            ("Lid", "language Stamp"),
            ("fDot", "is document a template"),
            ("fGlsy", "is document a glossary"),
            ("fComplex", "if 1, document is complex, fast-saved format"),
            ("fHasPic", "if 1, document has more than 1 picture"),
            (
                "cQuickSaves",
                "Number of times the document has been quick-saved",
            ),
            ("fEncrypted", "if 1, document is encrypted"),
            ("fWhichTblStm", "which table stream is used"),
            (
                "fReadOnlyRecommended",
                "set if user has recommended that document be read-only",
            ),
            (
                "fWriteReservation",
                "set if file owner has made the file write reserved",
            ),
            ("fExtChar", "set when using extended character set in file"),
            (
                "nFibBack",
                "compatible with file readers that support an nFib value at least this value",
            ),
            ("lKey", "Encryption key, valid only if fEncrypted is set"),
            (
                "Envr",
                "Environment in which the document was created, 0 for Word, 1 for Macintosh",
            ),
            (
                "fMac",
                "when 1, file was last saved in Macintosh environment",
            ),
            ("fcMin", "File offset of the first character of text"),
            ("fcMac", "File offset of the last character of text = 1"),
            (
                "Csw",
                "Count of fields in the \"array of shorts\" section of the FIB",
            ),
            (
                "wMagicCreated",
                "Unique number identifying the file's creator. 0x6A62 is for Word creator",
            ),
            ("wMagicRevised", "Identifies the file's last modifier"),
            ("wMagicCreatedPrivate", "Private data"),
            ("wMagicRevisedPrivate", "Private data"),
            ("pnfbpChpFirst_W6", "Not used"),
            ("pnChpFirst_W6", "Not used"),
            ("cpnBteChp_W6", "Not used"),
            ("pnFbpPapFirst_W6", "Not used"),
            ("pnPapFirst_W6", "Not used"),
            ("cpnBtePap_W6", "Not used"),
            ("pnFbpLvcFirst_W6", "Not used"),
            ("pnLvcFirst_W6", "Not used"),
            ("cpnBteLvc_W6", "Not used"),
            ("lidFE", "Lid equivalent if FIB.fFarEast is set"),
            (
                "clw",
                "Count of longs in the \"array of longs\" section of the FIB",
            ),
            ("cbMac", "File offset of last byte written to file + 1"),
            ("lProductCreated", "Build date of creator tool"),
            ("lProductRevised", "Build date of last modifier tool"),
            ("ccpText", "Length of main document text stream"),
            ("ccpFtn", "Length of footnote subdocument text stream"),
            ("ccpHdr", "Length of header subdocument text stream"),
            (
                "ccpMcr",
                "Length of macro subdocument text stream(should be 0)",
            ),
            ("ccpAtn", "Length of annotation subdocument text stream"),
            ("ccpEdn", "Length of endnote subdocument text stream"),
            ("ccpTxbx", "Length of textbox subdocument text stream"),
            (
                "ccpHdrTxbx",
                "Length of header textbox subdocument text stream",
            ),
            ("pnFbpChpFirst", "See specification"),
            ("pnChpFirst", ""),
            ("cpnBteChp", "Count of CHPX FKPs recorded in file"),
            ("pnFbpPapFirst", ""),
            ("pnPapFirst", ""),
            ("cpnBtePap", ""),
            ("pnFbpLvcFirst", ""),
            ("pnLvcFirst", "The page number of the lowest numbered page in the document that records LVS FKP information"),
            ("cpnBteLvc", "Count of LVC FKPs recorded in file"),
            ("fcIslandFirst", ""),
            ("fcIslandLim", ""),
            ("Cfclcb", "Number of FC/LCB pairs in the \"FC/LCB pairs\" section of the FIB"),
            ("fcStshfOrig", "File offset of original STSH structure allocation in the table stream"),
            ("lcbStshfOrig", "count of bytes of original STSH allocation"),
            ("fcStshf", "Offset of the STSH in the table stream"),
            ("lcbStshf", "Count of bytes of the above variable"),
            ("fcPlcffndRef", "Offset in the table stream o fthe footnote reference PLCF of FRD structures"),
            ("lcbPlcffndRef", ""),
            ("fcPlcffndTxt", "Offset in the table stream of footnote text PLC"),
            ("lcbPlcffndTxt", ""),
            ("fcPlcfandRef", "Offset in table stream of annotation reference ATRDPre10 PLC"),
            ("lcbPlcfandRef", ""),
            ("fcPlcfandTxt", "Offset in the table stream of annotation text PLC"),
            ("lcbPlcfandTxt", ""),
            ("fcPlcfsed", "Offset in the table stream of the section descriptor SED PLC"),
            ("lcbPlcfsed", ""),
            ("fcPlcpad", "No longer used"),
            ("lcbPlcpad", ""),
            ("fcPlcfphe", "Offset in table stream of PHE PLC"),
            ("lcbPlcfphe", ""),
            ("fcSttbfglsy", "Offset in table stream of glossary string table"),
            ("lcbSttbfglsy", ""),
            ("fcPlcfglsy", "offset in table stream of glossary PLC"),
            ("lcbPlcfglsy", ""),
            ("fcPlcfhdd", "Byte offset in the table stream of header HDD PLC"),
            ("lcbPlcfhdd", ""),
            ("fcPlcfbteChpx", "Offset in table stream of character property bin table"),
            ("lcbPlcfbteChpx", ""),
            ("fcPlcfbtePapx", "Offset in table stream of paragraph property bin table"),
            ("lcbPlcfbtePapx", ""),
            ("fcPlcfsea", "Offset in table stream of PLC reserved for private use"),
            ("lcbPlcfsea", ""),
            ("fcsttbfffn", "Offset in table stream of font information STTBF"),
            ("lcbsttbfffn", ""),
            ("fcPlcffldMom", "Offset in table stream o fthe FLD PLC of field positions in the main document"),
            ("lcbPlcffldMom", ""),
            ("fcPlcffldHdr", "Offset in table stream of the FLD PLC of field positions in the header subdocument"),
            ("lcbPlcffldHdr", ""),
            ("fcPlcffldFtn", "Offset in table stream of the FLD PLC of field positions in the footnote subdocument"),
            ("lcbPlcffldFtn", ""),
            ("fcPlcffldAtn", "Offset in table stream of the FLD PLC of field positions in the annotation subdocument"),
            ("lcbPlcffldAtn", ""),
            ("fcPlcffldMcr", "No Longer used"),
            ("lcbPlcffldMcr", "No Longer used"),
            ("fcSttbfbkmk", "Offset in the table stream of STTBF that records bookmark names in the main document"),
            ("lcbSttbfbkmk", ""),
            ("fcPlcfbkf", "offset in the table stream of the PLCF that records the beginning CP offset of bookmarks in the main document."),
            ("lcbPlcfbkf", ""),
            ("fcPlcfbkl", "Offset in the table stream that records the ending CP offsets of bookmarks recorded in the main document"),
            ("lcbPlcfbkl", ""),
            ("fcCmds", "Offsets in the table stream of Macro commands (commands are private and undocumented)"),
            ("lcbCmds", ""),
            ("fcPlcmcr", "No longer used"),
            ("lcbPlcmcr", "No longer used"),
            ("fcSttbfmcr", "No longer used"),
            ("lcbSttbfmcr", "No longer used"),
            ("fcPrDrvr", "Offset in the table stream of the printer driver information"),
            ("lcbPrDrvr", ""),
            ("fcPrEnvPort", "Offset in table stream o fthe print environment in portait mode"),
            ("lcbPrEnvPort", ""),
            ("fcPrEnvLand", "Offset in table stream o fthe print environment in landscape mode"),
            ("lcbPrEnvLand", ""),
            ("fcWss", "Offset in table stream of 'Window Save State'(WSS) data structure"),
            ("lcbWss", ""),
            ("fcDop", "Offset in table stream of document property data structure"),
            ("lcbDop", ""),
            ("fcSttbfAssoc", "Offset in table stream of STTBF of associated strings"),
            ("lcbSttbfAssoc", ""),
            ("fcClx", "Offset in table stream of beginning of information for Complex Files"),
            ("lcbClx", ""),
            ("fcPlcfpgdFtn", "Not used"),
            ("lcbPlcfpgdFtn", "Not used"),
            ("fcAutosaveSource", "Offset in the table stream of the name of the original file"),
            ("lcbAutosaveSource", ""),
            ("fcGrpXstAtnOwners", "Offset in table stream of group of strings recording the names of owners of annotations stored in document"),
            ("lcbGrpXstAtnOwners", ""),
            ("fcSttbfAtnBkmk", "Offset in table stream of the sttbf that records names of bookmarks for the annotation subdocument"),
            ("lcbSttbfAtnBkmk", ""),
            ("fcPlcdoaMom", "No Longer used"),
            ("lcbPlcdoaMom", "No Longer used"),
            ("fcPlcdoaHdr", "No Longer used"),
            ("lcbPlcdoaHdr", "No Longer used"),
            ("fcPlcspaMom", "Offset in table stream of the FSPA PLC"),
            ("lcbPlcspaMom", ""),
            ("fcPlcspaHdr", "offset in table stream of FSPA PLC for header document"),
            ("lcbPlcspaHdr", ""),
            ("fcPlcfAtnbkf", "Offset in table stream of BFK(bookmark first) PLC of the annotation subdocument"),
            ("lcbPlcfAtnbkf", ""),
            ("fcPlcfAtnbkl", "Offset in table stream of BKL(bookmark last) PLC of the annotation subdocument"),
            ("lcbPlcfAtnbkl", ""),
            ("fcPms", "Offset in table stream of PMS(Print Merge State) information block"),
            ("lcbPms", ""),
            ("fcFormFldSttbs", "Offset in table stream of form firld sttbf which contains strings used in form field dropdown constols"),
            ("lcbFormFldSttbs", ""),
            ("fcPlcfendRef", "offset in table stream of endnote reference PLCF of FRD structures"),
            ("lcbPlcfendRef", ""),
            ("fcPlcfendTxt", "Offset in table stream of plcfendRef which points to endnote text in the document stream"),
            ("lcbPlcfendTxt", ""),
            ("fcPlcffldEdn", "Offset in table stream to FLD PLCF of field positions in the endnote subdocument"),
            ("lcbPlcffldEdn", ""),
            ("fcPlcfpgdEdn", "Not used"),
            ("lcbPlcfpgdEdn", "Not used"),
            ("fcDggInfo", "Offset in stream of the Offcie Drawing object table data"),
            ("lcbDggInfo", ""),
            ("fcSttbfRMark", "Offset in table stream to STTBF that records the author abreviiations for authors who have made revisions in document"),
            ("lcbSttbfRMark", ""),
            ("fcSttbfCaption", "Offset in table stream to STTBF that records caption titles"),
            ("lcbSttbfCaption", ""),
            ("fcSttbfAutoCaption", "Offset in table stream to STTBF that records object names and indices into the caption STTBF for object which get auto captions"),
            ("lcbSttbfAutoCaption", ""),
            ("fcPlcfWkb", "Offset in the table stream of the WKB PLCF that describes the boundaries of contributind document in a master document"),
            ("lcbPlcfWkb", ""),
            ("fcPlcfSpl", "Offset in table stream of PLCF(of SPLS structures) that records spell check state"),
            ("lcbPlcfSpl", ""),
            ("fcPlcftxbxTxt", "Table stream offset of PLCF that records the beginning CP in the text box subdoc of the text of individual text box entries"),
            ("lcbPlcftxbxTxt", ""),
            ("fcPlcffldTxbx", "Table stream offset of FLD PLCF that records field boundaries recorded in the text box subdoc"),
            ("lcbPlcffldTxbx", ""),
            ("fcPlcfhdrtxbxTxt", ""),
            ("lcbPlcfhdrtxbxTxt", ""),
            ("fcPlcffldHdrTxbx", ""),
            ("lcbPlcffldHdrTxbx", ""),
            ("fcStwUser", "Macro user Storage"),
            ("lcbStwUser", ""),
            ("fcSttbTtmbd", "Table stream offset of embeded true type font data"),
            ("lcbSttbTtmbd", ""),
            ("fcCookieData", "NLCheck error hande will persist in file"),
            ("lcbCookieData", ""),
            ("fcPgdMotherOldOld", "Offsets in table stream of the PLF that records the page and break descriptors for the main text of the document"),
            ("fcpgdFtnOldOld", "Offsets in table stream of the PLF that records the page and break descriptors for the footnote text of the document"),
            ("fcpgdEdnOldOld", "Offsets in table stream of the PLF that records the page and break descriptors for the endnote text of the document"),
            ("fcSttbfIntlFld", "Table stream offset of STTBF containing field keywords, no longer written to for nfib >= 167"),
            ("lcbSttbfIntlFld", ""),
            ("fcRouteSlip", "Table stream offset of mailer routing slip"),
            ("lcbRouteSlip", ""),
            ("fcSttbSavedBy", "Table stream offset of STTBF recording the names of users who have saved this document alernating the save locations"),
            ("lcbSttbSavedBy", ""),
            ("fcSttbFnm", "Offset in table stream of STTBF recording filenames of documents which are referenced by this document"),
            ("lcbSttbFnm", ""),
            ("fcPlcfLst", "Table stream offset of list format information structure"),
            ("lcbPlcfLst", ""),
            ("fcPlfLfo", "Offset in table stream of list format override information"),
            ("lcbPlfLfo", ""),
            ("fcPlcftxbxBkd", "Table stream offset of the textbox break table for main document"),
            ("lcbPlcftxbxBkd", ""),
            ("fcPlcftxbxHdrBkd", ""),
            ("lcbPlcftxbxHdrBkd", ""),
            ("fcDocUndoWord9", ""),
            ("lcbDocUndoWord9", ""),
            ("fcRgbUse", ""),
            ("lcbRgbUse", ""),
            ("fcUsp", ""),
            ("lcbUsp", ""),
            ("fcUskf", ""),
            ("lcbUskf", ""),
            ("fcPlcupcRgbUse", ""),
            ("lcbPlcupcRgbUse", ""),
            ("fcPlcupcUsp", ""),
            ("lcbPlcupcUsp", ""),
            ("fcSttbGlsyStyle", ""),
            ("lcbSttbGlsyStyle", ""),
            ("fcPlgosl", ""),
            ("lcbPlgosl", ""),
            ("fcPlcocx", ""),
            ("lcbPlcocx", ""),
            ("fcPlcfBteLvc", "Offset in table stream of character property bin"),
            ("lcbPlcfBteLvc", ""),
            ("dwLowDateTime", ""),
            ("dwHighDateTime", ""),
            ("fcPlcfLvcPre10", "Table Stream offset of LVC PLCF used preWord10"),
            ("lcbPlcfLvcPre10", ""),
            ("fcPlcfAsumy", "Table stream offset of autosummary ASUMY PLCF"),
            ("lcbPlcfAsumy", ""),
            ("fcPlcfGram", "Offset in table stream of PLCF which records grammer check state"),
            ("lcbPlcfGram", ""),
            ("fcSttbListNames", "Table stream Offset of list naes string table"),
            ("lcbSttbListNames", ""),
            ("fcSttbfUssr", ""),
            ("lcbSttbfUssr", ""),
            ("fcPlcfTch", ""),
            ("lcbPlcfTch", ""),
            ("fcRmdfThreading", "Table stream offset of revision mark data(unused)"),
            ("lcbRmdfThreading", ""),
            ("fcMid", "Table stream offset of Message ID (unused)"),
            ("lcbMid", ""),
            ("fcSttbRgtplc", "Table stream offset of list gallery data"),
            ("lcbSttbRgtplc", ""),
            ("fcMsoEnvelope", "Table stream offset of persist the mail envelope"),
            ("lcbMsoEnvelope", ""),
            ("fcPlcfLad", ""),
            ("lcbPlcfLad", ""),
            ("fcRgDofr", ""),
            ("lcbRgDofr", ""),
            ("fcPlcosl", ""),
            ("lcbPlcosl", ""),
            ("fcPlcfCookieOld", ""),
            ("lcbPlcfCookieOld", ""),
            ("fcPgdMotherOld", ""),
            ("fcpgdFtnOld", ""),
            ("fcpgdEdnOld", ""),
            ("fcUnused1", ""),
            ("lcbUnused1", ""),
            ("fcPlcfPgp", ""),
            ("lcbPlcfPgp", ""),
            ("fcPlcfuim", ""),
            ("lcbPlcfuim", ""),
            ("fcPlfguidUim", ""),
            ("lcbPlfguidUim", ""),
            ("fcAtrdExtra", ""),
            ("lcbAtrdExtra", ""),
            ("fcPlrsid", ""),
            ("lcbPlrsid", ""),
            ("fcSttbfBkmkFactoid", ""),
            ("lcbSttbfBkmkFactoid", ""),
            ("fcPlcfBkfFactoid", ""),
            ("lcbPlcfBkfFactoid", ""),
            ("fcPlcfcookie", ""),
            ("lcbPlcfcookie", ""),
            ("fcPlcfBklFactoid", ""),
            ("lcbPlcfBklFactoid", ""),
            ("fcFactoidData", ""),
            ("lcbFactoidData", ""),
            ("fcDocUndo", ""),
            ("lcbDocUndo", ""),
            ("fcSttbfBkmkFcc", ""),
            ("lcbSttbfBkmkFcc", ""),
            ("fcPlcfBkfFcc", ""),
            ("lcbPlcfBkfFcc", ""),
            ("fcPlcfBklFcc", ""),
            ("lcbPlcfBklFcc", ""),
            ("fcSttbfbkmkBPRepairs", ""),
            ("lcbSttbfbkmkBPRepairs", ""),
            ("fcPlcfbkfBPRepairs", ""),
            ("lcbPlcfbkfBPRepairs", ""),
            ("fcPmsNew", ""),
            ("lcbPmsNew", ""),
            ("fcODSO", ""),
            ("lcbODSO", ""),
            ("fcPlcfpmiOldXP", ""),
            ("lcbPlcfpmiOldXP", ""),
            ("fcPlcfpmiNewXP", ""),
            ("lcbPlcfpmiNewXP", ""),
            ("fcPlcfpmiMixedXP", ""),
            ("lcbPlcfpmiMixedXP", ""),
            ("fcEncryptedProps", ""),
            ("lcbEncryptedProps", ""),
            ("fcPlcffactoid", ""),
            ("lcbPlcffactoid", ""),
            ("fcPlcflvcOldXP", ""),
            ("lcbPlcflvcOldXP", ""),
            ("fcPlcflvcNewXP", ""),
            ("lcbPlcflvcNewXP", ""),
            ("fcPlcflvcMixedXP", ""),
            ("lcbPlcflvcMixedXP", ""),
            ("fcHplxsdr", "XML Shhema Definition references"),
            ("lcbHplxsdr", ""),
            ("fcSttbfBkmkSdt", ""),
            ("lcbSttbfBkmkSdt", ""),
            ("fcPlcfBkfSdt", ""),
            ("lcbPlcfBkfSdt", ""),
            ("fcPlcBlkSdt", ""),
            ("lcbPlcBlkSdt", ""),
            ("fcCustomXForm", ""),
            ("lcbCustomXForm", ""),
            ("fcSttbfBkmkProt", ""),
            ("lcbSttbfBkmkProt", ""),
            ("fcPlcfBkfProt", ""),
            ("lcbPlcfBkfProt", ""),
            ("fcPlcfBklProt", ""),
            ("lcbPlcfBklProt", ""),
            ("fcSttbProtUser", ""),
            ("lcbSttbProtUser", ""),
            ("fcPlcftpc", ""),
            ("lcbPlcftpc", ""),
            ("fcPlcfpmiOld", ""),
            ("lcbPlcfpmiOld", ""),
            ("fcPlcfpmiOldInline", ""),
            ("lcbPlcfpmiOldInline", ""),
            ("fcPlcfpmiNew", ""),
            ("lcbPlcfpmiNew", ""),
            ("fcPlcfpmiNewInline", ""),
            ("lcbPlcfpmiNewInline", ""),
            ("fcPlcfvcOld", ""),
            ("lcbPlcfvcOld", ""),
            ("fcPlcfvcOldInline", ""),
            ("lcbPlcfvcOldInline", ""),
            ("fcPlcflvcNew", ""),
            ("lcbPlcflvcNew", ""),
            ("fcPlcflvcNewInline", ""),
            ("lcbPlcflvcNewInline", ""),
            ("fcpgdMother", ""),
            ("fcpgdFtn", ""),
            ("fcpgdEdn", ""),
            ("fcAfd", ""),
            ("lcbAfd", ""),
            ("cswNew", "The number of entries in rgswNew[]"),
            ("actualNFib", "The actual nFib"),
            ("cQuickSavesNew", ""),
        ];
        let mut return_value = object::Object::with_capacity(fields.len());
        for (name, description) in fields.into_iter().filter(|(_, descr)| *descr != "") {
            return_value.insert(name, description.into());
        }
        return_value.into()
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for SHSHI {
    fn descriptions() -> JsonValue {
        object! {
            cbStshi: "Count of bytes of this SHSHI allocation",
            cstd: "The Number of styles in the stylesheet",
            cbSTDBaseInFile: "The length of STD Base as stored in file",
            fStdStylenamesWritten: "Flag indicating whether built-in styles are stored",
            stiMaxWhenSaved: "Max sti known when this file was written",
            istdMaxFixedWhenSaved: "The number of fixed-index issd's there are",
            nVerBuiltInNamesWhenSaved: "Current version of build-in stylenames",
            rgftcStandardChpStsh: "rgftc used by StandardChpStsh for this document",
            cbLSD: "count of bytes of each LSD in mpstilsd",
            mpstilsd: "latent style data(array of LSDs)",
        }
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();

        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            // Ignoreing Styles as it is a substructure
            if field_name == "styles" {
                continue;
            }

            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        let mut substructures = vec![];

        for style in &self.styles {
            let style_structure = Structure::from(&style.xstzName, style);
            substructures.push(style_structure);
        }

        Some(substructures)
    }
}

impl ToStructure for STD {
    fn descriptions() -> JsonValue {
        object! {
            sti: "The style identifier",
            fScratch: "Spare fields for temporary use, should alwase be zero",
            fInvalHeight: "Flag indicating PHEs of all text with this styel are wrong",
            fHasUpe: "Flag indicating that UPEs have been generated",
            fMassCopy: "Flag indicating that std has been mas-copied",
            stk: "Style Kind",
            istdBase: "Base Style identifier",
            cupx: "Number of UPSx (and UPEs)",
            istdNext: "Next Style identifier",
            bchUpe: "Offset ot end of upx's, start of upe's"
        }
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for DocumentSummaryInfoStream {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];
        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for SummaryInformation {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];

        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}

impl ToStructure for PLCF<PCD> {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        vec![]
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        let mut substructures = vec![];

        for (i, pcd) in self.rgstruct.iter().enumerate() {
            let pcd_structure = Structure::from(&format!("PCD {}", i), pcd);
            substructures.push(pcd_structure);
        }

        Some(substructures)
    }
}

impl ToStructure for PCD {
    fn descriptions() -> JsonValue {
        object! {}
    }

    fn structure_items(&self) -> Vec<StructureItem> {
        let descriptions = Self::descriptions();
        let self_json = json::parse(&serde_json::to_string(&self).unwrap()).unwrap();
        let mut structure_items = vec![];

        for (field_name, _) in self.iter() {
            let field_val = self_json[field_name].to_string();
            let description = if descriptions.has_key(&field_name) {
                Some(descriptions[field_name].clone().to_string())
            } else {
                None
            };

            structure_items.push(StructureItem {
                name: field_name.to_string(),
                value: field_val,
                description,
            });
        }

        structure_items
    }

    fn substructures(&self) -> Option<Vec<Structure>> {
        None
    }
}
