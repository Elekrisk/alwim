use std::{str::Chars, io::Read};



pub struct Lexer<'a> {
    code: CodeStorage<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code: CodeStorage::new(code)
        }
    }


}


struct CodeStorage<'a> {
    iter: Chars<'a>,
    just_given: Option<char>,
    back_buffer: Vec<char>
}

impl<'a> CodeStorage<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            iter: code.chars(),
            just_given: None,
            back_buffer: vec![]
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.back_buffer.len() > 0 {
            Some(self.back_buffer.remove(0))
        } else {
            let c = self.iter.next()?;
            self.just_given = Some(c);
            Some(c)
        }
    }

    fn back(&mut self) -> Option<()> {
        if let Some(c) = self.just_given {
            self.back_buffer.push(c);
            Some(())
        } else {
            None
        }
    }
}
