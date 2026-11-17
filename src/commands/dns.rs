use std::{fmt, net::IpAddr};

use serde::Serialize;

use crate::{dns, error::AppError};

#[derive(Debug, Serialize)]
pub struct DnsInfo {
    pub host: String,
    pub addresses: Vec<IpAddr>,
}

impl fmt::Display for DnsInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Host: {}", self.host)?;
        writeln!(formatter, "Addresses:")?;
        for address in &self.addresses {
            writeln!(formatter, "  {address}")?;
        }
        Ok(())
    }
}

pub fn run(host: &str) -> Result<DnsInfo, AppError> {
    Ok(DnsInfo {
        host: host.to_owned(),
        addresses: dns::resolve(host)?,
    })
}
