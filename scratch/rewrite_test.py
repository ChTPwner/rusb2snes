import re

with open('tests/integration_test.rs', 'r') as f:
    content = f.read()

content = content.replace('SyncClient', 'AsyncClient')
content = content.replace('#[test]', '#[tokio::test]')
content = content.replace('std::thread::sleep(time::Duration::from_secs(', 'tokio::time::sleep(time::Duration::from_secs(')
content = content.replace(').unwrap();', ').await.unwrap();')
content = content.replace(')?', ').await?')
content = content.replace('.await.await', '.await') # Fix if accidentally duplicated

content = content.replace('fn test_connect()', 'async fn test_connect()')
content = content.replace('fn t01_menu_info()', 'async fn t01_menu_info()')
content = content.replace('fn t20_send_test_file()', 'async fn t20_send_test_file()')
content = content.replace('fn t21_check_test_file()', 'async fn t21_check_test_file()')
content = content.replace('fn t29_remove_test_file()', 'async fn t29_remove_test_file()')
content = content.replace('fn t22_boot_test_suite()', 'async fn t22_boot_test_suite()')
content = content.replace('fn t23_get_info()', 'async fn t23_get_info()')
content = content.replace('fn t24_get_address()', 'async fn t24_get_address()')
content = content.replace('fn t25_get_address_as_vec()', 'async fn t25_get_address_as_vec()')
content = content.replace('fn t26_get_address_from_pairs()', 'async fn t26_get_address_from_pairs()')
content = content.replace('fn t28_reset_game()', 'async fn t28_reset_game()')
content = content.replace('fn t6_boot_menu()', 'async fn t6_boot_menu()')

# Missing awaits for sleeps
content = content.replace('tokio::time::sleep(time::Duration::from_secs(2));', 'tokio::time::sleep(time::Duration::from_secs(2)).await;')
content = content.replace('tokio::time::sleep(time::Duration::from_secs(5));', 'tokio::time::sleep(time::Duration::from_secs(5)).await;')

content = content.replace('let mut usb2snes = test_connect().await?', 'let mut usb2snes = test_connect().await?') # Wait, the replace earlier already did test_connect().await?
# test_connect()? became test_connect().await?

with open('tests/integration_test.rs', 'w') as f:
    f.write(content)
