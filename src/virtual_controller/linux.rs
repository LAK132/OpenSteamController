use crate::virtual_controller::{AbstractVirtualController, ControllerInput};
use uinput::{event::controller, Device, Result};

/// Xbox series x
const PRODUCT_ID: u16 = 0x0b12;
/// Microsoft
const VENDOR_ID: u16 = 0x045e;
/// Name of the controller
const NAME: &str = "Xbox Controller";

/// map generic controller input the the uinput specific events
fn map_controller_input(input: ControllerInput) -> uinput::event::Controller {
    use uinput::event::Controller;
    match input {
        ControllerInput::South => Controller::GamePad(controller::GamePad::South),
        ControllerInput::A => Controller::GamePad(controller::GamePad::A),
        ControllerInput::East => Controller::GamePad(controller::GamePad::East),
        ControllerInput::B => Controller::GamePad(controller::GamePad::B),
        ControllerInput::C => Controller::GamePad(controller::GamePad::C),
        ControllerInput::North => Controller::GamePad(controller::GamePad::North),
        ControllerInput::X => Controller::GamePad(controller::GamePad::X),
        ControllerInput::West => Controller::GamePad(controller::GamePad::West),
        ControllerInput::Y => Controller::GamePad(controller::GamePad::Y),
        ControllerInput::Z => Controller::GamePad(controller::GamePad::Z),
        ControllerInput::LeftTrigger => Controller::GamePad(controller::GamePad::TL),
        ControllerInput::RightTrigger => Controller::GamePad(controller::GamePad::TR),
        ControllerInput::LeftBumper => Controller::GamePad(controller::GamePad::TL2),
        ControllerInput::RightBumper => Controller::GamePad(controller::GamePad::TR2),
        ControllerInput::Select => Controller::GamePad(controller::GamePad::Select),
        ControllerInput::Start => Controller::GamePad(controller::GamePad::Start),
        ControllerInput::Mode => Controller::GamePad(controller::GamePad::Mode),
        ControllerInput::LeftThumb => Controller::GamePad(controller::GamePad::ThumbL),
        ControllerInput::RightThumb => Controller::GamePad(controller::GamePad::ThumbR),
        ControllerInput::Up => Controller::DPad(controller::DPad::Up),
        ControllerInput::Down => Controller::DPad(controller::DPad::Down),
        ControllerInput::Left => Controller::DPad(controller::DPad::Left),
        ControllerInput::Right => Controller::DPad(controller::DPad::Right),
        _ => unimplemented!(),
    }
}

/// A virtual controller to send button inputs
pub struct VirtualController {
    device: Device,
}

impl VirtualController {
    /// create new virtual controller
    pub fn new() -> Result<VirtualController> {
        let device = uinput::default()?
            .name(NAME)?
            .vendor(VENDOR_ID)
            .product(PRODUCT_ID)
            .event(uinput::event::Controller::All)?
            .create()?;
        Ok(Self { device })
    }
}

impl AbstractVirtualController for VirtualController {
    fn press_button(&mut self, input: ControllerInput) -> anyhow::Result<()> {
        self.device.press(&map_controller_input(input))?;
        self.device.synchronize()?;
        Ok(())
    }

    fn release_button(&mut self, input: ControllerInput) -> anyhow::Result<()> {
        self.device.release(&map_controller_input(input))?;
        self.device.synchronize()?;
        Ok(())
    }

    fn joystick(&mut self, input: ControllerInput) -> anyhow::Result<()> {
        todo!()
    }
}
