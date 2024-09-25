use std::num::TryFromIntError;

pub trait Points {
    fn to_points(
        &self,
        row_size: usize,
    ) -> Result<impl Iterator<Item = (u16, u16, u16)>, TryFromIntError>;
}

impl Points for Vec<u8> {
    fn to_points(
        &self,
        row_size: usize,
    ) -> Result<impl Iterator<Item = (u16, u16, u16)>, TryFromIntError> {
        u16::try_from(row_size)?;

        Ok(self.chunks(row_size).enumerate().flat_map(|(col, chunk)| {
            chunk.iter().enumerate().map(move |(row, num)| {
                (
                    u16::try_from(row).unwrap(),
                    u16::try_from(col).unwrap(),
                    u16::from(*num),
                )
            })
        }))
    }
}
