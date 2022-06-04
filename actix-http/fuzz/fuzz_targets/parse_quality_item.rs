#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|header: &str| {
    let _ = header.parse::<actix_http::header::QualityItem<String>>();
});
