# Migration Plan: Asynchronous `rusb2snes`

This document outlines the step-by-step plan to migrate the `rusb2snes` crate from a synchronous execution model to an asynchronous one. 

## 1. Update Dependencies in `Cargo.toml`
Currently, the crate uses the blocking `tungstenite` library. We need to replace it with its async equivalent, `tokio-tungstenite`. We will also need the `futures-util` crate to interact with async streams and sinks.

```toml
[dependencies]
strum_macros = "0.27"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
# Replace `tungstenite` with `tokio-tungstenite`
tokio-tungstenite = "0.26" # Or latest compatible version
futures-util = "0.3"
```
*Note: For testing and running examples, you should also add `tokio = { version = "1", features = ["macros", "rt-multi-thread"] }` under `[dev-dependencies]`.*

## 2. Refactor the Client Struct
In `src/lib.rs`, rename `SyncClient` to `Client` (or `AsyncClient`) and update the underlying WebSocket client type. `tokio-tungstenite` wraps an underlying async stream (like `tokio::net::TcpStream`).

```rust
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

pub struct AsyncClient {
    client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    devel: bool,
}
```

## 3. Make Connection Initialization Asynchronous
The `connect` and `connect_with_devel` methods must become `async fn` and use `tokio_tungstenite::connect_async`.

```rust
impl AsyncClient {
    pub async fn connect(endpoint: &USB2SnesEndpoint) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let ws = format!("ws://{}:{}", endpoint.address, endpoint.port);
        let (client, _response) = tokio_tungstenite::connect_async(&ws).await?;
        
        Ok(Self {
            client,
            devel: false,
        })
    }
    
    // Similarly update connect_with_devel
}
```
*Tip: When moving to async, it's good practice to ensure errors are thread-safe by using `Box<dyn Error + Send + Sync>` instead of `Box<dyn Error>`.*

## 4. Adapt Core Sending/Receiving Methods
You will need to import `SinkExt` and `StreamExt` from `futures_util` to call `.send()` and `.next()` on the WebSocket stream.

```rust
use futures_util::{SinkExt, StreamExt};

// In impl AsyncClient:
async fn send_command_with_space(
    &mut self,
    command: Command,
    space: Space,
    args: Vec<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // ... setup JSON ...
    let message = tokio_tungstenite::tungstenite::Message::Text(json);
    self.client.send(message).await?;
    Ok(())
}

async fn get_reply(&mut self) -> Result<USB2SnesResult, Box<dyn Error + Send + Sync>> {
    // `next()` returns Option<Result<Message, Error>>
    let reply = self.client.next().await
        .ok_or("WebSocket stream closed unexpectedly")??;
        
    let textreply = match reply {
        tokio_tungstenite::tungstenite::Message::Text(value) => value,
        _ => return Err("Error getting a reply text".into()),
    };
    Ok(serde_json::from_str(&textreply)?)
}
```

## 5. Propagate `async`/`await` Through the Public API
Every public method that interacts with the WebSocket needs to become an `async fn`. You'll need to append `.await` on all calls to `send_command(...)` and `get_reply(...)`.

Methods to update:
- `set_name`
- `app_version`
- `list_device`
- `attach`
- `info`
- `reset`
- `menu`
- `boot`
- `ls`
- `send_file`
- `get_file`
- `remove_path`
- `get_address`
- `get_multi_address_as_vec`
- `get_multi_address_from_pairs`

For example:
```rust
pub async fn app_version(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
    self.send_command(Command::AppVersion, vec![]).await?;
    let usbreply = self.get_reply().await?;
    Ok(usbreply.Results[0].to_string())
}
```

## 6. Update Tests and Examples
Any existing tests (in `tests/`) or examples (in `examples/get_address.rs`) will break because they expect a synchronous API.
1. Add `tokio` to your `[dev-dependencies]` in `Cargo.toml`.
2. Wrap your example `main` functions and test functions with `#[tokio::main]` or `#[tokio::test]`.
3. Add `.await` to all `Client` interactions within those files.

## Summary of Benefits
- Non-blocking I/O allows integration into larger modern Rust UI/Web backends (like Tauri, Axum, or Tokio-based apps) without hanging the main thread.
- Better connection handling for long-lived operations (like large file transfers or continuous memory scraping).
