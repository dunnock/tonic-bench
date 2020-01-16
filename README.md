This package contains naive implementation of benchmarks for [tonic](https://docs.rs/tonic/0.1.0/tonic/index.html)

# Run load test

```
cargo run --bin=server --release
```

Load with empty request/response

```
cargo run --bin=client --release -- -m=100000 -c=4
```
```
Elapsed: 2632ms
processed 40000 with 15198 rps
```

OR Load with string request/response

```
cargo run --bin=client --release -- -c=4 -m=100000 -r=Something
```
```
Elapsed: 4264ms
processed 40000 with 9381 rps
```

# Run with flamegraph

```
sudo flamegraph target/release/server
```

![Server flamegraph with string request/response](https://raw.githubusercontent.com/dunnock/tonic-bench/master/flamegraph-server-something.svg?sanitize=true)