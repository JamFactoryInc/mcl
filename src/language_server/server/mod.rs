mod request;
mod response;

use jsonrpc_core::{IoHandler, Params};
use crate::compiler::Compiler;

pub struct LanguageServer {
    io: IoHandler,
    compiler: Compiler,

}

impl LanguageServer {
    pub fn new() -> LanguageServer {
        let mut ls = LanguageServer {
            io: IoHandler::new(),
            compiler: Compiler {},
        };

        ls.io.add_method("something", |x| -> {

        });

        Params


        todo!()
    }
}




