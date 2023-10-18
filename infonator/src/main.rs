use std::time::Instant;

use iced::keyboard::Event::KeyPressed;
use iced::widget::{column, container, row, svg, text, Column, Row, Rule};
use iced::window::Event::Unfocused;
use iced::{
    color, executor, subscription, theme, window, Alignment, Application, Command, Element, Event,
    Length, Subscription,
};

pub fn main() -> iced::Result {
    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            size: (800, 400),
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    };
    Infonator::run(settings)
}

#[derive(Debug)]
struct Infonator {
    wifi_name: String,
    wifi_quality: String,
    battery_percentage: String,
    battery_time_left: String,
    time: String,
    volume: String,
    brightness: String,
    date: String,
    cpu_temperature: String,
    ram_usage: String,
    //
    start: Instant,
}

#[rustfmt::skip]
impl Default for Infonator {
    fn default() -> Self {
        Self {
            wifi_name:          "...".into(),
            wifi_quality:       "...".into(),
            battery_percentage: "...".into(),
            battery_time_left:  "...".into(),
            time:               "...".into(),
            volume:             "...".into(),
            brightness:         "...".into(),
            date:               "...".into(),
            cpu_temperature:    "...".into(),
            ram_usage:          "...".into(),
            //
            start: Instant::now(),
            
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum Message {
    CmdDoneWifiName         (Result<std::process::Output, String>),
    CmdDoneWifiQuality      (Result<std::process::Output, String>),
    CmdDoneBatteryPercentage(Result<std::process::Output, String>),
    CmdDoneBatteryTimeLeft  (Result<std::process::Output, String>),
    CmdDoneTime             (Result<std::process::Output, String>),
    CmdDoneVolume           (Result<std::process::Output, String>),
    CmdDoneBrightness       (Result<std::process::Output, String>),
    CmdDoneDate             (Result<std::process::Output, String>),
    CmdDoneCpuTemperature   (Result<std::process::Output, String>),
    CmdDoneRamUsage         (Result<std::process::Output, String>),
    EventOccurred(Event),
}

async fn run_external_command(
    command_name: String,
    args: Vec<String>,
) -> Result<std::process::Output, String> {
    let path = std::path::PathBuf::from(command_name);
    match shared::run_user_script(&path) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e}")),
    }
}

impl Application for Infonator {
    type Message = Message;
    type Theme = theme::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = shared::Settings::load().unwrap_or_default();
        (
            Self::default(),
            Command::batch([
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_wifi_name
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneWifiName(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_wifi_quality
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneWifiQuality(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_battery_percentage
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneBatteryPercentage(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_battery_time_left
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneBatteryTimeLeft(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_time
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneTime(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_volume
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneVolume(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_brightness
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneBrightness(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_date
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneDate(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_cpu_temperature
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneCpuTemperature(output),
                ),
                Command::perform(
                    run_external_command(
                        settings
                            .script_path_ram_usage
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                        [].into(),
                    ),
                    |output| Message::CmdDoneRamUsage(output),
                ),
            ]), // Command::perform(run_external_command("date", ["3"].into()), |output| {
                //     Message::EchoDone(output)
                // }),
        )
    }

    fn title(&self) -> String {
        String::from("infonator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if self.start.elapsed().as_millis() < 500 {
                    return Command::none();
                }
                match event {
                    Event::Keyboard(KeyPressed { .. }) | Event::Window(Unfocused) => {
                        window::close()
                    }
                    _ => Command::none(),
                }
            }
            Message::CmdDoneWifiName(output) => match output {
                Ok(v) => {
                    self.wifi_name = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneWifiQuality(output) => match output {
                Ok(v) => {
                    self.wifi_quality = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneBatteryPercentage(output) => match output {
                Ok(v) => {
                    self.battery_percentage = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneBatteryTimeLeft(output) => match output {
                Ok(v) => {
                    self.battery_time_left = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneTime(output) => match output {
                Ok(v) => {
                    self.time = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneVolume(output) => match output {
                Ok(v) => {
                    self.volume = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneBrightness(output) => match output {
                Ok(v) => {
                    self.brightness = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneDate(output) => match output {
                Ok(v) => {
                    self.date = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneCpuTemperature(output) => match output {
                Ok(v) => {
                    self.cpu_temperature = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
            Message::CmdDoneRamUsage(output) => match output {
                Ok(v) => {
                    self.ram_usage = String::from_utf8(v.stdout).unwrap();
                    Command::none()
                }
                Err(_) => Command::none(),
            },
        }
    }

    fn theme(&self) -> theme::Theme {
        theme::Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Self::Message> {
        // primary info
        let svg_larger_height = Length::Fixed(100.);
        let svg_larger_width = Length::Fixed(100.);
        let primary_spacing = 10.;

        let wifi_svg_handle = svg::Handle::from_path(format!(
            "{}/assets/wifi-high.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let wifi_svg = svg(wifi_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);

        let wifi_text = Column::new()
            .push(text(self.wifi_name.clone()))
            .push(text("192.168.1.116"))
            .push(text(self.wifi_quality.clone()));

        let wifi = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(wifi_svg)
            .push(wifi_text);

        let battery_svg_handle = svg::Handle::from_path(format!(
            "{}/assets/battery-low.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let battery_svg = svg(battery_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let battery_text = Column::new()
            .push(text(self.battery_percentage.clone()))
            .push(text(self.battery_time_left.clone()));

        let battery = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(battery_svg)
            .push(battery_text);

        let time_svg_handle =
            svg::Handle::from_path(format!("{}/assets/time.svg", env!("CARGO_MANIFEST_DIR")));
        let time_svg = svg(time_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let time_text = Column::new().push(text(self.time.clone()));

        let time = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(time_svg)
            .push(time_text);

        let volume_svg_handle = svg::Handle::from_path(format!(
            "{}/assets/volume-max.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let volume_svg = svg(volume_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let volume_text = Column::new().push(text(self.volume.clone()));

        let volume = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(volume_svg)
            .push(volume_text);

        // secondary info
        let svg_smaller_height = Length::Fixed(20.);
        let svg_smaller_width = Length::Fixed(20.);
        let secondary_spacing = 6.;

        let brightness_svg_handle = svg::Handle::from_path(format!(
            "{}/assets/brightness.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let brightness_svg = svg(brightness_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let brightness_text = Column::new().push(text(self.brightness.clone()));
        let brightness = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(brightness_svg)
            .push(brightness_text);

        let cpu_svg_handle =
            svg::Handle::from_path(format!("{}/assets/cpu.svg", env!("CARGO_MANIFEST_DIR")));
        let cpu_svg = svg(cpu_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let cpu_text = Column::new().push(text(self.cpu_temperature.clone()));
        let cpu = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(cpu_svg)
            .push(cpu_text);

        let date_svg_handle =
            svg::Handle::from_path(format!("{}/assets/date.svg", env!("CARGO_MANIFEST_DIR")));
        let date_svg = svg(date_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let date_text = Column::new().push(text(self.date.clone()));
        let date = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(date_svg)
            .push(date_text);

        let ram_svg_handle =
            svg::Handle::from_path(format!("{}/assets/ram.svg", env!("CARGO_MANIFEST_DIR")));
        let ram_svg = svg(ram_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let ram_text = Column::new().push(text(self.ram_usage.clone()));
        let ram = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(ram_svg)
            .push(ram_text);

        let info_primary = row!(column!(wifi, battery), column!(time, volume)).spacing(40.);
        let ruler_thickness = 10.;
        container(
            column!(
                row!(brightness, date).spacing(20.),
                Rule::horizontal(ruler_thickness),
                info_primary,
                Rule::horizontal(ruler_thickness),
                row!(cpu, ram).spacing(20.),
            )
            .align_items(Alignment::Center),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
