#![no_main]
use actix_codec::Decoder;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let local = tokio::task::LocalSet::new();

    local.block_on(&rt, async move {
        let mut bytes = bytes::BytesMut::from(data);
        let mut codec = actix_http::h1::Codec::default();

        loop {
            let prev_size = bytes.len();
            let r = codec.decode(&mut bytes);
            if r.is_err() || bytes.len() == 0 || bytes.len() >= prev_size {
                break;
            }
        }
    });
});
