//! ICC Profile header parsing.

use crate::Result;
use exiftool_attrs::{AttrValue, Attrs};

/// ICC Profile class types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileClass {
    Input,
    Display,
    Output,
    DeviceLink,
    ColorSpace,
    Abstract,
    NamedColor,
    Unknown,
}

impl ProfileClass {
    fn from_sig(sig: &[u8; 4]) -> Self {
        match sig {
            b"scnr" => Self::Input,
            b"mntr" => Self::Display,
            b"prtr" => Self::Output,
            b"link" => Self::DeviceLink,
            b"spac" => Self::ColorSpace,
            b"abst" => Self::Abstract,
            b"nmcl" => Self::NamedColor,
            _ => Self::Unknown,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Input => "Input Device Profile",
            Self::Display => "Display Device Profile",
            Self::Output => "Output Device Profile",
            Self::DeviceLink => "DeviceLink Profile",
            Self::ColorSpace => "ColorSpace Conversion Profile",
            Self::Abstract => "Abstract Profile",
            Self::NamedColor => "NamedColor Profile",
            Self::Unknown => "Unknown",
        }
    }
}

/// Rendering intent types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingIntent {
    Perceptual,
    RelativeColorimetric,
    Saturation,
    AbsoluteColorimetric,
    Unknown(u32),
}

impl RenderingIntent {
    fn from_u32(val: u32) -> Self {
        match val {
            0 => Self::Perceptual,
            1 => Self::RelativeColorimetric,
            2 => Self::Saturation,
            3 => Self::AbsoluteColorimetric,
            n => Self::Unknown(n),
        }
    }

    fn as_str(&self) -> String {
        match self {
            Self::Perceptual => "Perceptual".to_string(),
            Self::RelativeColorimetric => "Media-Relative Colorimetric".to_string(),
            Self::Saturation => "Saturation".to_string(),
            Self::AbsoluteColorimetric => "ICC-Absolute Colorimetric".to_string(),
            Self::Unknown(n) => format!("Unknown ({})", n),
        }
    }
}

/// ICC Profile header (128 bytes).
#[derive(Debug)]
pub struct IccHeader {
    pub size: u32,
    pub cmm_type: [u8; 4],
    pub version_major: u8,
    pub version_minor: u8,
    pub version_bugfix: u8,
    pub profile_class: ProfileClass,
    pub color_space: [u8; 4],
    pub pcs: [u8; 4],
    pub date_time: Option<String>,
    pub signature: [u8; 4],
    pub platform: [u8; 4],
    pub flags: u32,
    pub device_manufacturer: [u8; 4],
    pub device_model: [u8; 4],
    pub device_attributes: u64,
    pub rendering_intent: RenderingIntent,
    pub illuminant_x: f64,
    pub illuminant_y: f64,
    pub illuminant_z: f64,
    pub creator: [u8; 4],
    pub profile_id: [u8; 16],
}

impl IccHeader {
    /// Parse header from 128 bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

        let mut cmm_type = [0u8; 4];
        cmm_type.copy_from_slice(&data[4..8]);

        // ICC version: byte 8 = major, byte 9 high nibble = minor, low nibble = bugfix
        let version_major = data[8];
        let version_minor = data[9] >> 4;
        let version_bugfix = data[9] & 0x0F;

        let mut class_sig = [0u8; 4];
        class_sig.copy_from_slice(&data[12..16]);
        let profile_class = ProfileClass::from_sig(&class_sig);

        let mut color_space = [0u8; 4];
        color_space.copy_from_slice(&data[16..20]);

        let mut pcs = [0u8; 4];
        pcs.copy_from_slice(&data[20..24]);

        // Date/time
        let year = u16::from_be_bytes([data[24], data[25]]);
        let month = u16::from_be_bytes([data[26], data[27]]);
        let day = u16::from_be_bytes([data[28], data[29]]);
        let hour = u16::from_be_bytes([data[30], data[31]]);
        let min = u16::from_be_bytes([data[32], data[33]]);
        let sec = u16::from_be_bytes([data[34], data[35]]);
        let date_time = if year > 0 {
            Some(format!(
                "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, min, sec
            ))
        } else {
            None
        };

        let mut signature = [0u8; 4];
        signature.copy_from_slice(&data[36..40]);

        let mut platform = [0u8; 4];
        platform.copy_from_slice(&data[40..44]);

        let flags = u32::from_be_bytes([data[44], data[45], data[46], data[47]]);

        let mut device_manufacturer = [0u8; 4];
        device_manufacturer.copy_from_slice(&data[48..52]);

        let mut device_model = [0u8; 4];
        device_model.copy_from_slice(&data[52..56]);

        let device_attributes = u64::from_be_bytes([
            data[56], data[57], data[58], data[59], data[60], data[61], data[62], data[63],
        ]);

        let intent_val = u32::from_be_bytes([data[64], data[65], data[66], data[67]]);
        let rendering_intent = RenderingIntent::from_u32(intent_val);

        // Illuminant XYZ (s15Fixed16)
        let illuminant_x = Self::read_s15fixed16(&data[68..72]);
        let illuminant_y = Self::read_s15fixed16(&data[72..76]);
        let illuminant_z = Self::read_s15fixed16(&data[76..80]);

        let mut creator = [0u8; 4];
        creator.copy_from_slice(&data[80..84]);

        let mut profile_id = [0u8; 16];
        profile_id.copy_from_slice(&data[84..100]);

        Ok(Self {
            size,
            cmm_type,
            version_major,
            version_minor,
            version_bugfix,
            profile_class,
            color_space,
            pcs,
            date_time,
            signature,
            platform,
            flags,
            device_manufacturer,
            device_model,
            device_attributes,
            rendering_intent,
            illuminant_x,
            illuminant_y,
            illuminant_z,
            creator,
            profile_id,
        })
    }

    fn read_s15fixed16(data: &[u8]) -> f64 {
        let raw = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        raw as f64 / 65536.0
    }

    fn sig_to_str(sig: &[u8; 4]) -> String {
        String::from_utf8_lossy(sig).trim().to_string()
    }

    /// Convert header to attributes.
    pub fn to_attrs(&self, attrs: &mut Attrs) {
        attrs.set("ICC:ProfileSize", AttrValue::UInt(self.size));

        let cmm = Self::sig_to_str(&self.cmm_type);
        if !cmm.is_empty() {
            attrs.set("ICC:ProfileCMMType", AttrValue::Str(cmm));
        }

        attrs.set(
            "ICC:ProfileVersion",
            AttrValue::Str(format!(
                "{}.{}.{}",
                self.version_major, self.version_minor, self.version_bugfix
            )),
        );

        attrs.set(
            "ICC:ProfileClass",
            AttrValue::Str(self.profile_class.as_str().to_string()),
        );

        let cs = Self::sig_to_str(&self.color_space);
        attrs.set("ICC:ColorSpaceData", AttrValue::Str(cs));

        let pcs = Self::sig_to_str(&self.pcs);
        attrs.set("ICC:ProfileConnectionSpace", AttrValue::Str(pcs));

        if let Some(ref dt) = self.date_time {
            attrs.set("ICC:ProfileDateTime", AttrValue::Str(dt.clone()));
        }

        let platform = Self::sig_to_str(&self.platform);
        if !platform.is_empty() {
            let platform_name = match platform.as_str() {
                "APPL" => "Apple Computer Inc.",
                "MSFT" => "Microsoft Corporation",
                "SGI " | "SGI" => "Silicon Graphics Inc.",
                "SUNW" => "Sun Microsystems Inc.",
                _ => &platform,
            };
            attrs.set("ICC:PrimaryPlatform", AttrValue::Str(platform_name.to_string()));
        }

        // Flags
        let embedded = (self.flags & 0x01) != 0;
        let independent = (self.flags & 0x02) == 0;
        attrs.set(
            "ICC:CMMFlags",
            AttrValue::Str(format!(
                "{}, {}",
                if embedded { "Embedded" } else { "Not Embedded" },
                if independent { "Independent" } else { "Not Independent" }
            )),
        );

        let mfr = Self::sig_to_str(&self.device_manufacturer);
        if !mfr.is_empty() {
            attrs.set("ICC:DeviceManufacturer", AttrValue::Str(mfr));
        }

        let model = Self::sig_to_str(&self.device_model);
        if !model.is_empty() {
            attrs.set("ICC:DeviceModel", AttrValue::Str(model));
        }

        // Device attributes
        let attr_low = (self.device_attributes & 0xFFFFFFFF) as u32;
        let reflective = (attr_low & 0x01) == 0;
        let glossy = (attr_low & 0x02) == 0;
        let positive = (attr_low & 0x04) == 0;
        let color = (attr_low & 0x08) == 0;
        attrs.set(
            "ICC:DeviceAttributes",
            AttrValue::Str(format!(
                "{}, {}, {}, {}",
                if reflective { "Reflective" } else { "Transparency" },
                if glossy { "Glossy" } else { "Matte" },
                if positive { "Positive" } else { "Negative" },
                if color { "Color" } else { "B&W" }
            )),
        );

        attrs.set(
            "ICC:RenderingIntent",
            AttrValue::Str(self.rendering_intent.as_str()),
        );

        attrs.set(
            "ICC:ConnectionSpaceIlluminant",
            AttrValue::Str(format!(
                "{:.6} {:.6} {:.6}",
                self.illuminant_x, self.illuminant_y, self.illuminant_z
            )),
        );

        let creator = Self::sig_to_str(&self.creator);
        if !creator.is_empty() {
            attrs.set("ICC:ProfileCreator", AttrValue::Str(creator));
        }

        // Profile ID (MD5)
        if self.profile_id.iter().any(|&b| b != 0) {
            let id_hex: String = self.profile_id.iter().map(|b| format!("{:02x}", b)).collect();
            attrs.set("ICC:ProfileID", AttrValue::Str(id_hex));
        }
    }
}
