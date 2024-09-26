use std::error::Error;

// TODO: change this
pub trait Pointify {
    fn to_points(
        &self,
        row_size: usize,
    ) -> Result<impl Iterator<Item = (u16, u16, u16)>, Box<dyn Error>>;
}

impl Pointify for Vec<u8> {
    fn to_points(
        &self,
        row_size: usize,
    ) -> Result<impl Iterator<Item = (u16, u16, u16)>, Box<dyn Error>> {
        if u16::try_from(row_size).is_err() || row_size == 0 {
            return Err(String::from("invalid row size").into());
        }

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
