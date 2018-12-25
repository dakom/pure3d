use wasm_bindgen::prelude::JsValue; 

pub enum Error {
    String(String),
    Js(JsValue),
    Native(NativeError)
}

pub enum NativeError {
    CanvasCreate,
    AttributeLocation
}

impl Error {
    pub fn to_js(self:&Self) -> JsValue {
        match self {
            Error::String(s) => JsValue::from_str(&s[..]),
            Error::Js(jval) => jval.clone(),
            Error::Native(err) => JsValue::from_str(err.default_str()),
        }
    }
}


impl NativeError {
    pub fn default_str (self:&Self) -> &'static str{
        match self {
            NativeError::CanvasCreate => "Couldn't create canvas!",
            NativeError::AttributeLocation => "Couldn't get attribute location!"
        }
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        err.to_js()
    }
}

impl From<NativeError> for Error {
    fn from(err: NativeError) -> Self {
        Error::Native(err)
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Error::Js(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::String(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}

/* TODO: this doesn't work, but maybe it could!
 * idea is to consolidate str and String into one impl
impl From<Borrow<str>> for Error 
{
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}
*/
