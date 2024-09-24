use itertools::Itertools;

pub fn print_to_console<T, I>(iterator: &I, row_size: usize, include: fn(T) -> bool)
where
    I: Iterator<Item = T> + Clone,
{
    let h_per_v = 2;

    let formatted_rows = iterator
        .clone()
        .chunks(row_size)
        .into_iter()
        .map(|row| {
            row.map(|x| {
                if include(x) {
                    String::from("█").repeat(h_per_v)
                } else {
                    String::from(" ").repeat(h_per_v)
                }
            })
            .join("")
        })
        .map(|row| "┃".to_owned() + &row.clone() + "┃")
        .collect_vec();

    let top_border = vec!["┏".to_string() + &"━".repeat(row_size * h_per_v) + "┓"];
    let bot_border = vec!["┗".to_string() + &"━".repeat(row_size * h_per_v) + "┛"];
    let full_image = [top_border, formatted_rows, bot_border].concat();

    print!(
        "{}",
        full_image
            .iter()
            .fold(String::new(), |s, row| s + row + "\n")
    );
}
