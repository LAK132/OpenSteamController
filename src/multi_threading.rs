use std::sync::{
    mpsc::{self, Receiver, Sender, TryIter},
    Arc, Mutex,
};

use crate::devices::{DeviceEvent, DeviceProperties};

/// Used to receive commands form the front end
/// and
struct ControllerReceiver {
    command_rx: Receiver<DeviceEvent>,
    state: Arc<Mutex<DeviceProperties>>,
}

impl ControllerReceiver {
    /// receive new commands
    pub fn receive_commands(&mut self) -> TryIter<'_, DeviceEvent> {
        self.command_rx.try_iter()
    }

    /// update the properties for the front end non blocking
    pub fn try_update_state(&mut self, properties: &DeviceProperties) {
        if let Ok(mut state) = self.state.try_lock() {
            state.clone_from(properties);
        }
    }
}

/// Used to send commands to a controller thread
/// and update the state for the front end
struct ControllerSender {
    command_tx: Sender<DeviceEvent>,
    state: Arc<Mutex<DeviceProperties>>,
}

impl ControllerSender {
    /// send commands to the controller thread
    pub fn send_command(&mut self, command: DeviceEvent) {
        self.command_tx.send(command).unwrap();
    }

    /// get the latest properties blocking
    pub fn get_latest_properties(&self) -> DeviceProperties {
        self.state.lock().unwrap().clone()
    }
}

/// create a new ControllerSender and ControllerReceiver pair
pub fn create_controller_channel() -> (ControllerSender, ControllerReceiver) {
    let (command_tx, command_rx) = mpsc::channel();
    let properties = Arc::new(Mutex::new(DeviceProperties::new(0, 0, None)));
    (
        ControllerSender {
            command_tx,
            state: properties.clone(),
        },
        ControllerReceiver {
            command_rx,
            state: properties.clone(),
        },
    )
}
