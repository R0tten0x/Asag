use std::process::Command;
use std::str;

pub fn is_ethernet_connected() -> bool {
    let output = Command::new("networksetup")
        .arg("-listnetworkserviceorder")
        .output()
        .map_err(|_| "Failed to execute networksetup")
        .unwrap();

    let stdout = str::from_utf8(&output.stdout).unwrap_or("");

    // identify whether ethernet is connnected or not.
    let mut ethernet_device = Vec::new();
    let mut lines = stdout.lines();

    // while loop checks for wired conections while searching for 'Ethernet' or 'LAN'
    while let Some(line) = lines.next() {
        // seek 'Ethernet' or 'LAN'
        if line.contains("Ethernet") || line.contains("LAN") {
            if let Some(next_line) = lines.next() {
                // locate the device (e.g., Device: en0)
                if let Some(idx) = next_line.find("Device: ") {
                    let dev = &next_line[idx + 8..].trim_end_matches(')');
                    ethernet_device.push(dev.to_string());
                }
            }
        }
    }

    // query interface for active status.
    for device in ethernet_device {
        if let Ok(if_output) = Command::new("ifconfig").arg(&device).output() {
            let if_stdout = str::from_utf8(&if_output.stdout).unwrap_or("");
            if if_stdout.contains("status: active") {
                let output = Command::new("networksetup")
                    .args(["-setairportpower", "en0", "off"])
                    .output()
                    .expect("Failed to disable Wi-Fi");
                return true;
            } else if if_stdout.contains("status: inactive") {
                let output = Command::new("networksetup")
                    .args(["-setairportpower", "en0", "on"])
                    .output()
                    .expect("Failed to disable Wi-Fi");
                return false;
            }
        }
    }
    false
}

// osascript notification helper.
pub fn notify(title: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("osascript")
        .args([
            "-e",
            &format!(
                r#"display notification "{}" with title "{}""#,
                message, title
            ),
        ])
        .output()
        .expect("Failed to push notification");

    Ok(())
}
