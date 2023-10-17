#![allow(unused)]
use clap::{Arg, ArgAction, Command};

fn main() {
    let mut settings = match shared::Settings::load() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to load internal configuration script: {e}");
            shared::Settings::default()
        }
    };

    let matches = Command::new("infonator-set")
        .about("Configuration tool for infonator.")
        .long_about("This tool is used to configure the system information panel Infonator. It is use for customization, and to aquire system information. To be cross platform, it uses custom user scripts to aquire information. To see expected output for scripts, use `help` on the different `script-path` commands, example: `infonator-set script-path wifi-name --help`. You can output whatever text you want to stdout, but some icons like for example the battery, will react accordingly if you provide a percentage in the right format.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("script-path")
                .about("Path to script used to aquire information.")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("wifi-name").about("SSID of connected WiFi."),
                    Command::new("wifi-quality").about("Quality of connected WiFi in percentage."),
                    Command::new("battery-percentage").about("Percentage of battery charge."),
                    Command::new("battery-time-left")
                        .about("How many hours and minutes left of battery charge."),
                    Command::new("time").about("The current time, in for example 24 hours format."),
                    Command::new("volume").about("Speaker or headset volume in percentage."),
                    Command::new("brightness").about("Screen brightness in percentage."),
                    Command::new("date").about("Date, example format: day, month, year."),
                    Command::new("cpu-temperature").about("Average CPU temperature in Celsius degrees."),
                    Command::new("ram-usage").about("How much RAM memory is used, in percentage."),
                ].into_iter().map(|cmd|
                        cmd.arg_required_else_help(true).arg(Arg::new("PATH").help("Path to executable script that outputs corresponding information to stdout.").value_parser(clap::value_parser!(std::path::PathBuf)))
                    ).collect::<Vec<_>>(),
        )).subcommands([
            Command::new("close-on-any-key").about("Set to `true` if program should close on key input."),
            Command::new("content-size-ratio").about("Change the default size of the content with a decimal. 1.0 is deafult.")
        ]).get_matches();

    // test
    match matches.subcommand() {
        Some(("script-path", sub)) => match sub
            .subcommand()
            .map(|(name, sub)| (name, sub.get_one::<std::path::PathBuf>("PATH").unwrap()))
        {
            Some(("wifi-name", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_wifi_name = path.into();
                    print_script_path_success_message("wifi name", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("wifi-quality", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    match shared::extract_wifi_quality(
                        &String::from_utf8(output.stdout.clone()).unwrap(),
                    ) {
                        Ok(_) => print_recognized_yes("wifi quality"),
                        Err(e) => print_recognized_no("wifi quality"),
                    }
                    settings.script_path_wifi_quality = path.into();
                    print_script_path_success_message("wifi quality", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("battery-percentage", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    match shared::extract_battery_percentage(
                        &String::from_utf8(output.stdout.clone()).unwrap(),
                    ) {
                        Ok(_) => print_recognized_yes("battery percentage"),
                        Err(e) => print_recognized_no("battery percentage"),
                    }
                    settings.script_path_battery_percentage = path.into();
                    print_script_path_success_message("battery percentage", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("battery-time-left", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_battery_time_left = path.into();
                    print_script_path_success_message("battery, time left", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("time", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    match shared::extract_time(&String::from_utf8(output.stdout.clone()).unwrap()) {
                        Ok(_) => print_recognized_yes("time"),
                        Err(e) => print_recognized_no("time"),
                    }
                    settings.script_path_time = path.into();
                    print_script_path_success_message("time", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("volume", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_volume = path.into();
                    print_script_path_success_message("volume", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("brightness", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_brightness = path.into();
                    print_script_path_success_message("brightness", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("date", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_date = path.into();
                    print_script_path_success_message("date", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("cpu-temperature", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_cpu_temperature = path.into();
                    print_script_path_success_message("cpu temperature", path, &output);
                }
                Err(e) => todo!(),
            },
            Some(("ram-usage", path)) => match shared::run_user_script(path) {
                Ok(output) => {
                    settings.script_path_ram_usage = path.into();
                    print_script_path_success_message("ram usage", path, &output);
                }
                Err(e) => todo!(),
            },
            _ => {
                panic!("Unexpected argument. This is most likely a developer issue.");
            }
        },
        Some(("close-on-any-key", sub)) => {}
        Some(("content-size-ratio", sub)) => {}
        _ => {
            panic!("Unexpected argument. This is most likely a developer issue.")
        }
    }

    settings.save();
}

fn print_script_path_success_message(
    info_type: &str,
    script_path: &std::path::Path,
    script_output: &std::process::Output,
) {
    println!(
        "Set script path for {} to {}. Your script printed this to stdout:\n{}",
        info_type,
        script_path.display(),
        String::from_utf8(script_output.stdout.clone()).unwrap()
    );
}

fn print_recognized_yes(info_type: &str) {
    println!("Recognized {info_type} from script output.");
}
fn print_recognized_no(info_type: &str) {
    println!("Could not recognize {info_type} from script output.");
}
