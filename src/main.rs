use iced::widget::{column, container, row, svg, text, Column, Row, Rule};
use iced::{color, executor, theme, Alignment, Application, Command, Element, Length};

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

#[derive(Debug, Default)]
struct Infonator {}

#[derive(Debug, Clone)]
pub enum Message {
    EchoDone(Result<std::process::Output, String>),
}

async fn run_external_command(
    command_name: &str,
    args: Vec<&str>,
) -> Result<std::process::Output, String> {
    match std::process::Command::new(command_name).args(args).output() {
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
        (
            Self::default(),
            Command::perform(run_external_command("sleep", ["3"].into()), |output| {
                Message::EchoDone(output)
            }),
        )
    }

    fn title(&self) -> String {
        String::from("infonator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EchoDone(command) => {
                println!("{command:?}");
                Command::none()
            }
        }
    }

    fn theme(&self) -> theme::Theme {
        theme::Theme::Dark
    }

    fn view(&self) -> Element<Self::Message> {
        // primary info
        let svg_larger_height = Length::Fixed(100.);
        let svg_larger_width = Length::Fixed(100.);
        let primary_spacing = 10.;

        let wifi_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/wifi-high.svg");
        let wifi_svg = svg(wifi_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);

        let wifi_text = Column::new()
            .push(text("Storgaten48"))
            .push(text("192.168.1.116"));

        let wifi = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(wifi_svg)
            .push(wifi_text);

        let battery_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/battery-low.svg");
        let battery_svg = svg(battery_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let battery_text = Column::new().push(text("51%")).push(text("5h left"));

        let battery = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(battery_svg)
            .push(battery_text);

        let time_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/time.svg");
        let time_svg = svg(time_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let time_text = Column::new().push(text("10:30"));

        let time = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(time_svg)
            .push(time_text);

        let volume_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/volume-max.svg");
        let volume_svg = svg(volume_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_larger_width)
            .height(svg_larger_height);
        let volume_text = Column::new().push(text("80%"));

        let volume = Row::new()
            .align_items(Alignment::Center)
            .spacing(primary_spacing)
            .push(volume_svg)
            .push(volume_text);

        // secondary info
        let svg_smaller_height = Length::Fixed(20.);
        let svg_smaller_width = Length::Fixed(20.);
        let secondary_spacing = 6.;

        let brightness_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/brightness.svg");
        let brightness_svg = svg(brightness_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let brightness_text = Column::new().push(text("90%"));
        let brightness = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(brightness_svg)
            .push(brightness_text);

        let cpu_svg_handle = svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/cpu.svg");
        let cpu_svg = svg(cpu_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let cpu_text = Column::new().push(text("20°C"));
        let cpu = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(cpu_svg)
            .push(cpu_text);

        let date_svg_handle =
            svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/date.svg");
        let date_svg = svg(date_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let date_text = Column::new().push(text("10/9/2023"));
        let date = Row::new()
            .align_items(Alignment::Center)
            .spacing(secondary_spacing)
            .push(date_svg)
            .push(date_text);

        let ram_svg_handle = svg::Handle::from_path("/home/ask/kode/rust/infonator/assets/ram.svg");
        let ram_svg = svg(ram_svg_handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(150, 196, 255)),
            }))
            .width(svg_smaller_width)
            .height(svg_smaller_height);
        let ram_text = Column::new().push(text("15%"));
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
