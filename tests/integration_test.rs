use core::time;
use rusb2snes::{Infos, SyncClient, USB2SnesEndpoint, USB2SnesFileInfo};
use std::error::Error;
use std::fs;

const CLIENT_NAME: &str = "TESTS";
const TEST_FILE: &str = "240pSuite_test.sfc";

#[cfg(test)]
mod tests {

    use super::*;
    fn test_connect() -> Result<SyncClient, Box<dyn Error>> {
        let endpoint = USB2SnesEndpoint::default();
        let mut usb2snes = SyncClient::connect(&endpoint)?;
        usb2snes.set_name(CLIENT_NAME.to_string())?;
        let devices = usb2snes.list_device()?;
        usb2snes.attach(&devices[0])?;
        Ok(usb2snes)
    }
    #[test]
    fn t01_menu_info() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let info = usb2snes.info()?;
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

    #[test]
    fn t20_send_test_file() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let local_path = format!("files/{TEST_FILE}");
        let data = fs::read(&local_path).expect("Error opening the file or reading the content");
        usb2snes.send_file(&TEST_FILE.to_string(), data)?;
        std::thread::sleep(time::Duration::from_secs(2));
        Ok(())
    }

    #[test]
    fn t21_check_test_file() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let list = usb2snes.ls(&"/".to_string())?;
        let expected = USB2SnesFileInfo {
            name: TEST_FILE.to_string(),
            file_type: rusb2snes::USB2SnesFileType::File,
        };
        match list.contains(&expected) {
            true => Ok(()),
            false => Err("Test file not found.".into()),
        }
    }

    #[test]
    fn t29_remove_test_file() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        usb2snes.remove_path(TEST_FILE)?;
        Ok(())
    }

    #[test]
    fn t22_boot_test_suite() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        usb2snes.boot(TEST_FILE)?;
        std::thread::sleep(time::Duration::from_secs(5));
        Ok(())
    }

    #[test]
    fn t23_get_info() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let info = usb2snes.info()?;
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
    #[test]
    fn t24_get_address() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let result = usb2snes.get_address(0xF5000A, 1).unwrap();
        let expected: u8 = 85;
        match expected == result[0] {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
    }

    #[test]
    fn t25_get_address_as_vec() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let addresses: Vec<u32> = vec![0xF5000A, 0xF5000C];
        let sizes: Vec<usize> = vec![1, 2];
        let result = usb2snes.get_multi_address_as_vec(addresses, sizes)?;
        dbg!(&result);
        let expected: Vec<u8> = vec![85, 0, 85];
        match expected == result {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
    }

    #[test]
    fn t26_get_address_from_pairs() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        let addresses: Vec<u32> = vec![0xF5000A, 0xF5000C];
        let sizes: Vec<usize> = vec![1, 2];
        let result = usb2snes
            .get_multi_address_from_pairs(&[(addresses[0], sizes[0]), (addresses[1], sizes[1])])?;
        dbg!(&result);
        let expected: Vec<Vec<u8>> = vec![[85].to_vec(), [0, 85].to_vec()];
        match expected == result {
            true => Ok(()),
            false => Err(format!("Expected {:?}, got {:?}.", expected, result).into()),
        }
        // Ok(())
    }

    #[test]
    fn t28_reset_game() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        usb2snes.reset()?;
        std::thread::sleep(time::Duration::from_secs(5));
        Ok(())
    }

    #[test]
    fn t6_boot_menu() -> Result<(), Box<dyn Error>> {
        let mut usb2snes = test_connect()?;
        usb2snes.menu()?;
        std::thread::sleep(time::Duration::from_secs(5));
        Ok(())
    }
}
