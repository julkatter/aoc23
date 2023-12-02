use std::io;

fn main() -> io::Result<()> {
    let file = std::env::args_os()
        .nth(1)
        .ok_or(io::ErrorKind::InvalidInput)?;

    let result = std::fs::read_to_string(file)?
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter(char::is_ascii_digit);
            let first = digits.next().map(u32::from)?;
            let last = digits.last().map(u32::from).unwrap_or(first);

            let number = (first - 48) * 10 + (last - 48);

            Some(number)
        })
        .sum::<u32>();

    println!("{result}");

    Ok(())
}
