use serde::Serialize;


#[derive(Clone, Serialize, Debug, Default)]
pub struct LimitResp {
    pub is_ok: bool,
    pub count: usize,
}