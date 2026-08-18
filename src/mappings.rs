use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "29";

pub const ROW_COUNT: usize = 3;
pub const COL_COUNT: usize = 5;
pub const KEY_COUNT: usize = ROW_COUNT * COL_COUNT;
pub const ENCODER_COUNT: usize = 0;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Kind {
    MIRABOX_293V3,
}

pub const MIRABOX_VID: u16 = 0x6603;
pub const MIRABOX_293V3_PID: u16 = 0x1005;
pub const MIRABOX_293V3_1006_PID: u16 = 0x1006;

// Map all queries to usage page 65440 and usage id 1 for now
pub const MIRABOX_293V3_QUERY: DeviceQuery =
    DeviceQuery::new(65440, 1, MIRABOX_VID, MIRABOX_293V3_PID);
pub const MIRABOX_293V3_1006_QUERY: DeviceQuery =
    DeviceQuery::new(65440, 1, MIRABOX_VID, MIRABOX_293V3_1006_PID);

pub const QUERIES: [DeviceQuery; 2] = [MIRABOX_293V3_QUERY, MIRABOX_293V3_1006_QUERY];

/// Returns the image format used by the device; all keys use the same format
pub fn get_image_format_for_key(_kind: &Kind, _key: u8) -> ImageFormat {
    ImageFormat {
        mode: ImageMode::JPEG,
        size: (112, 112),
        rotation: ImageRotation::Rot180,
        mirror: ImageMirroring::None,
    }
}

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match (vid, pid) {
            (MIRABOX_VID, MIRABOX_293V3_PID) => Some(Kind::MIRABOX_293V3),
            (MIRABOX_VID, MIRABOX_293V3_1006_PID) => Some(Kind::MIRABOX_293V3),
            _ => None,
        }
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        3
    }

    /// There is no point relying on manufacturer/device names reported by the USB stack,
    /// so we return custom names for all the kinds of devices
    pub fn human_name(&self) -> String {
        match &self {
            Self::MIRABOX_293V3 => "Mirabox 293V3",
        }
        .to_string()
    }

    /// This method would not be called for protocol 3 devices, so mark it as unreachable
    pub fn id_suffix(&self) -> String {
        match &self {
            Self::MIRABOX_293V3 => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
