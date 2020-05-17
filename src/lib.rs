mod error;
mod format;
mod pacfile;
mod pacfile_builder;
mod proto;

pub use error::{Error, Result};
pub use pacfile::Pacfile;
pub use pacfile_builder::PacfileBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let n: u32 = 0x32636170;
        let buf: [u8; 4] = [
            (n & 0xFF) as u8,
            ((n & 0xFF00) >> 8) as u8,
            ((n & 0xFF0000) >> 16) as u8,
            ((n & 0xFF000000) >> 24) as u8,
        ];
        assert_eq!(buf, [0x70, 0x61, 0x63, 0x32]);
    }
}
