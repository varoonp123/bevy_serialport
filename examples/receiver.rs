use std::time::Duration;

use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use clap::Parser;

use bevy_serialport::{Runtime, SerialPortPlugin, SerialResource};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    port: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 115_200)]
    rate: u32,
}

fn main() {
    let args = Args::parse();

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(16)))
        .add_plugin(LogPlugin)
        .add_plugins(MinimalPlugins)
        .add_plugin(SerialPortPlugin)
        .insert_resource(args)
        .add_startup_system(setup)
        .add_system(receive)
        .run()
}

fn setup(cmd_args: Res<Args>, mut serial_res: ResMut<SerialResource>, rt: Res<Runtime>) {
    serial_res
        .open(rt.clone(), &cmd_args.port, cmd_args.rate)
        .expect("open serial port error");
}

fn receive(mut serial_res: ResMut<SerialResource>, cmd_args: Res<Args>) {
    let messages = serial_res.view_messages(&cmd_args.port);

    for message in messages {
        info!("receive {:?}", message);
        serial_res.send_message(&cmd_args.port, message);
    }
}
