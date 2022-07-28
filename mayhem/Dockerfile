# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd actix-http/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv actix-http/fuzz/target/x86_64-unknown-linux-gnu/release/parse_quality_item /parse_quality_item && \
    mv actix-http/fuzz/target/x86_64-unknown-linux-gnu/release/decode_h1 /decode_h1 && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd actix-http/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv actix-http/fuzz/target/release/parse_quality_item /parse_quality_item_no_inst && \
    mv actix-http/fuzz/target/release/decode_h1 /decode_h1_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /parse_quality_item /parse_quality_item_no_inst /
COPY --from=builder /decode_h1 /decode_h1_no_inst /
