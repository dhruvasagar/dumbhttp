# Dumb Http v0.1.0

A dumb http server that responds with a preconfigured http response

## Usage

### Running with no arguments always returns 200 on all interfaces at port 3000

```
dumbhttp
curl localhost:3000 -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 0
date: Wed, 16 Mar 2022 04:46:46 GMT

```

### Always respond with 400 Bad Request

```
STATUS=400 dumbhttp
curl localhost:3000 -i
HTTP/1.1 400 Bad Request
content-type: application/json
content-length: 0
date: Wed, 16 Mar 2022 04:48:38 GMT

```

### Always return a specific body

```
BODY='{"ok": true, "view": {}}' dumbhttp
curl localhost:3000 -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 24
date: Wed, 16 Mar 2022 04:49:40 GMT

{"ok": true, "view": {}}
```
## Installation

### Using Cargo

```
cargo install dumbhttp
```

### Using Docker

```
docker run --rm -d -p 3000:3000 -it dhruvasagar/dumbhttp
```
