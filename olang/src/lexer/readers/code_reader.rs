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
            scanner_current: 0, // current position per document. does not include last char
            scanner_start: 0, // where has the token started
            line: 0, // current line
        }
    }

    pub fn collect(&self) -> &str {
        /*
            let code = "hello"
            code[5..6] -> panic
            code[5..5] -> ""
            code[4..5] -> o
         */
        return &self.input[self.scanner_start..self.scanner_current];
    }

    pub fn advance(self: &mut Self, char_count: usize) -> bool {
        // if code.len = 5
        // if current = 5 & start = 4 we have a valid [4..5]
        // this means that line_current + char_count can be self.input.len()
        if (self.line_current + char_count) > self.input.len() {
            return false;
        }

        self.line_current += char_count;
        self.scanner_current += char_count;

        return true;
    }

    pub fn peek_one(self: &Self) -> Option<char>{
        /*
         let code = "hello"
         if current == len then [current .. current+1] means [len .. len + 1]
         code[5..5+1] -> panic
         code[4..4+1] -> "o"
      */
        return if (self.scanner_current + 1 >= self.input.len()) {
            // if 4 + 1 = 5 and code.len() = 5 we have nothing to peek
            None
        } else {
            let peeked = &self.input[self.scanner_current..self.scanner_current + 1];
            peeked.chars().next()
        }
    }

    pub fn revert_advance(self: &mut Self){
        self.scanner_current = self.scanner_start;
        self.line_current = self.line_start;
    }

    pub(crate) fn forward(self: &mut Self) {
        self.scanner_start = self.scanner_current;
        self.line_start = self.line_current;
    }

    pub fn can_advance(self: &Self) -> bool {
        // if code.len = 5
        // if current = 5 it means that a next advance would be 5 + 1
        //    which would result in current = 6 which is outside of bounds
        //
        self.scanner_current < self.input.len()
    }
    pub fn get_lexeme(self: &Self) -> String {
        let mut text = String::new();

        for i in self.scanner_start..self.scanner_current {
            text.push(self.input.as_bytes()[i] as char)
        }

        text
    }
}