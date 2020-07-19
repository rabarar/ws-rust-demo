# ws-rust-demo

### Demo

start the server

```
$ ./target/release/ws
```

start the client

```
$ websocat ws://localhost:9001
```


### using wss

Generate cert/key

use Common Name = 127.0.0.1

```
$ openssl req -x509 -sha256 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365'
```

Generate pkcs12
```
$ openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem
```

Add the indentity to the Keychain and manually mark it as trusted

start the client
```
$ websocat wss://127.0.0.1:8443
```
