This package contains naive implementation of benchmarks for [tonic](https://docs.rs/tonic/0.1.0/tonic/index.html)

# Test with ghz

```
cargo run --bin=server --release
```

> Rust version will start on port 50051


```
ghz  --insecure --proto proto/hellobench.proto --call hellobench.Greeter.SayEmpty -d '{}' -n 100000 -c 16 --connections=4 localhost:50051
```
```
Summary:
  Count:        100000
  Total:        6.26 s
  Slowest:      100.07 ms
  Fastest:      0.16 ms
  Average:      0.96 ms
  Requests/sec: 15984.89
```

## Compile and run go version

```
cd go
go build server.go
server
```

> Go version will start on port 50050, so above ghz should be amended:

```
ghz  --insecure --proto proto/hellobench.proto --call hellobench.Greeter.SayEmpty -d '{}' -n 100000 -c 16 --connections=4 localhost:50051
```
```
Summary:
  Count:        100000
  Total:        4.57 s
  Slowest:      14.45 ms
  Fastest:      0.11 ms
  Average:      0.67 ms
  Requests/sec: 21890.10
```


# Test with Rust client


Load with empty request/response

```
target/release/client --connections=4 --concurency=8 --request=Empty --messages=100000 --port=50051
```
```
Elapsed: 20022ms
processed 400000 with 19978 rps
successful 400000 failed 0 requests
```

OR Load with string request/response

```
target/release/client --connections=4 --concurency=8 --request=Something --messages=100000 --port=50051
```
```
Elapsed: 22059ms
processed 400000 with 18133 rps
successful 400000 failed 0 requests
```

# Run with flamegraph

```
sudo flamegraph target/release/server
```

![Server flamegraph with string request/response](https://raw.githubusercontent.com/dunnock/tonic-bench/master/flamegraph-server-empty.svg?sanitize=true)


# Comparison with plain text hyper http

Simple naive hyper HTTP server on same machine gives about ~3x more requests/responses

```
cargo run --release --bin=http_server
```
```
❯ ab -n 100000 -c 4 -k http://127.0.0.1:3000/
...
Concurrency Level:      4
Time taken for tests:   3.125 seconds
Requests per second:    32004.64 [#/sec] (mean)
Transfer rate:          3500.51 [Kbytes/sec] received
```

![HTTP1 plaintext server flamegraph](https://raw.githubusercontent.com/dunnock/tonic-bench/master/flamegraph-http-server.svg?sanitize=true)
