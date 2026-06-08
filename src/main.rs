use open_steam_controller::debug_println;
use open_steam_controller::multi_threading;

#[cfg(target_os = "linux")]
mod status_tray;

#[cfg(not(target_os = "linux"))]
mod status_tray_not_linux;

#[cfg(target_os = "windows")]
mod tray_battery_icon_state;

#[cfg(not(target_os = "linux"))]
fn main() {
    use clap::ArgAction;
    use open_steam_controller::multi_threading::ControllerSender;
    use std::sync::mpsc;

    use crate::status_tray_not_linux::TrayApp;
    use open_steam_controller::devices::{DeviceEvent, DeviceProperties};
    use open_steam_controller::virtual_controller::AbstractVirtualController;
    use open_steam_controller::VERBOSE;
    use open_steam_controller::{
        devices::connect_compatible_devices, virtual_controller::VirtualController,
    };
    use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

    let event_loop: EventLoop<Vec<DeviceProperties>> =
        EventLoop::with_user_event().build().unwrap();
    let proxy: EventLoopProxy<Vec<DeviceProperties>> = event_loop.create_proxy();
    event_loop.set_control_flow(ControlFlow::Wait);

    use std::time::Duration;

    use clap::{Arg, Command};

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_version_flag(false)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A tray application for monitoring the new Steam Controller")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Use verbose output "),
        )
        .get_matches();

    VERBOSE.set(matches.get_flag("verbose")).unwrap();

    let devices = loop {
        match connect_compatible_devices() {
            Ok(d) => break d,
            Err(e) => {
                let _ = proxy.send_event(Vec::new());
                eprintln!("Connecting failed with error: {e}");
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    };

    let mut controller_tx = devices
        .into_iter()
        .map(|mut device| {
            let (device_tx, mut device_rx) = multi_threading::create_controller_channel();
            let mut virt_controller = VirtualController::new().unwrap();
            std::thread::spawn(move || {
                // Run loop
                let mut run_counter = 0;
                loop {
                    match if run_counter % 300 == 0 {
                        device.active_refresh_state()
                    } else {
                        device.passive_refresh_state()
                    } {
                        Ok(input_events) => {
                            for input_event in input_events {
                                if let Err(e) = virt_controller.send_input(input_event) {
                                    debug_println!("{e}");
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("{error}");
                        }
                    };
                    device_rx.try_update_state(&device.device_properties());
                    run_counter += 1;
                }
            });
            device_tx
        })
        .collect::<Vec<ControllerSender>>();

    let (tx, rx) = mpsc::channel::<(u32, DeviceEvent)>();

    std::thread::spawn(move || {
        // Run loop
        loop {
            let first = rx.recv_timeout(Duration::from_millis(500));
            for (device_id, command) in first.into_iter().chain(rx.try_iter()) {
                controller_tx[device_id as usize].send_command(command);
            }
            let state = controller_tx
                .iter()
                .map(|d| d.get_latest_properties())
                .collect::<Vec<DeviceProperties>>();
            let _ = proxy.send_event(state);
        }
    });

    event_loop.run_app(&mut TrayApp::new(tx)).unwrap();
}

#[cfg(target_os = "linux")]
fn main() {
    use clap::ArgAction;
    use clap::{Arg, Command};
    use open_steam_controller::devices::DeviceProperties;
    use open_steam_controller::multi_threading::ControllerSender;
    use open_steam_controller::virtual_controller::{AbstractVirtualController, VirtualController};
    use open_steam_controller::VERBOSE;
    use std::sync::mpsc;
    use std::time::Duration;

    use status_tray::{StatusTray, TrayHandler};

    if let Ok(name) = std::env::current_exe() {
        if let Some(name) = name.to_str() {
            if let Ok(askpass) = std::env::var("SUDO_ASKPASS") {
                if name == askpass {
                    open_steam_controller::act_as_askpass_handler();
                }
            }
        }
    }
    open_steam_controller::prompt_user_for_udev_rule();
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_version_flag(false)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A tray application for monitoring the new Steam Controller.")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Use verbose output "),
        )
        .arg(
            Arg::new("monochrome_icons")
                .long("monochrome-icons")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Use the symbolic (monochrome) variants of the system tray icons"),
        )
        .get_matches();

    VERBOSE.set(matches.get_flag("verbose")).unwrap();
    let monochrome_icons = matches.get_flag("monochrome_icons");

    let (tx, rx) = mpsc::channel();
    let tray_handler = TrayHandler::new(StatusTray::new(tx, monochrome_icons));

    let devices = loop {
        match connect_compatible_devices() {
            Ok(d) => break d,
            Err(e) => {
                tray_handler.clear_state();
                eprintln!("Connecting failed with error: {e}");
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    };

    let mut controller_tx = devices
        .into_iter()
        .map(|mut device| {
            let (device_tx, mut device_rx) = multi_threading::create_controller_channel();
            let mut virt_controller = VirtualController::new().unwrap();
            std::thread::spawn(move || {
                // Run loop
                let mut run_counter = 0;
                loop {
                    match if run_counter % 300 == 0 {
                        device.active_refresh_state()
                    } else {
                        device.passive_refresh_state()
                    } {
                        Ok(input_events) => {
                            for input_event in input_events {
                                if let Err(e) = virt_controller.send_input(input_event) {
                                    debug_println!("{e}");
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("{error}");
                        }
                    };
                    device_rx.try_update_state(&device.device_properties());
                    run_counter += 1;
                }
            });
            device_tx
        })
        .collect::<Vec<ControllerSender>>();

    // Run loop
    loop {
        let first = rx.recv_timeout(Duration::from_millis(500));
        for (device_id, command) in first.into_iter().chain(rx.try_iter()) {
            controller_tx[device_id as usize].send_command(command);
        }
        let a = &controller_tx
            .iter()
            .map(|d| d.get_latest_properties())
            .collect::<Vec<DeviceProperties>>();
        tray_handler.update(a);
    }
}
