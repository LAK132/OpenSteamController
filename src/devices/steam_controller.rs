use crate::devices::{Device, DeviceEvent, DeviceState};

pub const VENDOR_IDS: [u16; 1] = [0x28de];
pub const PRODUCT_IDS: [u16; 2] = [
    0x1302, // direct connection to controller
    0x1304, // connection via puck
];

/// Flag to start a feature report
const FEATURE_REPORT: u8 = 0x01;
/// Command to set a setting
const SET_SETTING_CMD: u8 = 0x87;

pub struct SteamController {
    state: DeviceState,
}

impl SteamController {
    /// Returns the complete packet to disable "lizard mode"
    /// Has to be sent frequently to keep it disabled
    pub fn get_disable_lizard_mode_packet() -> Vec<u8> {
        Self::decorate_packet(SET_SETTING_CMD, vec![0x09, 0x00, 0x00])
    }

    /// Initially disables the "lizard mode" and constructs the controller
    pub fn new_from_state(state: DeviceState) -> Self {
        state.write_hid_report(&SteamController::get_disable_lizard_mode_packet());
        Self { state }
    }

    /// Builds a packet based on a given command and a payload
    /// Basic structure: feature report flag, command, size of payload, payload
    fn decorate_packet(command: u8, mut payload: Vec<u8>) -> Vec<u8> {
        let mut packet = vec![FEATURE_REPORT, command, payload.len() as u8];
        packet.append(&mut payload);
        packet.append(&mut vec![0x00; 64 - packet.len()]);
        packet
    }
}

impl Device for SteamController {
    fn get_charging_packet(&self) -> Option<Vec<u8>> {
        unimplemented!();
    }

    fn get_battery_packet(&self) -> Option<Vec<u8>> {
        unimplemented!();
    }

    fn get_wireless_connected_status_packet(&self) -> Option<Vec<u8>> {
        unimplemented!();
    }

    fn get_event_from_device_response(&self, _response: &[u8]) -> Option<Vec<DeviceEvent>> {
        unimplemented!();
    }

    fn get_device_state(&self) -> &DeviceState {
        &self.state
    }

    fn get_device_state_mut(&mut self) -> &mut DeviceState {
        &mut self.state
    }

    fn allow_passive_refresh(&mut self) -> bool {
        true
    }
}
