use lsp_types::{error_codes, OneOf}

pub trait Method {
    /// something like `window/` or `$/`
    const NAMESPACE: &'static str;
    /// something like `cancelRequest`
    const METHOD: &'static str;
    type Params;
}

pub trait RequestMethod : Method {
    type Response;
}

pub struct RequestMessage<T : Method> {
    id: OneOf<i32, String>,
    params: Option<T::Params>
}

pub struct ResponseError<T> {
    code: i32,
    message: String,
    data: Option<T>
}

pub enum MessageIdentifier {


}

pub enum LifecycleIdentifier {
    Initialize,
    Initialized,
    RegisterCapability,
    UnregisterCapability,
    SetTrace,
    LogTrace,
    Shutdown,
    Exit,
}