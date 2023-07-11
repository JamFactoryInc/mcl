use std::fs::File;
use std::future::Future;
use std::io::{BufReader, Read};
use std::mem;
use futures::executor::block_on;

pub struct Source {
    pub absolute_index: usize,
    pub line: usize,
    pub index: isize,
    pub max_index: usize,
    cursor: usize,
    buffer: [u8; 1024],
    buffer_next: [u8; 1024],
    reader: BufReader<File>,
    read_promise: Option<Box<dyn Future<Output = usize>>>,
    pub current_line: Vec<u8>,
}

impl Source {

    pub fn skip(&mut self) {

    }

    pub fn peek(&self) -> u8 {
        self.buffer[self.cursor]
    }

    fn inc_cursor(&mut self) {
        if self.cursor < self.max_index {
            self.cursor += 1;
        } else {
            self.cursor = 0;
            self.max_index = block_on(self.read_promise.unwrap());
            mem::swap(&mut self.buffer, &mut self.buffer_next);
            self.read_promise.unwrap()
        }
    }

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

    async fn buffer_next(&mut self) -> usize {
        match self.reader.read(&mut self.buffer_next) {
            Ok(bytes) => {
                bytes - 1
            },
            Err(e) => panic!("{}", e)
        }
    }
}

impl Iterator for Source {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.buffer[self.cursor];
        self.handle_increment(next);

        self.inc_cursor()
    }
}