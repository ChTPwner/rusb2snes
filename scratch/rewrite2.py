import re

with open('src/lib.rs', 'r') as f:
    content = f.read()

# Fix tokio dependency
with open('Cargo.toml', 'r') as f:
    cargo = f.read()
if 'tokio = { version = "1"' not in cargo.split('[dev-dependencies]')[0]:
    cargo = cargo.replace('futures-util = "0.3.34"', 'futures-util = "0.3.34"\ntokio = { version = "1", features = ["net"] }')
    with open('Cargo.toml', 'w') as f:
        f.write(cargo)

# Fix Message::Text and Message::Binary
content = content.replace("Message::Text(json)", "Message::Text(json.into())")
content = content.replace("Message::Binary(odata)", "Message::Binary(odata.into())")

# Fix missing .await on multi-line send_command
content = re.sub(r'self\.send_command\(\s*Command::PutFile,\s*vec!\[path\.to_string\(\),\s*format!\("\{:x\}",\s*data\.len\(\)\)\],\s*\)\?;', r'self.send_command(\n            Command::PutFile,\n            vec![path.to_string(), format!("{:x}", data.len())],\n        ).await?;', content)

content = re.sub(r'self\.send_command_with_space\(\s*Command::GetAddress,\s*Space::SNES,\s*vec!\[format!\("\{:x\}",\s*address\),\s*format!\("\{:x\}",\s*size\)\],\s*\)\?;', r'self.send_command_with_space(\n            Command::GetAddress,\n            Space::SNES,\n            vec![format!("{:x}", address), format!("{:x}", size)],\n        ).await?;', content)

with open('src/lib.rs', 'w') as f:
    f.write(content)
