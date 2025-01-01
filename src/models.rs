use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub id: usize,
    pub text: String,
}
