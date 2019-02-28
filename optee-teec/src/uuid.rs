use core::fmt;
use hex;
use optee_teec_sys as raw;
use uuid as uuid_crate;
use uuid_crate::parser::ParseError;

pub struct Uuid {
    raw: raw::TEEC_UUID,
}

impl Uuid {
    pub fn parse_str(input: &str) -> Result<Uuid, ParseError> {
        let uuid = uuid_crate::Uuid::parse_str(input)?;
        let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = uuid.as_fields();
        Ok(Self::new_raw(
            time_low,
            time_mid,
            time_hi_and_version,
            *clock_seq_and_node,
        ))
    }

    /// Creates a `Uuid` using the supplied big-endian bytes.
    pub fn from_bytes(bytes: [u8; 16]) -> Uuid {
        let uuid = uuid_crate::Uuid::from_bytes(bytes);
        let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = uuid.as_fields();
        Self::new_raw(
            time_low,
            time_mid,
            time_hi_and_version,
            *clock_seq_and_node,
        )
    }

    pub fn new_raw(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clock_seq_and_nod: [u8; 8],
    ) -> Uuid {
        let raw_uuid = raw::TEEC_UUID {
            timeLow: time_low,
            timeMid: time_mid,
            timeHiAndVersion: time_hi_and_version,
            clockSeqAndNode: clock_seq_and_nod,
        };
        Self { raw: raw_uuid }
    }

    pub fn as_ptr(&self) -> *const raw::TEEC_UUID {
        &self.raw
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:x}-{:x}-{:x}-{}",
            self.raw.timeLow,
            self.raw.timeMid,
            self.raw.timeHiAndVersion,
            hex::encode(self.raw.clockSeqAndNode)
        )
    }
}
