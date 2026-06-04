use crate::devices::{Device, DeviceEvent, DeviceState};

pub const VENDOR_IDS: [u16; 1] = [0x28de];
pub const PRODUCT_IDS: [u16; 2] = [
    0x1302, // direct connection to controller
    0x1304, // connection via puck
];

pub struct SteamController {
    state: DeviceState
}

impl SteamController {
    pub fn new_from_state(state: DeviceState) -> Self {
        state.write_hid_report(&SteamController::get_disable_lizard_mode_packet());
        Self { state }
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
