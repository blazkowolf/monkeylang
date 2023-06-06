#[derive(Default)]
pub struct Lexer {
    /// Source code string
    input: String,
    /// Current position in `input` (points to current char)
    position: usize,
    /// Current reading position in `input` (after current char)
    read_position: usize,
    /// Current char under examination
    ch: u8,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
