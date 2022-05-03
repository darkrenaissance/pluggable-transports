# Pluggable transports

## Design considerations

### Dialer high-level API

Schemes could be:

* `tcp://`
* `tcp+tls://`
* `tor://`
* `tor+tls://`
* etc.

Example usage:

```rust
let url = Url::parse("tcp://123.123.123.123:1234")?;

// Dialer should be transparent for any scheme

let dialer = Dialer::new(url).await?;
let mut stream = dialer.dial().await?;
stream.write_all(b"hello").await?;
let mut buf = vec![0_u8; 5];
stream.read_exact(&mut buf).await?;
assert_eq!(buf, b"hello");
```

### Dialer instantiation

* Option 1:

```rust
async fn new(url: Url) -> Dialer {
    let transport = match url.scheme() {
        "tcp" => {
            // Build a TCP transport
        }
        "tcp+tls" => {
            // Build a TCP transport wrapped with TLS
        }
        "tor" => {
            // Build a Tor transport
        }
        "tor+tls" => {
            // Build a Tor transport wrapped with TLS
        }
    }

    return transport
}
```

* Option 2:

```rust
async fn new(url: Url) -> Dialer {
    let (base, upgrade) = url.scheme().split('+');

    let transport = match base {
        "tcp" => // Build a TCP transport
        "tor" => // Build a Tor transport
    };

    if let Some(upgrade) {
        transport.upgrade("tls");
    }

    return transport
}
```

### Listener high-level APi

Schemes could be:

* `tcp://`
* `tcp+tls://`
* `tor://`
* `tor+tls://`
* etc.

Example usage:

* Option 1:

```rust
let url = Url::parse("tcp://123.123.123.123:1234")?;

// Listener should be transparent for any scheme

let listener = Listener::new(url).await?;
let mut incoming = listener.listen().await?;
while let Some(stream) = incoming.next().await {
    let stream = stream.unwrap();
    let (reader, writer) = &mut (&stream, &stream);
    io::copy(reader, writer).await.unwrap();
}
```

* Option 2:

```rust
let url = Url::parse("tcp://123.123.123.123:1234")?;

// Listener should be transparent for any scheme

let listener = Listener::new(url).await?;
loop {
    let stream = listener.accept().await?;
    let (mut reader, mut writer) = smol::io::split(stream);
    io::copy(&mut reader, &mut writer).await?;
}
```
