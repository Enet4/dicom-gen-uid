//! # DICOM UID generator
//! 
//! Designed for the simple need of wanting a DICOM unique identifier.
//! 
//! All UIDs are produced according to [DICOM PS3.5 2023d Annex B.2][1],
//! creating UUID derived UIDs.
//! 
//! [1]: https://dicom.nema.org/medical/dicom/2023c/output/chtml/part05/sect_B.2.html
use std::io::Write;

pub use uuid;

use uuid::Uuid;

/// Generate a UUID (v4) and derive a DICOM UID from it.
#[inline]
pub fn gen_uid() -> String {
    new_uid(Uuid::new_v4())
}

/// Generate a UUID (v4),
/// derive a DICOM UID from it,
/// and print it to the given writer.
#[inline]
pub fn gen_uid_to(to: impl Write) -> Result<(), std::io::Error> {
    new_uid_to(Uuid::new_v4(), to)
}

/// Create a DICOM UID derived from the given UUID
#[inline]
pub fn new_uid(uuid: Uuid) -> String {
    format!("2.25.{}", uuid.to_u128_le())
}

/// Generate a UUID derived DICOM
/// and print it to the given writer.
#[inline]
pub fn new_uid_to(uuid: Uuid, mut to: impl Write) -> Result<(), std::io::Error> {
    write!(to, "2.25.{}", uuid.to_u128_le())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use uuid::Uuid;

    #[test]
    fn base_test() {
        for _ in 0..16 {
            let uid = gen_uid();
            assert!(uid.starts_with("2.25."));
            assert!(
                uid.len() > 8,
                "expected more than 8 characters, but UID {} is {} characters long",
                uid,
                uid.len()
            );
            assert!(
                uid.len() <= 44,
                "expected less than 40 characters, but UID {} is {} characters long",
                uid,
                uid.len()
            );

            for c in uid.chars() {
                assert!(c == '.' || ('0'..='9').contains(&c));
            }
        }
    }

    #[test]
    fn new_uid_consistency() {
        let uuid = Uuid::new_v4();

        let uid = new_uid(uuid);
        assert!(uid.starts_with("2.25."));
        assert!(
            uid.len() > 8,
            "expected more than 8 characters, but UID {} is {} characters long",
            uid,
            uid.len()
        );
        assert!(
            uid.len() <= 44,
            "expected less than 44 characters, but UID {} is {} characters long",
            uid,
            uid.len()
        );

        for c in uid.chars() {
            assert!(c == '.' || ('0'..='9').contains(&c));
        }

        let uid2 = new_uid(uuid);

        assert_eq!(
            uid, uid2,
            "UIDs obtained from the same UUID should be equal"
        );
    }

    #[test]
    fn test_write_to() {
        let mut out = vec![];

        gen_uid_to(&mut out).unwrap();

        let uid = std::str::from_utf8(&out).expect("output should be valid UTF-8");

        let len = out.len();
        assert_eq!(
            len,
            uid.len(),
            "byte length should be equivalent to string length"
        );

        assert!(
            len > 8,
            "expected more than 8 characters, but UID {} is {} characters long",
            uid,
            len
        );
        assert!(
            uid.len() <= 44,
            "expected less than 44 characters, but UID {} is {} characters long",
            uid,
            uid.len()
        );

        for c in &out {
            assert!(*c == b'.' || (b'0'..=b'9').contains(c));
        }

        gen_uid_to(&mut out).unwrap();
        assert!(out.len() > len);
    }
}
