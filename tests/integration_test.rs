use core::time;
use rusb2snes::{AsyncClient, Infos, USB2SnesEndpoint, USB2SnesFileInfo};
use std::error::Error;
use std::fs;

const CLIENT_NAME: &str = "TESTS";
const TEST_FILE: &str = "240pSuite_test.sfc";

#[cfg(test)]
mod tests {

    use super::*;
    async fn test_connect() -> Result<AsyncClient, Box<dyn Error + Send + Sync>> {
        let endpoint = USB2SnesEndpoint::default();
        let mut usb2snes = AsyncClient::connect(&endpoint).await?;
        usb2snes.set_name(CLIENT_NAME.to_string()).await?;
        let devices = usb2snes.list_device().await?;
        usb2snes.attach(&devices[0]).await?;
        Ok(usb2snes)
    }
    #[tokio::test]
    async fn t01_menu_info() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let info = usb2snes.info().await?;
        let expected = Infos {
            version: "1.11.0".to_string(),
            dev_type: "FXPAK PRO STM32".to_string(),
            game: "/sd2snes/m3nu.bin".to_string(),
            flags: vec![],
        };

        match expected == info {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, info).into()),
        }
    }

    #[tokio::test]
    async fn t20_send_test_file() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let local_path = format!("files/{TEST_FILE}");
        let data = fs::read(&local_path).expect("Error opening the file or reading the content");
        usb2snes.send_file(&TEST_FILE.to_string(), data).await?;
        tokio::time::sleep(time::Duration::from_secs(2)).await;
        Ok(())
    }

    #[tokio::test]
    async fn t21_check_test_file() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let list = usb2snes.ls(&"/".to_string()).await?;
        let expected = USB2SnesFileInfo {
            name: TEST_FILE.to_string(),
            file_type: rusb2snes::USB2SnesFileType::File,
        };
        match list.contains(&expected) {
            true => Ok(()),
            false => Err("Test file not found.".into()),
        }
    }

    #[tokio::test]
    async fn t29_remove_test_file() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        usb2snes.remove_path(TEST_FILE).await?;
        Ok(())
    }

    #[tokio::test]
    async fn t22_boot_test_suite() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        usb2snes.boot(TEST_FILE).await?;
        tokio::time::sleep(time::Duration::from_secs(5)).await;
        Ok(())
    }

    #[tokio::test]
    async fn t23_get_info() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let info = usb2snes.info().await?;
        let expected = Infos {
            version: "1.11.0".to_string(),
            dev_type: "FXPAK PRO STM32".to_string(),
            game: TEST_FILE.to_string(),
            flags: vec![],
        };

        match expected == info {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, info).into()),
        }
    }
    #[tokio::test]
    async fn t24_get_address() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let result = usb2snes.get_address(0xF5000A, 1).await.unwrap();
        let expected: u8 = 85;
        match expected == result[0] {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
    }

    #[tokio::test]
    async fn t25_get_address_as_vec() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let addresses: Vec<u32> = vec![0xF5000A, 0xF5000C];
        let sizes: Vec<usize> = vec![1, 2];
        let result = usb2snes.get_multi_address_as_vec(addresses, sizes).await?;
        dbg!(&result);
        let expected: Vec<u8> = vec![85, 0, 85];
        match expected == result {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
    }

    #[tokio::test]
    async fn t26_get_address_from_pairs() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        let addresses: Vec<u32> = vec![0xF5000A, 0xF5000C];
        let sizes: Vec<usize> = vec![1, 2];
        let result = usb2snes
            .get_multi_address_from_pairs(&[(addresses[0], sizes[0]), (addresses[1], sizes[1])])
            .await?;
        dbg!(&result);
        let expected: Vec<Vec<u8>> = vec![[85].to_vec(), [0, 85].to_vec()];
        match expected == result {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
        // Ok(())
    }

    #[tokio::test]
    async fn t28_reset_game() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        usb2snes.reset().await?;
        tokio::time::sleep(time::Duration::from_secs(5)).await;
        Ok(())
    }

    #[tokio::test]
    async fn t6_boot_menu() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut usb2snes = test_connect().await?;
        usb2snes.menu().await?;
        tokio::time::sleep(time::Duration::from_secs(5)).await;
        Ok(())
    }
}
