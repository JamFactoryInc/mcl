use jsonrpc_core::serde::{Deserialize, Deserializer, Serialize, Serializer};
use lsp_types::{OneOf};

pub trait Notification {
    /// something like `window/` or `$/`
    const NAMESPACE: &'static str;
    /// something like `cancelRequest`
    const METHOD: &'static str;
    type Params;
}

pub trait Request : Notification {
    type Response: Serialize;
}

pub struct RequestMessage<T : Request> {
    id: OneOf<i32, String>,
    params: Option<T::Params>
}

pub struct ResponseError<T: Serialize> {
    code: i32,
    message: String,
    data: Option<T>
}
impl<T: Serialize> Serialize for ResponseError<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_struct("")
    }
}