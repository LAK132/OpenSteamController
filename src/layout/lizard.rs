use crate::devices::{
    DeviceButton, DeviceCommand, DeviceOutput, DeviceProperties, HapticEvent, HapticSide,
};
use crate::layout::{AnalogHatX, AnalogHatY, ControllerLayout, LayoutState, MappedOutput};
use crate::virtual_controller::ControllerInput;

pub struct LizardLayout {}

impl ControllerLayout for LizardLayout {
    fn map_device_output(
        state: &mut LayoutState,
        input: DeviceOutput,
        device_properties: &DeviceProperties,
    ) -> Vec<MappedOutput> {
        match input {
            DeviceOutput::A(p) => vec![MappedOutput::Controller(ControllerInput::EnterKey(p))],
            DeviceOutput::B(p) => vec![MappedOutput::Controller(ControllerInput::EscKey(p))],
            DeviceOutput::X(p) => vec![MappedOutput::Controller(ControllerInput::BackspaceKey(p))],
            DeviceOutput::Y(_) => vec![],

            DeviceOutput::Up(p) => vec![MappedOutput::Controller(ControllerInput::UpKey(p))],
            DeviceOutput::Down(p) => vec![MappedOutput::Controller(ControllerInput::DownKey(p))],
            DeviceOutput::Left(p) => vec![MappedOutput::Controller(ControllerInput::LeftKey(p))],
            DeviceOutput::Right(p) => vec![MappedOutput::Controller(ControllerInput::RightKey(p))],

            DeviceOutput::Menu(p) => vec![MappedOutput::Controller(ControllerInput::MetaKey(p))],
            DeviceOutput::View(p) => vec![MappedOutput::Controller(ControllerInput::TabKey(p))],
            DeviceOutput::Steam(_) => vec![],
            DeviceOutput::QuickAccessMenu(p) => {
                if !p {
                    vec![MappedOutput::Device(DeviceCommand::SwitchLayout(
                        device_properties.controller_layout,
                    ))]
                } else {
                    vec![]
                }
            }

            DeviceOutput::LeftJoyStick(x, y) => {
                if state.buttons_prev.contains(DeviceButton::Steam) {
                    state.left_joy_hat.way4(
                        (x, y),
                        0.05,
                        |x, p| match x {
                            AnalogHatX::Left => {
                                MappedOutput::Controller(ControllerInput::MediaPrev(p))
                            }
                            AnalogHatX::Right => {
                                MappedOutput::Controller(ControllerInput::MediaNext(p))
                            }
                        },
                        |y, p| match y {
                            AnalogHatY::Up => {
                                MappedOutput::Controller(ControllerInput::VolumeUp(p))
                            }
                            AnalogHatY::Down => {
                                MappedOutput::Controller(ControllerInput::VolumeDown(p))
                            }
                        },
                    )
                } else {
                    state.wheel.joystick((x, y)).map_or_else(
                        || vec![],
                        |(x, y)| vec![MappedOutput::Controller(ControllerInput::SmoothWheel(x, y))],
                    )
                }
            }
            DeviceOutput::LeftJoyStickTouch(_) => vec![],

            DeviceOutput::RightJoyStick(x, y) => state.mouse.joystick((x, y)).map_or_else(
                || vec![],
                |(x, y)| vec![MappedOutput::Controller(ControllerInput::Mouse(x, y))],
            ),
            DeviceOutput::RightJoyStickTouch(_) => vec![],

            DeviceOutput::L1(p) => vec![MappedOutput::Controller(ControllerInput::CtrlKey(p))],
            DeviceOutput::L2(p) => vec![MappedOutput::Controller(ControllerInput::RightMouse(p))],
            DeviceOutput::L3(p) => vec![MappedOutput::Controller(ControllerInput::PlayPause(p))],
            DeviceOutput::L4(_) => vec![],
            DeviceOutput::L5(p) => vec![MappedOutput::Controller(ControllerInput::Back(p))],

            DeviceOutput::LeftTrigger(_) => vec![],
            DeviceOutput::LeftGripTouch(_) => vec![],

            DeviceOutput::R1(p) => vec![MappedOutput::Controller(ControllerInput::AltKey(p))],
            DeviceOutput::R2(p) => vec![MappedOutput::Controller(ControllerInput::LeftMouse(p))],
            DeviceOutput::R3(_) => vec![],
            DeviceOutput::R4(p) => vec![MappedOutput::Controller(ControllerInput::MiddleMouse(p))],
            DeviceOutput::R5(p) => vec![MappedOutput::Controller(ControllerInput::Forward(p))],

            DeviceOutput::RightTrigger(_) => vec![],
            DeviceOutput::RightGripTouch(_) => vec![],

            DeviceOutput::LeftTrackpad(x, y, z) => {
                state.wheel.notched_trackpad((x, y, z)).map_or_else(
                    || vec![],
                    |(x, y, h)| {
                        let mut result =
                            vec![MappedOutput::Controller(ControllerInput::Wheel(x, y))];
                        if h {
                            result.push(MappedOutput::Device(DeviceCommand::Haptic(HapticEvent {
                                pad: HapticSide::Left,
                                duration: 0x000A,
                                interval: 0x0000,
                                count: 0x0001,
                                gain: 0xff,
                            })));
                        }
                        result
                    },
                )
            }
            DeviceOutput::LeftTrackpadClick(_) => vec![],
            DeviceOutput::LeftTrackpadTouch(_) => {
                state.wheel.reset_trackpad();
                vec![]
            }

            DeviceOutput::RightTrackpad(x, y, z) => state.mouse.trackpad((x, y, z)).map_or_else(
                || vec![],
                |(x, y, h)| {
                    let mut result = vec![MappedOutput::Controller(ControllerInput::Mouse(x, y))];
                    if h {
                        result.push(MappedOutput::Device(DeviceCommand::Haptic(HapticEvent {
                            pad: HapticSide::Right,
                            duration: 0x000A,
                            interval: 0x0000,
                            count: 0x0001,
                            gain: 0xff,
                        })));
                    }
                    result
                },
            ),
            DeviceOutput::RightTrackpadClick(p) => {
                state.mouse.reset_trackpad();
                vec![
                    MappedOutput::Controller(ControllerInput::LeftMouse(p)),
                    MappedOutput::Device(DeviceCommand::Haptic(HapticEvent {
                        pad: HapticSide::Right,
                        duration: 0x000A,
                        interval: 0x0000,
                        count: 0x0001,
                        gain: 0xff,
                    })),
                ]
            }
            DeviceOutput::RightTrackpadTouch(_) => {
                state.mouse.reset_trackpad();
                vec![]
            }
        }
    }
}
