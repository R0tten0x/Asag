use crate::network::{is_ethernet_connected, notify};
mod network;

fn main() {
    if is_ethernet_connected() {
        notify("Ethernet connection detected.", "\nDisabling Wi-Fi.").unwrap();
    } else {
        notify("Ethernet connection disconnected", "\nEnabling WiFi").unwrap();
    }
}
