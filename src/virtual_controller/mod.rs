#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub use linux::VirtualController;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Backed independent controller inputs
pub enum ControllerInput {
    South,
    A,
    East,
    B,
    C,
    North,
    X,
    West,
    Y,
    Z,
    TL,
    TR,
    TL2,
    TR2,
    Select,
    Start,
    Mode,
    ThumbL,
    ThumbR,
    Up,
    Down,
    Left,
    Right,
}

/// Used to implement OS independent virtual controller
pub trait AbstractVirtualController {
    /// simulate a button press
    fn press_button(&mut self, input: ControllerInput) -> anyhow::Result<()>;
    /// simulate a button release
    fn release_button(&mut self, input: ControllerInput) -> anyhow::Result<()>;
    /// simulate joy stick movement
    fn joystick(&mut self, input: ControllerInput) -> anyhow::Result<()>;
}
