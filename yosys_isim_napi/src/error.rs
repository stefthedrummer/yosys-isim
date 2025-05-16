pub struct JsError {
    message: String,
}

impl From<yosys_isim::common::SimError> for JsError {
    fn from(value: yosys_isim::common::SimError) -> Self {
        JsError {
            message: format!("{:?}", value),
        }
    }
}

impl From<JsError> for napi::JsError {
    fn from(value: JsError) -> Self {
        napi::Error::new(napi::Status::GenericFailure, value.message).into()
    }
}
