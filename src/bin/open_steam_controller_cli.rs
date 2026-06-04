use std::{process::exit, time::Duration};

use clap::{Arg, ArgAction, Command};
use open_steam_controller::{
    devices::{
        connect_compatible_device, Controller, DeviceError, DeviceProperties,
        PropertyDescriptorWrapper,
    },
    VERBOSE,
};

#[allow(dead_code)]
const SHOW_ALL_OPTIONS: bool = false;

/// helper function to enable help messages
#[allow(dead_code)]
fn device_supports<F>(device: &Result<Controller, DeviceError>, f: F) -> bool
where
    F: FnOnce(&DeviceProperties) -> bool,
{
    device
        .as_ref()
        .map(|controller| f(&controller.device_properties()))
        .unwrap_or(false)
}

fn create_command(_device: &Result<Controller, DeviceError>) -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_version_flag(false)
        .disable_help_flag(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A CLI application for monitoring and managing the new Steam Controller.")
        .after_help("Help only lists commands supported by this controller.")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Use verbose output"),
        )
        .arg(
            Arg::new("help")
                .long("help")
                .short('h')
                .action(ArgAction::SetTrue)
                .help("Print help"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .default_value("false")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Use JSON output. Time is in seconds."),
        )
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        use open_steam_controller::act_as_askpass_handler;
        use open_steam_controller::prompt_user_for_udev_rule;

        if let Ok(name) = std::env::current_exe() {
            if let Some(name) = name.to_str() {
                if let Ok(askpass) = std::env::var("SUDO_ASKPASS") {
                    if name == askpass {
                        act_as_askpass_handler();
                    }
                }
            }
        }
        prompt_user_for_udev_rule();
    }

    let device = Err(DeviceError::NoDeviceFound());

    // prep help without any controller specific options
    let command = create_command(&device);
    let matches = command.get_matches();
    VERBOSE.set(matches.get_flag("verbose")).unwrap();

    let device = connect_compatible_device();

    // print help with controller specific options
    if matches.get_flag("help") {
        let mut command = create_command(&device);
        command.print_long_help().unwrap();
        exit(0);
    }

    let mut device = match device {
        Ok(device) => device,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    let commands = Vec::new();

    for command in commands {
        if let Err(e) = device.try_apply(command) {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }

    std::thread::sleep(Duration::from_secs_f64(0.5));

    // setting an option may cause a response form thecontroller
    if device.allow_passive_refresh() {
        if let Err(error) = device.passive_refresh_state() {
            eprintln!("{error}");
            std::process::exit(1);
        };
    }

    if let Err(error) = device.active_refresh_state() {
        eprintln!("{error}");
        std::process::exit(1);
    };

    if let Some(output_json) = matches.get_one::<bool>("json") {
        if *output_json {
            let properties = device.device_properties();
            let mut controller_info_json = "{\n  ".to_string();

            let json_properties: Vec<String> = properties
                .get_properties()
                .iter()
                .filter_map(|property| match property {
                    PropertyDescriptorWrapper::Int(property_descriptor, _items) => {
                        property_descriptor
                            .data
                            .map(|data| format!("\"{}\": {}", property_descriptor.name, data))
                    }
                    PropertyDescriptorWrapper::Bool(property_descriptor) => property_descriptor
                        .data
                        .map(|data| format!("\"{}\": {}", property_descriptor.name, data)),
                    PropertyDescriptorWrapper::String(property_descriptor) => property_descriptor
                        .data
                        .as_ref()
                        .map(|data| format!("\"{}\": \"{}\"", property_descriptor.name, data)),
                })
                .collect();

            controller_info_json += &json_properties.join(",\n  ");

            controller_info_json += "\n}";
            println!("{}", controller_info_json);
        } else {
            println!("{}", device.device_properties());
        }
    } else {
        println!("{}", device.device_properties());
    }
}
