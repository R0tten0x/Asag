use crate::network::{is_ethernet_connected, notify};
mod network;

fn main() {
    let mut previous_state = is_ethernet_connected();

    loop {
        let current_state = is_ethernet_connected();

        if current_state != previous_state {
            if current_state {
                notify("Ethernet connection detected.", "\nDisabling Wi-Fi.").unwrap();
            } else {
                notify("Ethernet connection disconnected", "\nEnabling WiFi").unwrap();
            }

            previous_state = current_state;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
