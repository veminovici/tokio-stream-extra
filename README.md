# Tokio-Stream-Extra
A crate that extends the [Stream](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) trait. For more details about streams please check the [tokio-stream](https://crates.io/crates/tokio-stream) crate as well as [StreamExt](https://docs.rs/tokio-stream/0.1.11/tokio_stream/trait.StreamExt.html) trait documentation.

[![Rust](https://github.com/veminovici/tokio-stream-extra/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/tokio-stream-extra/actions/workflows/ci.yml)
![Crates.io](https://img.shields.io/crates/v/tokio-stream-extra)
![Crates.io](https://img.shields.io/crates/l/tokio-stream-extra)
![Crates.io](https://img.shields.io/crates/d/tokio-stream-extra)

## Examples

### Split
Splits this stream's items at a separation item. The separation item
is determined by provided closure. A stream of vectors of item type will be returned,
which will yield elements until the closure returns `None`.

```rust
[tokio::main]
async fn main() {
    use tokio_stream::{self as stream, StreamExt};
    use tokio_stream_extra::StreamExtra;

    let stream = stream::iter(vec![1,2,0,3,4,0]);
    let mut stream = stream.split(|x| x == &0);

    assert_eq!(stream.next().await, Some(vec![1,2]));
    assert_eq!(stream.next().await, Some(vec![3,4]));
```

## Tests

### Test Coverage
To get the test coverage, I use the [grcov](https://github.com/mozilla/grcov#how-to-get-grcov).
See the instructions [steps](https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project).

```bash
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./coverage/lib-%p-%m.profraw"
cargo build
cargo test
grcov ./coverage -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
open ./target/debug/coverage/index.html
```

### Property Based Testing
The library is using property based testing. It uses the [quickcheck](https://docs.rs/quickcheck/latest/quickcheck/) crate.


## About

> Code designed and written on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.