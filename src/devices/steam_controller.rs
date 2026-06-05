use crate::{
    devices::{Device, DeviceEvent, DeviceState},
    virtual_controller::ControllerInput,
};
use bitflags::{bitflags, bitflags_match};

pub const VENDOR_IDS: [u16; 1] = [0x28de];
pub const PRODUCT_IDS: [u16; 2] = [
    0x1302, // direct connection to controller
    0x1304, // connection via puck
];

/// Flag to start a feature report
const FEATURE_REPORT: u8 = 0x01;
/// Command to set a setting
const SET_SETTING_CMD: u8 = 0x87;
/// Response prefix for button event
const RESPONSE_BUTTON_EVENT: u8 = 0x45;

pub struct SteamController {
    state: DeviceState,
}

bitflags! {
    #[derive(Debug, PartialEq)]
    struct Button: u32 {
        const A                 = 1 << 00;
        const B                 = 1 << 01;
        const X                 = 1 << 02;
        const Y                 = 1 << 03;
        const Menu              = 1 << 04;
        const ThumbRight        = 1 << 05;
        const Select            = 1 << 06;
        const R4                = 1 << 07;
        const R5                = 1 << 08;
        const R1                = 1 << 09;
        const DpadDown          = 1 << 10;
        const DpadRight         = 1 << 11;
        const DpadLeft          = 1 << 12;
        const DpadUp            = 1 << 13;
        const Start             = 1 << 14;
        const ThumbLeft         = 1 << 15;
        const Steam             = 1 << 16;
        const L4                = 1 << 17;
        const L5                = 1 << 18;
        const L1                = 1 << 19;
        const ThumbRightTouch   = 1 << 20;
        const PadRightTouch     = 1 << 21;
        const PadRightClick     = 1 << 22;
        const R2                = 1 << 23;
        const ThumbLeftTouch    = 1 << 24;
        const PadLeftTouch      = 1 << 25;
        const PadLeftClick      = 1 << 26;
        const L2                = 1 << 27;
        const GripRight         = 1 << 28;
        const GripLeft          = 1 << 29;
    }
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

    fn get_buttons(buttons_bits: u32) -> Vec<ControllerInput> {
        Button::from_bits_truncate(buttons_bits)
            .iter()
            .map(|button| {
                bitflags_match!(button, {
                    Button::A => ControllerInput::A,
                    Button::B => ControllerInput::B,
                    Button::X => ControllerInput::X,
                    Button::Y => ControllerInput::Y,
                    Button::Menu => ControllerInput::Menu,
                    Button::ThumbRight => ControllerInput::RightThumb,
                    Button::Select => ControllerInput::Select,
                    Button::R4 => ControllerInput::RightBumper, // TODO
                    Button::R5 => ControllerInput::RightTrigger, // TODO
                    Button::R1 => ControllerInput::RightTrigger,
                    Button::DpadDown => ControllerInput::Down,
                    Button::DpadRight => ControllerInput::Right,
                    Button::DpadLeft => ControllerInput::Left,
                    Button::DpadUp => ControllerInput::Up,
                    Button::Start => ControllerInput::Start,
                    Button::ThumbLeft => ControllerInput::LeftThumb,
                    Button::Steam => ControllerInput::Home,
                    Button::L4 => ControllerInput::LeftBumper, // TODO
                    Button::L5 => ControllerInput::LeftTrigger, // TODO
                    Button::L1 => ControllerInput::LeftBumper, // TODO
                    Button::ThumbRightTouch => ControllerInput::RightThumb, // TODO
                    Button::PadRightTouch => ControllerInput::RightThumb, // TODO
                    Button::PadRightClick => ControllerInput::RightThumb, // TODO
                    Button::R2 => ControllerInput::RightTrigger,
                    Button::ThumbLeftTouch => ControllerInput::LeftThumb, //TODO
                    Button::PadLeftTouch => ControllerInput::LeftThumb,   // TODO
                    Button::PadLeftClick => ControllerInput::LeftThumb,   // TODO
                    Button::L2 => ControllerInput::LeftTrigger,
                    Button::GripRight => ControllerInput::RightThumb, // TODO
                    Button::GripLeft => ControllerInput::LeftThumb,   // TODO
                    _ => panic!("Undefined Button!"),
                })
            })
            .collect::<Vec<ControllerInput>>()
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

    fn get_event_from_device_response(&self, response: &[u8]) -> Option<Vec<DeviceEvent>> {
        let mut events = vec![];
        if response[0] == RESPONSE_BUTTON_EVENT {
            let button_bits = u32::from_le_bytes(response[2..6].try_into().unwrap());
            Self::get_buttons(button_bits)
                .iter()
                .map(|controller_input| DeviceEvent::ButtonPressed(*controller_input))
                .for_each(|event| events.push(event));
        }

        if events.len() == 0 {
            None
        } else {
            Some(events)
        }
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
