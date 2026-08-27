use rusb2snes::AsyncClient;
use rusb2snes::USB2SnesEndpoint;

#[tokio::main]
async fn main() {
    let endpoint = USB2SnesEndpoint::default();
    let mut usb2snes = AsyncClient::connect(&endpoint).await.unwrap();

    usb2snes.set_name(String::from("example")).await.unwrap();

    let devices = usb2snes.list_device().await.unwrap();

    usb2snes.attach(&devices[0]).await.unwrap();
    let info = usb2snes.info().await.unwrap();
    println!("Attached to {} - {}", info.dev_type, info.version);
    let address = usb2snes.get_address(0xF5008B, 2).await.unwrap();
    for elt in address {
        dbg!(elt);
    }
}
