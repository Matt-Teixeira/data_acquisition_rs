use std::net::IpAddr;

pub trait HasHostIp {
    fn host_ip(&self) -> Option<&IpAddr>;
}

pub trait HasSME {
    fn get_sme(&self) -> Option<&String>;
}