build:
    docker run --rm -v $(pwd):/workspace rust:1.95 cargo build --release --manifest-path /workspace/Cargo.toml