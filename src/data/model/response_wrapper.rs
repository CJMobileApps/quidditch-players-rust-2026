#[derive(Debug, serde::Serialize)]
pub struct ResponseWrapper<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,

    pub status_code: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct ResponseError {
    pub is_error: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub trait ResponseWrapperTrait {
    fn to_response_wrapper(&self) -> ResponseWrapper<()>;
}
