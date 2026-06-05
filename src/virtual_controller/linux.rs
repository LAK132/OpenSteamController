use std::i32;
use std::time::Duration;

use crate::virtual_controller::{AbstractVirtualController, ControllerInput};
use uinput::event::absolute::Position;
use uinput::event::Controller;
use uinput::{event::controller, Device, Result};

/// Xbox series x
const PRODUCT_ID: u16 = 0x0b12;
/// Microsoft
const VENDOR_ID: u16 = 0x045e;
/// Name of the controller
const NAME: &str = "Xbox Controller";

/// map generic controller input the the uinput specific events
fn map_digital_controller_input(input: ControllerInput) -> (uinput::event::Controller, bool) {
    match input {
        ControllerInput::South(pressed) => {
            (Controller::GamePad(controller::GamePad::South), pressed)
        }
        ControllerInput::A(pressed) => (Controller::GamePad(controller::GamePad::A), pressed),
        ControllerInput::East(pressed) => (Controller::GamePad(controller::GamePad::East), pressed),
        ControllerInput::B(pressed) => (Controller::GamePad(controller::GamePad::B), pressed),
        ControllerInput::C(pressed) => (Controller::GamePad(controller::GamePad::C), pressed),
        ControllerInput::North(pressed) => {
            (Controller::GamePad(controller::GamePad::North), pressed)
        }
        ControllerInput::X(pressed) => (Controller::GamePad(controller::GamePad::X), pressed),
        ControllerInput::West(pressed) => (Controller::GamePad(controller::GamePad::West), pressed),
        ControllerInput::Y(pressed) => (Controller::GamePad(controller::GamePad::Y), pressed),
        ControllerInput::Z(pressed) => (Controller::GamePad(controller::GamePad::Z), pressed),
        ControllerInput::LeftBumper(pressed) => {
            (Controller::GamePad(controller::GamePad::TL2), pressed)
        }
        ControllerInput::RightBumper(pressed) => {
            (Controller::GamePad(controller::GamePad::TR2), pressed)
        }
        ControllerInput::Select(pressed) => {
            (Controller::GamePad(controller::GamePad::Select), pressed)
        }
        ControllerInput::Start(pressed) => {
            (Controller::GamePad(controller::GamePad::Start), pressed)
        }
        ControllerInput::Mode(pressed) => (Controller::GamePad(controller::GamePad::Mode), pressed),
        ControllerInput::LeftThumb(pressed) => {
            (Controller::GamePad(controller::GamePad::ThumbL), pressed)
        }
        ControllerInput::RightThumb(pressed) => {
            (Controller::GamePad(controller::GamePad::ThumbR), pressed)
        }
        ControllerInput::Up(pressed) => (Controller::DPad(controller::DPad::Up), pressed),
        ControllerInput::Down(pressed) => (Controller::DPad(controller::DPad::Down), pressed),
        ControllerInput::Left(pressed) => (Controller::DPad(controller::DPad::Left), pressed),
        ControllerInput::Right(pressed) => (Controller::DPad(controller::DPad::Right), pressed),
        _ => unimplemented!(),
    }
}

/// A virtual controller to send button inputs
pub struct VirtualController {
    device: Device,
}

const STICK_MIN: i32 = -32768;
const STICK_MAX: i32 = 32767;

impl VirtualController {
    /// create new virtual controller
    pub fn new() -> Result<VirtualController> {
        let device = uinput::default()?
            .name(NAME)?
            .vendor(VENDOR_ID)
            .product(PRODUCT_ID)
            .event(uinput::event::Controller::GamePad(controller::GamePad::A))?
            .event(uinput::event::Controller::GamePad(controller::GamePad::B))?
            .event(uinput::event::Controller::GamePad(controller::GamePad::X))?
            .event(uinput::event::Controller::GamePad(controller::GamePad::Y))?
            .event(uinput::event::Controller::GamePad(controller::GamePad::TL))?
            .event(uinput::event::Controller::GamePad(controller::GamePad::TR))?
            .event(uinput::event::Controller::GamePad(
                controller::GamePad::Select,
            ))?
            .event(uinput::event::Controller::GamePad(
                controller::GamePad::Start,
            ))?
            .event(uinput::event::Controller::GamePad(
                controller::GamePad::ThumbL,
            ))?
            .event(uinput::event::Controller::GamePad(
                controller::GamePad::ThumbR,
            ))?
            // Left stick
            .event(uinput::event::absolute::Absolute::Position(Position::X))?
            .min(STICK_MIN)
            .max(STICK_MAX)
            .fuzz(0)
            .flat(0)
            .event(uinput::event::absolute::Absolute::Position(Position::Y))?
            .min(STICK_MIN)
            .max(STICK_MAX)
            .fuzz(0)
            .flat(0)
            // Right stick
            .event(uinput::event::absolute::Absolute::Position(Position::RX))?
            .min(STICK_MIN)
            .max(STICK_MAX)
            .fuzz(0)
            .flat(0)
            .event(uinput::event::absolute::Absolute::Position(Position::RY))?
            .min(STICK_MIN)
            .max(STICK_MAX)
            .fuzz(0)
            .flat(0)
            // Triggers
            .event(uinput::event::absolute::Absolute::Position(Position::Z))?
            .min(0)
            .max(255)
            .fuzz(0)
            .flat(0)
            .event(uinput::event::absolute::Absolute::Position(Position::RZ))?
            .min(0)
            .max(255)
            .fuzz(0)
            .flat(0)
            .create()?;
        Ok(Self { device })
    }

    /// helper for digital inputs
    fn perform_digital_input(&mut self, input: ControllerInput) -> anyhow::Result<()> {
        let (input, pressed) = map_digital_controller_input(input);
        if pressed {
            self.device.press(&input)?;
        } else {
            self.device.release(&input)?;
        }
        Ok(())
    }
}

impl AbstractVirtualController for VirtualController {
    fn send_input(&mut self, input: ControllerInput) -> anyhow::Result<()> {
        match input {
            ControllerInput::RightJoyStick(x, y) => {
                todo!();
                self.device.position(
                    &uinput::event::absolute::Position::X,
                    ((STICK_MAX - STICK_MIN) as f32 * x) as i32,
                )?;
                self.device.position(
                    &uinput::event::absolute::Position::Y,
                    ((STICK_MAX - STICK_MIN) as f32 * y) as i32,
                )?;
                self.device.synchronize()?;
            }
            ControllerInput::LeftJoyStick(x, y) => todo!(),
            ControllerInput::LeftTrigger(force) => todo!(),
            ControllerInput::RightTrigger(force) => todo!(),
            digital_input => self.perform_digital_input(digital_input)?,
        }
        self.device.synchronize()?;
        Ok(())
    }
}
