use crate::database::models::ge_model::GeSystems;
use crate::database::models::philips_model::PhilipsSystems;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Systems {
    Ge(GeSystems),
    Philips(PhilipsSystems),
}
