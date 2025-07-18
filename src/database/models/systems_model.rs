use crate::database::models::{ge_model::GeSystems, philips_model::PhilipsSystems};
use crate::util::traits::{HasHostIp, HasSME};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Systems {
    Ge(GeSystems),
    Philips(PhilipsSystems),
}

impl Systems {
    pub fn host_ip(&self) -> Option<&std::net::IpAddr> {
        match self {
            Systems::Ge(s) => s.host_ip(),
            Systems::Philips(s) => s.host_ip(),
        }
    }
}

impl Systems {
    pub fn get_sme(&self) -> Option<&String> {
        match self {
            Systems::Ge(s) => s.get_sme(),
            Systems::Philips(s) => s.get_sme(),
        }
    }
}
