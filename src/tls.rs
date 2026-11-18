use std::{net::TcpStream, time::Duration};

use crate::{dns, error::AppError};

pub struct CertificateInfo {
    pub der_bytes: usize,
}

pub fn connect(host: &str, port: u16) -> Result<CertificateInfo, AppError> {
    let addresses = dns::resolve_addrs(host, port)?;
    let mut last_error = None;
    let stream = addresses.into_iter().find_map(|address| {
        match TcpStream::connect_timeout(&address, Duration::from_secs(10)) {
            Ok(stream) => Some(stream),
            Err(error) => {
                last_error = Some(error);
                None
            }
        }
    });
    let stream =
        stream.ok_or_else(|| {
            AppError::Io(last_error.unwrap_or_else(|| {
                std::io::Error::other("could not connect to any resolved address")
            }))
        })?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;

    let connector = native_tls::TlsConnector::new()?;
    let connection = connector
        .connect(host, stream)
        .map_err(|error| AppError::InvalidInput(format!("TLS handshake failed: {error}")))?;
    let certificate = connection
        .peer_certificate()?
        .ok_or_else(|| AppError::InvalidInput("server did not provide a certificate".to_owned()))?;
    let der_bytes = certificate.to_der()?.len();

    Ok(CertificateInfo { der_bytes })
}
