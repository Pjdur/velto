use urlencoding::decode;
use std::collections::HashMap;

/// Parses a URL-encoded form body into a HashMap with percent-decoding.
pub fn parse(body: &str) -> HashMap<String, String> {
    body.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = decode(parts.next()?.trim()).ok()?.to_string();
            let val = decode(parts.next()?.trim()).ok()?.to_string();
            Some((key, val))
        })
        .collect()
}
