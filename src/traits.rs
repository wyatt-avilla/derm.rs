use font_kit::canvas::Canvas;

pub trait RowMajorOrder {
    fn rm_iter(&self) -> impl Iterator<Item = (u16, u16, u16)>;
}

impl RowMajorOrder for Canvas {
    fn rm_iter(&self) -> impl Iterator<Item = (u16, u16, u16)> {
        let temp: u16 = 0;
        self.pixels.iter().map(move |x| (temp, temp, u16::from(*x)))
    }
}
