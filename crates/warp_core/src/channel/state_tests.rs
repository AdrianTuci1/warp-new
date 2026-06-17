use std::borrow::Cow;

use super::{derive_http_origin_from_ws_url, sanitize_loopback_zero_port_url};

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.localhost:8080"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.localhost:8080").is_none());
}

#[test]
fn loopback_zero_port_falls_back_to_known_good_url() {
    let got = sanitize_loopback_zero_port_url(
        Cow::Borrowed("http://localhost:0"),
        Cow::Borrowed("http://localhost:8080"),
        "server root URL",
    );
    assert_eq!(&*got, "http://localhost:8080");
}

#[test]
fn non_loopback_zero_port_is_left_unchanged() {
    let got = sanitize_loopback_zero_port_url(
        Cow::Borrowed("http://example.com:0"),
        Cow::Borrowed("http://localhost:8080"),
        "server root URL",
    );
    assert_eq!(&*got, "http://example.com:0");
}

#[test]
fn ipv6_loopback_zero_port_falls_back_to_known_good_url() {
    let got = sanitize_loopback_zero_port_url(
        Cow::Borrowed("http://[::1]:0"),
        Cow::Borrowed("http://localhost:8080"),
        "server root URL",
    );
    assert_eq!(&*got, "http://localhost:8080");
}

#[test]
fn ordinary_loopback_port_is_left_unchanged() {
    let got = sanitize_loopback_zero_port_url(
        Cow::Borrowed("http://localhost:8081"),
        Cow::Borrowed("http://localhost:8080"),
        "server root URL",
    );
    assert_eq!(&*got, "http://localhost:8081");
}
