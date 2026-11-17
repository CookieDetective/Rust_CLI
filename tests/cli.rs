use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
};

fn site_scout() -> Command {
    Command::new(env!("CARGO_BIN_EXE_site-scout"))
}

#[test]
fn help_lists_the_network_commands() {
    let output = site_scout().arg("--help").output().expect("run CLI");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("inspect"));
    assert!(stdout.contains("dns"));
    assert!(stdout.contains("tls"));
}

#[test]
fn dns_command_can_return_json_for_localhost() {
    let output = site_scout()
        .args(["dns", "localhost", "--json"])
        .output()
        .expect("run DNS command");
    let result: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("DNS output should be JSON");

    assert!(output.status.success());
    assert_eq!(result["host"], "localhost");
    assert!(
        result["addresses"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
}

#[test]
fn inspect_rejects_non_http_schemes() {
    let output = site_scout()
        .args(["inspect", "ftp://example.com"])
        .output()
        .expect("run inspect command");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("scheme must be http or https"));
}

#[test]
fn inspect_fetches_a_local_page() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
    let address = listener.local_addr().expect("local server address");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept CLI request");
        let mut request = [0; 1024];
        let _ = stream.read(&mut request).expect("read CLI request");
        let body = "<html><title>Local test page</title></html>";
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .expect("write local response");
    });

    let output = site_scout()
        .args(["inspect", &format!("http://{address}/")])
        .output()
        .expect("run inspect command");
    server.join().expect("local server should finish");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Local test page"));
}
