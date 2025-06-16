use crate::database::models::ge_model::GeSystems;
use crate::database::models::philips_model::PhilipsSystems;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Systems {
    Ge(GeSystems),
    Philips(PhilipsSystems),
}
