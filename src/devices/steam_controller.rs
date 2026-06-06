use std::ops::Range;

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
const RESPONSE_INPUT_EVENT: u8 = 0x45;
/// Range in the response containing the button bits
const BUTTON_LOCATION: Range<usize> = 2..6;
/// Range in the response containing the triggers
const TRIGGER_LOCATION: Range<usize> = 6..10;
/// Range in the response containing the left yoystick
const JOYSTICK_LOCATION: Range<usize> = 10..18;
/// Range in the response containing the trackpads (x y pressure)
const TRACKPAD_LOCATION: Range<usize> = 18..30;
/// Max value of an analog input
const ANALOG_MAX: f32 = 0b01111111_11111111 as f32;

pub struct SteamController {
    state: DeviceState,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Button: u32 {
        const A                 = 1 <<  0;
        const B                 = 1 <<  1;
        const X                 = 1 <<  2;
        const Y                 = 1 <<  3;
        const Menu              = 1 <<  4;
        const ThumbRight        = 1 <<  5;
        const Select            = 1 <<  6;
        const R4                = 1 <<  7;
        const R5                = 1 <<  8;
        const R1                = 1 <<  9;
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

    fn wrap_controller_input_into_device_event(input: Vec<ControllerInput>) -> Vec<DeviceEvent> {
        input
            .iter()
            .map(|event| DeviceEvent::ButtonPressed(*event))
            .collect()
    }

    /// Converts the slice containing the bits of currently pressed buttons into corresponding
    /// events of press or release button and updates the previous bitmap for comparison
    fn handle_buttons(&self, response: [u8; 4]) -> Vec<DeviceEvent> {
        let button_bitmap = Button::from_bits_truncate(u32::from_le_bytes(response));
        let previous_bitmap = Button::from_bits_truncate(
            self.get_device_state()
                .device_properties
                .previous_button_bitmap as u32,
        );
        let changed_buttons = button_bitmap.symmetric_difference(previous_bitmap);
        if !changed_buttons.is_empty() {
            let mut result = Self::wrap_controller_input_into_device_event(Self::get_buttons(
                &changed_buttons,
                &button_bitmap,
            ));
            result.push(DeviceEvent::UpdateBitmap(button_bitmap.bits() as u64));
            result
        } else {
            vec![]
        }
    }

    /// Converts a bitmap of which buttons changed state and a bitmap of the current button states
    /// into a Vec of ControllerInput filled with digital ControllerInput, either pressed or
    /// released
    fn get_buttons(changes: &Button, states: &Button) -> Vec<ControllerInput> {
        println!("Changes: {:0>32b}", changes.bits());
        changes
            .iter()
            .map(|button| {
                bitflags_match!(button, {
                    Button::A =>                ControllerInput::A(states.contains(Button::A)),
                    Button::B =>                ControllerInput::B(states.contains(Button::B)),
                    Button::X =>                ControllerInput::X(states.contains(Button::X)),
                    Button::Y =>                ControllerInput::Y(states.contains(Button::Y)),
                    Button::Menu =>             ControllerInput::Menu(states.contains(Button::Menu)),
                    Button::ThumbRight =>       ControllerInput::RightThumb(states.contains(Button::ThumbRight)),
                    Button::Select =>           ControllerInput::Select(states.contains(Button::Select)),
                    Button::R4 =>               ControllerInput::RightBumper(states.contains(Button::R4)), // TODO
                    Button::R5 =>               ControllerInput::RightTrigger(1.0), // TODO
                    Button::R1 =>               ControllerInput::RightBumper(states.contains(Button::R1)),
                    Button::DpadDown =>         ControllerInput::Down(states.contains(Button::DpadDown)),
                    Button::DpadRight =>        ControllerInput::Right(states.contains(Button::DpadRight)),
                    Button::DpadLeft =>         ControllerInput::Left(states.contains(Button::DpadLeft)),
                    Button::DpadUp =>           ControllerInput::Up(states.contains(Button::DpadUp)),
                    Button::Start =>            ControllerInput::Start(states.contains(Button::Start)),
                    Button::ThumbLeft =>        ControllerInput::LeftThumb(states.contains(Button::ThumbLeft)),
                    Button::Steam =>            ControllerInput::Home(states.contains(Button::Steam)),
                    Button::L4 =>               ControllerInput::LeftBumper(states.contains(Button::L4)), // TODO
                    Button::L5 =>               ControllerInput::LeftTrigger(1.0), // TODO
                    Button::L1 =>               ControllerInput::LeftBumper(states.contains(Button::L1)), // TODO
                    Button::ThumbRightTouch =>  ControllerInput::RightThumb(states.contains(Button::ThumbRightTouch)), // TODO
                    Button::PadRightTouch =>    ControllerInput::RightThumb(states.contains(Button::PadRightTouch)), // TODO
                    Button::PadRightClick =>    ControllerInput::RightThumb(states.contains(Button::PadRightClick)), // TODO
                    Button::R2 =>               ControllerInput::RightTrigger(1.0),
                    Button::ThumbLeftTouch =>   ControllerInput::LeftThumb(states.contains(Button::ThumbLeftTouch)), //TODO
                    Button::PadLeftTouch =>     ControllerInput::LeftThumb(states.contains(Button::PadLeftTouch)),   // TODO
                    Button::PadLeftClick =>     ControllerInput::LeftThumb(states.contains(Button::PadLeftClick)),   // TODO
                    Button::L2 =>               ControllerInput::LeftTrigger(1.0),
                    Button::GripRight =>        ControllerInput::RightThumb(states.contains(Button::GripRight)), // TODO
                    Button::GripLeft =>         ControllerInput::LeftThumb(states.contains(Button::GripLeft)),   // TODO
                    _ => panic!("Undefined Button!"),
                })
            })
            .collect::<Vec<ControllerInput>>()
    }

    /// Converts two bytes of data into a value between -1.0 and 1.0
    fn convert_analog(input: [u8; 2]) -> f32 {
        let bits = i16::from_le_bytes(input) as f32;
        bits / ANALOG_MAX
    }

    /// Converts four bytes of data into two axes between -1.0 and 1.0
    fn convert_analog_2d(input: [u8; 4]) -> (f32, f32) {
        let x = Self::convert_analog(input[0..2].try_into().unwrap());
        let y = -Self::convert_analog(input[2..4].try_into().unwrap());
        (x, y)
    }

    /// Converts six bytes of data into two three between -1.0 and 1.0
    fn convert_analog_3d(input: [u8; 6]) -> (f32, f32, f32) {
        let x = Self::convert_analog(input[0..2].try_into().unwrap());
        let y = -Self::convert_analog(input[2..4].try_into().unwrap());
        let z = Self::convert_analog(input[4..6].try_into().unwrap());
        (x, y, z)
    }

    /// Converts four bytes of data into controller input events for the left and right trigger
    fn handle_triggers(response: [u8; 4]) -> Vec<DeviceEvent> {
        let left = Self::convert_analog(response[0..2].try_into().unwrap());
        let right = Self::convert_analog(response[2..4].try_into().unwrap());
        Self::wrap_controller_input_into_device_event(vec![
            ControllerInput::LeftTrigger(left),
            ControllerInput::RightTrigger(right),
        ])
    }

    /// Converts four bytes of data into controller input events for the left and right joystick
    fn handle_joysticks(response: [u8; 8]) -> Vec<DeviceEvent> {
        let (left_x, left_y) = Self::convert_analog_2d(response[0..4].try_into().unwrap());
        let (right_x, right_y) = Self::convert_analog_2d(response[4..8].try_into().unwrap());
        Self::wrap_controller_input_into_device_event(vec![
            ControllerInput::LeftJoyStick(left_x, left_y),
            ControllerInput::RightJoyStick(right_x, right_y),
        ])
    }

    /// Converts four bytes of data into controller input events for the left and right trackpad
    fn handle_trackpads(response: [u8; 12]) -> Vec<DeviceEvent> {
        let (left_x, left_y, left_force) =
            Self::convert_analog_3d(response[0..6].try_into().unwrap());
        let (right_x, right_y, right_force) =
            Self::convert_analog_3d(response[6..12].try_into().unwrap());
        Self::wrap_controller_input_into_device_event(vec![
            ControllerInput::LeftTrackpad(left_x, left_y, left_force),
            ControllerInput::RightTrackpad(right_x, right_y, right_force),
        ])
    }
}

impl Device for SteamController {
    fn get_charging_packet(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_battery_packet(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_wireless_connected_status_packet(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_event_from_device_response(&self, response: &[u8]) -> Option<Vec<DeviceEvent>> {
        let mut events = vec![];
        if response[0] == RESPONSE_INPUT_EVENT {
            events.append(&mut self.handle_buttons(response[BUTTON_LOCATION].try_into().unwrap()));
            events.append(&mut SteamController::handle_triggers(
                response[TRIGGER_LOCATION].try_into().unwrap(),
            ));
            events.append(&mut SteamController::handle_joysticks(
                response[JOYSTICK_LOCATION].try_into().unwrap(),
            ));
            events.append(&mut SteamController::handle_trackpads(
                response[TRACKPAD_LOCATION].try_into().unwrap(),
            ));
        }

        if events.is_empty() {
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
