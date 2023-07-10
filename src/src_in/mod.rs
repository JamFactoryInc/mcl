use std::fs::File;
use std::io::{BufReader, Read};

pub struct Source {
    pub absolute_index: usize,
    pub line: usize,
    pub index: isize,
    pub size: usize,
    cursor: usize,
    buffer: [u8; 2048],
    reader: BufReader<File>,
    pub current_line: Vec<u8>,
}

impl Source {
    pub fn peek(&self) -> u8 {
        self.buffer[self.cursor]
    }

    #[inline(always)]
    fn handle_increment(&mut self, next: u8) {
        match next {
            b'\n' => {
                self.current_line.clear();
                self.line += 1;
                self.index = -1;
            },
            _ => {
                self.index += 1;
                self.absolute_index += 1;
                self.current_line.push(next)
            }
        }
    }

    fn buffer(&mut self) -> Option<u8> {
        match self.reader.read(&mut self.buffer) {
            Ok(bytes) => {
                self.size = bytes;
                return if bytes == 0 {
                    None
                } else {
                    let next = self.buffer[0];

                    self.handle_increment(next);

                    self.cursor = 1;
                    Some(next)
                }

            },
            Err(e) => panic!("{}", e)
        }
    }
}

impl Iterator for Source {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.size {
            let next = self.buffer[self.cursor];

            self.handle_increment(next);

            self.cursor += 1;

            Some(next)
        } else {
            self.buffer()
        }
    }
}