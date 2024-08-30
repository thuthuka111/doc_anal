use super::model::*;

/// Trait to indicate that a struct can be built from its C representation in bytes
pub trait FromCStruct {
    /// The size of the C struct in bytes
    fn c_size() -> usize;
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl<T: FromCStruct> PLCF<T> {
    pub fn from_bytes(bytes: &[u8]) -> Self {
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
