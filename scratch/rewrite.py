import re

with open('src/lib.rs', 'r') as f:
    content = f.read()

# Imports
content = content.replace("use std::net::TcpStream;", "use tokio::net::TcpStream;\nuse tokio_tungstenite::{WebSocketStream, MaybeTlsStream};\nuse futures_util::{SinkExt, StreamExt};\nuse tokio_tungstenite::tungstenite::Message;")
content = content.replace("use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};", "")

# Struct rename
content = content.replace("SyncClient", "AsyncClient")
content = content.replace("client: WebSocket<MaybeTlsStream<TcpStream>>", "client: WebSocketStream<MaybeTlsStream<TcpStream>>")

# Box<dyn Error> -> Box<dyn Error + Send + Sync>
content = content.replace("Box<dyn Error>", "Box<dyn Error + Send + Sync>")

# async fn
content = re.sub(r'pub fn (connect|connect_with_devel|set_name|app_version|list_device|attach|info|reset|menu|boot|ls|send_file|get_file|remove_path|get_address|get_multi_address_as_vec|get_multi_address_from_pairs)', r'pub async fn \1', content)
content = re.sub(r'fn (send_command|send_command_with_space|get_reply|parse_multi_addresses)', r'async fn \1', content)

# connect
content = content.replace("let (client, _) = connect(ws).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;", "let (client, _) = tokio_tungstenite::connect_async(&ws).await?;")
content = content.replace("AsyncClient::connect(endpoint)?", "AsyncClient::connect(endpoint).await?")

# send
content = content.replace("Message::text", "Message::Text")
content = content.replace("Message::binary", "Message::Binary")
content = content.replace("self.client.send(message)?;", "self.client.send(message).await?;")

# get_reply
content = content.replace("let reply = self.client.read()?;", "let reply = self.client.next().await.ok_or(\"WebSocket stream closed unexpectedly\")??;")
content = content.replace("self.get_reply()?", "self.get_reply().await?")

# read loop in get_file, get_address, parse_multi_addresses
content = content.replace("let reply = self.client.read()?;", "let reply = self.client.next().await.ok_or(\"WebSocket stream closed unexpectedly\")??;")

# await for send_command and send_command_with_space
content = re.sub(r'self.send_command\((.*?)\)\?;', r'self.send_command(\1).await?;', content)
content = re.sub(r'self.send_command_with_space\((.*?)\)\?;', r'self.send_command_with_space(\1).await?;', content)
content = content.replace("self.parse_multi_addresses(total_size)?", "self.parse_multi_addresses(total_size).await?")

with open('src/lib.rs', 'w') as f:
    f.write(content)
