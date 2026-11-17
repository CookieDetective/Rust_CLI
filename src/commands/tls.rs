use std::fmt;

use serde::Serialize;

use crate::{error::AppError, tls};

#[derive(Debug, Serialize)]
pub struct TlsInfo {
    pub host: String,
    pub port: u16,
    pub certificate_der_bytes: usize,
}

impl fmt::Display for TlsInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "TLS connection: {}:{}", self.host, self.port)?;
        write!(
            formatter,
            "Certificate size: {} bytes",
            self.certificate_der_bytes
        )
    }
}

pub fn run(host: &str, port: u16) -> Result<TlsInfo, AppError> {
    let certificate = tls::connect(host, port)?;
    Ok(TlsInfo {
        host: host.to_owned(),
        port,
        certificate_der_bytes: certificate.der_bytes,
    })
}
