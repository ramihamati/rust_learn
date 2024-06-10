
pub struct InputReader<'a> {
    input: &'a str,
    input_as_bytes: &'a [u8],
    pub line_current: usize, // line position of current cursor of token scan
    pub line_start: usize, // line position of start of token scan
    pub scanner_current: usize, // position in line
    pub line: usize, // line number
    pub scanner_start: usize, // scanner start
}

impl<'a> InputReader<'a> {
    pub fn new(input: &'a str) -> InputReader {
        InputReader {
            input,
            input_as_bytes: input.as_bytes(),
            line_current: 0, // position in current line
            line_start: 0, // position in current line
            scanner_current: 0, // current position per document
            scanner_start: 0, // where has the token started
            line: 0, // current line
        }
    }

    pub(crate) fn scanner_advance(self: &mut Self) -> bool {
        self.scanner_current += 1;
        self.line_current += 1;
        !self.is_at_end()
    }
    pub(crate) fn scanner_forward(self: &mut Self) -> bool {
        self.scanner_start = self.scanner_current;
        self.line_start = self.line_current;
        !self.is_at_end()
    }

    pub(crate) fn get_current_char(self: &Self) -> char {
        let c = self.input_as_bytes[self.scanner_current];
        c as char
    }

    fn is_at_end(self: &Self) -> bool {
        self.scanner_current >= self.input.len()
    }

    pub(crate) fn get_lexeme(self: &Self) -> String {
        let mut text = String::new();

        for i in self.scanner_start..self.scanner_current {
            text.push(self.input.as_bytes()[i] as char)
        }

        text
    }
}