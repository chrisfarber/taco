# an experiment in rust

This repository is my first attempt at learning to write Rust.

## What is it?

Right now, a RESTy key value store. Data is stored in sqlite, and the API is built atop tokio,
hyper, and axum.

It compiles down to ~7MB and uses ~4MB RAM. Neat.

## Examples

```
curl  --data '{"msg":"hello!."}' --header 'Content-Type: application/json' http://localhost:3000/json-echo
{"msg":"hello!."}%
```

```
curl http://localhost:3000/kv/some-key --data "this is some data"

curl http://localhost:3000/kv/some-key
this is some data%
```

```
curl -v http://localhost:3000/kv/this-key-does-not-exist
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /kv/this-key-does-not-exist HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.79.1
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 404 Not Found
< content-length: 0
< date: Sat, 03 Sep 2022 01:13:42 GMT
<
* Connection #0 to host localhost left intact
```
