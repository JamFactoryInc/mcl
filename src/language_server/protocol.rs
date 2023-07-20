pub struct JsonRPCMessage {

}

pub enum RequestMethod {

}

pub enum RequestMessageParam {

}

pub struct Message {
    id: i32,
    method: RequestMethod,
    params: Option<RequestMessageParam>,
}

pub enum MessageVariant {
    Message(Message),
    /// message prefix `$/`
    Notification(Message),
}

pub enum ResponseResult {

}

pub enum ResponseErrorData {

}

pub struct ErrorCode(i32);
impl ErrorCode {
    const PARSE_ERROR: i32 = -32700;
    const INVALID_REQUEST: i32 = -32600;
    const METHOD_NOT_FOUND: i32 = -32601;
    const INVALID_PARAMS: i32 = -32602;
    const INTERNAL_ERROR: i32 = -32603;

    /// This is the start range of JSON-RPC reserved error codes.
    /// It doesn't denote a real error code. No LSP error codes should
    /// be defined between the start and end range. For backwards
    /// compatibility the `ServerNotInitialized` and the `UnknownErrorCode`
    /// are left in the range.
    const JSON_RPC_RESERVED_START: i32 = -32099;


    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    const SERVER_NOT_INITIALIZED: i32 = -32002;
    const UNKNOWN_ERROR_CODE: i32 = -32001;

    /// This is the end range of JSON-RPC reserved error codes.
    /// It doesn't denote a real error code.
    const JSON_RPC_RESERVED_ERROR_RANGE_END: i32 = -32000;

    /// This is the start range of LSP reserved error codes.
    /// It doesn't denote a real error code.
    const LSP_RESERVED_ERROR_RANGE_START: i32 = -32899;

    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    const REQUEST_FAILED: i32 = -32803;

    /// The server cancelled the request. This error code should
    // only be used for requests that explicitly support being
    // server cancellable.
    const SERVER_CANCELLED: i32 = -32802;

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    const CONTENT_MODIFIED: i32 = -32801;

    /// The client has canceled a request and a server as detected
    /// the cancel.
    const REQUEST_CANCELLED: i32 = -32800;

    /// This is the end range of LSP reserved error codes.
    /// It doesn't denote a real error code.
    const LSP_RESERVED_ERROR_RANGE_END: i32 = -32800;

}

pub struct ResponseError {
    code: ErrorCode,
    message: String,
    data: Option<ResponseErrorData>
}

pub struct ResponseMessage {
    id: i32,
    result: Option<ResponseResult>,
    error: Option<ResponseError>,
}

pub enum CancelParams {

}

pub enum ProgressToken {
    Int(i32),
    Str(String)
}
pub struct ProgressParams<T> {
    /**
     * The progress token provided by the client or server.
     */
    token: ProgressToken,
    /**
     * The progress data.
     */
    value: T,
}

/// represents a character position within a text document
///
/// [Position](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)
pub struct Position {
    /**
     * Line position in a document (zero-based).
     */
    line: u32,
    /**
     * Character offset on a line in a document (zero-based). The meaning of this
     * offset is determined by the negotiated `PositionEncodingKind`.
     *
     * If the character value is greater than the line length it defaults back
     * to the line length.
     */
    character: u32,
}

/// A type indicating how positions are encoded,
/// specifically what column offsets mean.
///
/// [PositionEncodingKind](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#positionEncodingKind)
pub enum PositionEncodingKind {
    UTF8,
    UTF16,
    UTF32,
}

/// [Format](https://datatracker.ietf.org/doc/html/rfc3986)
///
/// A URI to a given text document
pub struct TextDocument(String);

/// [HoverParams](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#hoverParams)
///
/// [textDocument/hover](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_hover)
pub struct HoverParams {
    text_document: TextDocument,
    position: Position,
}

pub struct HoverResult(String);

/// [RegularExpressionsClientCapabilities](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#regExp)
pub struct RegularExpressionsClientCapabilities {
    /**
     * The engine's name.
     */
    engine: String,
    /**
     * The engine's version.
     */
    version: Option<String>,
}

pub struct Range {
    start: Position,
    end: Position,
}

pub struct TextDocumentItem {
    /**
     * The text document's URI.
     */
    uri: TextDocument,

    /**
     * The text document's language identifier.
     */
    language_id: String,

    /**
     * The version number of this document (it will increase after each
     * change, including undo/redo).
     */
    version: i32,

    /**
     * The content of the opened text document.
     */
    text: String,
}

