use std::{
    collections::BTreeSet,
    net::{IpAddr, SocketAddr, ToSocketAddrs},
};

use crate::error::AppError;

pub fn resolve(host: &str) -> Result<Vec<IpAddr>, AppError> {
    let addresses = resolve_addrs(host, 0)?;
    Ok(addresses
        .into_iter()
        .map(|address| address.ip())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect())
}

pub(crate) fn resolve_addrs(host: &str, port: u16) -> Result<Vec<SocketAddr>, AppError> {
    if host.trim().is_empty() {
        return Err(AppError::InvalidInput("host cannot be empty".to_owned()));
    }
    let addresses = (host, port).to_socket_addrs()?.collect::<BTreeSet<_>>();
    if addresses.is_empty() {
        return Err(AppError::InvalidInput(format!(
            "no addresses found for {host}"
        )));
    }
    Ok(addresses.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::resolve;

    #[test]
    fn resolves_localhost() {
        let addresses = resolve("localhost").expect("localhost should resolve");
        assert!(!addresses.is_empty());
    }

    #[test]
    fn rejects_an_empty_host() {
        assert!(resolve(" ").is_err());
    }
}
