type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    RowSize(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RowSize(i) => write!(f, "Invalid Row Size: {i}"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::RowSize(_) => "Invalid Row Size",
        }
    }
}

// TODO: change this
pub trait Pointify {
    fn to_points(&self, row_size: usize) -> Result<impl Iterator<Item = (u16, u16, u16)>>;
}

impl Pointify for Vec<u8> {
    fn to_points(&self, row_size: usize) -> Result<impl Iterator<Item = (u16, u16, u16)>> {
        if u16::try_from(row_size).is_err() || row_size == 0 {
            return Err(Error::RowSize(row_size));
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
