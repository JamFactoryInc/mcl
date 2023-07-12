use std::fs::File;
use std::future::Future;
use std::io::{BufReader, Read};
use std::mem;
use std::pin::Pin;
use futures::executor::block_on;

pub struct Source {
    pub absolute_index: usize,
    pub line: usize,
    pub max_index: usize,
    cursor: usize,
    buffer: [u8; 1024],
    buffer_next: [u8; 1024],
    reader: BufReader<File>,
    read_promise: Option<Pin<Box<dyn Future<Output = usize> + 'static>>>,
    pub current_line: Vec<u8>,
}

impl Source {

    pub fn get_index(&self) -> usize {
        self.current_line.len()
    }

    #[inline(always)]
    pub fn peek(&self) -> u8 {
        self.buffer[self.cursor]
    }

    fn inc_cursor(&mut self, current: u8) -> Option<u8> {
        if self.cursor < self.max_index {
            self.cursor += 1;
        } else if self.cursor == self.max_index {
            return None
        } else {
            self.cursor = 0;
            let new_max_index = block_on((self.read_promise).take().unwrap());
            match new_max_index {
                usize::MAX => return None,
                len @ _ => {
                    mem::swap(&mut self.buffer, &mut self.buffer_next);
                    //self.read_promise = Some(Box::pin(self.buffer_next()));
                    //self.max_index = len;

                },
            };
        }
        Some(current)
    }

    fn handle_increment(&mut self, next: u8) {
        match next {
            b'\n' => {
                self.current_line.clear();
                self.line += 1;
            },
            _ => {
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
            Err(_) => usize::MAX
        }
    }
}

impl Iterator for Source {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.buffer[self.cursor];
        self.handle_increment(next);
        self.inc_cursor(next)
    }
}