use error::Error;

pub trait Read {
	fn copy(& self) -> Self;
    fn next(&mut self) -> Result<Option<u8>, Error>;
    fn peek(&mut self) -> Result<Option<u8>, Error>;
    fn discard(&mut self);
    fn position(&self) -> Position;
    fn peek_position(&self) -> Position;
}

#[derive(Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy)]
pub struct SliceReader<'a> {
    slice: &'a [u8],
    index: usize,
    position: Position,
}

impl<'a> SliceReader<'a> {

    pub fn new(slice: &'a [u8]) -> Self {
        SliceReader {
            slice: slice,
            index: 0,
            position : Position{line: 0, column: 0}
        }
    }
}

impl<'a> Read for SliceReader<'a> {
    fn copy(&self) -> Self {
        SliceReader{slice: self.slice, index: self.index, position: self.position}
    }

    fn next(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.slice.len() {
            self.position.column += 1;
            let c = self.slice[self.index];
            self.index += 1;
            Ok(Some(c))
        }
        else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.slice.len() {
            Ok(Some(self.slice[self.index]))
        }
        else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.index += 1;
    }

    fn position(&self) -> Position {
        Position{line: self.position.line, column: self.position.column}
    }

    fn peek_position(&self) -> Position {
        Position{line: self.position.line, column: self.position.column}
    }
}