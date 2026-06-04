use crate::devices::{Device, DeviceEvent, DeviceState};

pub const VENDOR_IDS: [u16; 1] = [0x28de];
pub const PRODUCT_IDS: [u16; 1] = [0x1304];

pub struct SteamController {
    state: DeviceState
}

impl SteamController {
    pub fn new_from_state(state: DeviceState) -> Self {
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
