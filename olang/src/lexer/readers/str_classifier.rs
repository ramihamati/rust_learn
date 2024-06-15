pub struct str_classifier{

}

impl str_classifier{
    pub fn is_identifier_char(c: char) -> bool{
        return c.is_alphanumeric() || c == '_';
    }

}