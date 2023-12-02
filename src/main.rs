use std::io;

struct DigitParser {
    character: u8,
    string: &'static [u8],
    depth: usize,
}

impl DigitParser {
    fn new(character: u8, string: &'static str) -> Self {
        Self {
            character,
            string: string.as_bytes(),
            depth: 0,
        }
    }

    fn consume(&mut self, ch: u8) -> Option<u32> {
        if ch == self.expected() {
            self.depth += 1;
        } else if ch == self.first_expected() {
            self.depth = 1;
        } else {
            self.depth = 0;
        }

        (ch == self.character || self.depth == self.string.len()).then(|| {
            self.reset();
            self.digit()
        })
    }

    fn expected(&self) -> u8 {
        self.string[self.depth]
    }

    fn first_expected(&self) -> u8 {
        self.string[0]
    }

    fn reset(&mut self) {
        self.depth = 0;
    }

    fn digit(&self) -> u32 {
        u32::from(self.character) - 48
    }
}

struct Parser {
    parsers: [DigitParser; 9],
}

impl Parser {
    fn new() -> Self {
        Self {
            parsers: [
                DigitParser::new(b'1', "one"),
                DigitParser::new(b'2', "two"),
                DigitParser::new(b'3', "three"),
                DigitParser::new(b'4', "four"),
                DigitParser::new(b'5', "five"),
                DigitParser::new(b'6', "six"),
                DigitParser::new(b'7', "seven"),
                DigitParser::new(b'8', "eight"),
                DigitParser::new(b'9', "nine"),
            ],
        }
    }

    fn consume(&mut self, ch: u8) -> Option<u32> {
        let results: [_; 9] = std::array::from_fn(|index| self.parsers[index].consume(ch));
        results.into_iter().find_map(std::convert::identity)
    }
}

fn parse(line: &str) -> Option<u32> {
    let mut parser = Parser::new();
    let (first, last) = line.bytes().fold((None, None), |(first, last), current| {
        match (first, last, parser.consume(current)) {
            (_, _, None) => (first, last),
            (None, _, new) => (new, new),
            (_, _, new) => (first, new),
        }
    });

    let (first, last) = (first?, last?);
    let number = first * 10 + last;

    Some(number)
}

fn main() -> io::Result<()> {
    let file = std::env::args_os()
        .nth(1)
        .ok_or(io::ErrorKind::InvalidInput)?;

    let result = std::fs::read_to_string(file)?
        .lines()
        .filter_map(parse)
        .sum::<u32>();

    println!("{result}");

    Ok(())
}
