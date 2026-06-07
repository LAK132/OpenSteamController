use std::sync::mpsc::Sender;

use ksni::{
    menu::{StandardItem, SubMenu},
    Handle, MenuItem, ToolTip, Tray, TrayService,
};
use open_steam_controller::{
    devices::{
        format_int_value, DeviceEvent, DeviceProperties, PropertyDescriptorWrapper, PropertyType,
    },
    APP_NAME_PRETTY,
};

pub struct TrayHandler {
    handle: Handle<StatusTray>,
}

const NO_COMPATIBLE_DEVICE: &str = "No compatible device found.\nIs the dongle plugged in?\nIf you are using Linux did you\nadd the Udev rules?";
const CONTROLLER_NOT_CONNECTED: &str = "Controller is not connected";
const ICON_NAME: &str = "input-gaming";

impl TrayHandler {
    pub fn new(tray: StatusTray) -> Self {
        let tray_service = TrayService::new(tray);
        let handle = tray_service.handle();
        tray_service.spawn();
        TrayHandler { handle }
    }

    pub fn update(&self, properties: &[DeviceProperties]) {
        self.handle.update(|tray| {
            tray.device_properties = properties.to_vec();
        })
    }

    pub fn clear_state(&self) {
        self.handle.update(|tray| {
            tray.device_properties = Vec::new();
        })
    }
}

pub struct StatusTray {
    device_properties: Vec<DeviceProperties>,
    update_sender: Sender<(u32, DeviceEvent)>,
    monochrome_icons: bool,
}

impl StatusTray {
    pub fn new(update_sender: Sender<(u32, DeviceEvent)>, monochrome_icons: bool) -> Self {
        StatusTray {
            device_properties: Vec::new(),
            update_sender,
            monochrome_icons,
        }
    }

    fn fallback_controller_icon(&self) -> &'static str {
        if self.monochrome_icons {
            "controller-symbolic"
        } else {
            "controller"
        }
    }

    fn exit_icon(&self) -> &'static str {
        if self.monochrome_icons {
            "application-exit-symbolic"
        } else {
            "application-exit"
        }
    }
}

impl Tray for StatusTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn icon_name(&self) -> String {
        ICON_NAME.to_string()
    }

    fn tool_tip(&self) -> ToolTip {
        if self.device_properties.is_empty() {
            return ToolTip {
                title: "Unknown".to_string(),
                description: NO_COMPATIBLE_DEVICE.to_string(),
                icon_name: self.fallback_controller_icon().into(),
                icon_pixmap: Vec::new(),
            };
        };
        let description = self
            .device_properties
            .iter()
            .enumerate()
            .map(|(device_id, property)| {
                format!(
                    "Controller: {device_id}\n{}",
                    if property.connected.unwrap_or(false) {
                        property
                            .to_string_with_padding(0)
                            .lines()
                            .filter(|l| !l.contains("Unknown"))
                            .collect::<Vec<&str>>()
                            .join("\n")
                    } else {
                        CONTROLLER_NOT_CONNECTED.to_string()
                    }
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        ToolTip {
            title: APP_NAME_PRETTY.to_string(),
            description,
            icon_name: ICON_NAME.to_string(),
            icon_pixmap: Vec::new(),
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let exit_icon = self.exit_icon();
        let make_exit = || StandardItem {
            label: "Quit".into(),
            icon_name: exit_icon.into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        };
        let mut menu_items: Vec<MenuItem<Self>> = Vec::new();

        if self.device_properties.is_empty() {
            menu_items.push(
                StandardItem {
                    label: NO_COMPATIBLE_DEVICE.to_string(),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );
            menu_items.push(MenuItem::Separator);
            menu_items.push(make_exit().into());
            return menu_items;
        };

        for (device_id, device_properties) in self.device_properties.iter().enumerate() {
            menu_items.push(
                StandardItem {
                    label: format!("Controller: {device_id}"),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );
            if !device_properties.connected.unwrap_or(false) {
                menu_items.push(
                    StandardItem {
                        label: CONTROLLER_NOT_CONNECTED.to_string(),
                        enabled: false,
                        ..Default::default()
                    }
                    .into(),
                );
                menu_items.push(MenuItem::Separator);
                continue;
            }
            for property in device_properties.get_properties() {
                match property {
                    PropertyDescriptorWrapper::Int(property, []) => {
                        let Some(current_value) = property.data else {
                            continue;
                        };
                        let create_event = property.create_event;
                        menu_items.push(
                            StandardItem {
                                label: format!(
                                    "{}: {}",
                                    property.pretty_name,
                                    format_int_value(current_value, property.suffix)
                                ),
                                enabled: false,
                                activate: Box::new(move |_| {
                                    let _ = (create_event)(!current_value);
                                }),
                                ..Default::default()
                            }
                            .into(),
                        );
                    }
                    PropertyDescriptorWrapper::Int(property, options) => {
                        let Some(current_value) = property.data else {
                            continue;
                        };
                        let create_event = property.create_event;
                        let sub_menu = options
                            .iter()
                            .map(|val| {
                                let update_sender = self.update_sender.clone();
                                StandardItem {
                                    label: format_int_value(*val, property.suffix),
                                    enabled: property.property_type == PropertyType::ReadWrite
                                        && property.data.is_some(),
                                    activate: Box::new(move |_| {
                                        if let Some(command) = (create_event)(*val) {
                                            let _ = update_sender.send((device_id as u32, command));
                                        }
                                    }),
                                    ..Default::default()
                                }
                                .into()
                            })
                            .collect();
                        menu_items.push(
                            SubMenu {
                                label: format!(
                                    "{}: {}",
                                    property.pretty_name,
                                    format_int_value(current_value, property.suffix)
                                ),
                                enabled: property.property_type == PropertyType::ReadWrite
                                    && property.data.is_some(),
                                submenu: sub_menu,
                                ..Default::default()
                            }
                            .into(),
                        );
                    }
                    PropertyDescriptorWrapper::Bool(property) => {
                        let Some(current_value) = property.data else {
                            continue;
                        };
                        let create_event = property.create_event;
                        let update_sender = self.update_sender.clone();
                        menu_items.push(
                            StandardItem {
                                label: format!(
                                    "{}: {}{}",
                                    property.pretty_name, current_value, property.suffix
                                ),
                                enabled: property.property_type == PropertyType::ReadWrite
                                    && property.data.is_some(),
                                activate: Box::new(move |_| {
                                    if let Some(command) = (create_event)(!current_value) {
                                        let _ = update_sender.send((device_id as u32, command));
                                    }
                                }),
                                ..Default::default()
                            }
                            .into(),
                        );
                    }
                    PropertyDescriptorWrapper::String(property) => {
                        let Some(current_value) = property.data else {
                            continue;
                        };
                        let create_event = property.create_event;
                        menu_items.push(
                            StandardItem {
                                label: format!(
                                    "{}: {}{}",
                                    property.pretty_name, current_value, property.suffix
                                ),
                                enabled: false,
                                activate: Box::new(move |_| {
                                    let _ = (create_event)(String::new());
                                }),
                                ..Default::default()
                            }
                            .into(),
                        );
                    }
                }
            }
            menu_items.push(MenuItem::Separator);
        }

        menu_items.push(make_exit().into());
        menu_items
    }
}
