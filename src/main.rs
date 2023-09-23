use std::{
    io,
    process::{exit, Command, ExitStatus},
};

use argparse::{ArgumentParser, Store, StoreTrue};

const PACTL: &str = "pactl";
const MUTE_COMMAND: &str = "set-sink-mute";
const VOLUME_CHANGE_COMMAND: &str = "set-sink-volume";
const TOGGLE_ARGUMENT: &str = "toggle";

fn failure(msg: &str) -> ! {
    eprintln!("Failure: {}", msg);
    exit(1);
}

fn pactl_command_report(exit_status: io::Result<ExitStatus>, sink_id: u8, command_label: &str) {
    let exit_status = match exit_status {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Could not execute {} command on sink: {}: {}",
                sink_id, command_label, e
            );
            return;
        }
    };

    if !exit_status.success() {
        eprintln!(
            "Toggle command on sink {} did not succeed: {}",
            sink_id, exit_status
        );
    }
}

fn main() {
    let mut toggle = false;
    let mut volume_change: i32 = 0;
    let mut needle: String = String::from("RUNNING");
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut toggle).add_option(
            &["-t", "--toggle"],
            StoreTrue,
            "Toggle mute on matching sinks (default: false)",
        );

        ap.refer(&mut volume_change).add_option(
            &["-c", "--volume-change"],
            Store,
            "Volume change percentage on matching sinks (default: 0)",
        );

        ap.refer(&mut needle).add_option(
            &["-n", "--needle"],
            Store,
            "Substring that triggers matching on `pactl list sinks short` command output lines (default: RUNNING)",
        );

        ap.refer(&mut verbose).add_option(
            &["-v", "--verbose"],
            StoreTrue,
            "Verbose output (default: false)",
        );

        ap.parse_args_or_exit();
    }
    let pactl_volume_change = format!(
        "{}{}%",
        if volume_change > 0 { "+" } else { "" },
        volume_change
    );

    let output = match Command::new(PACTL)
        .arg("list")
        .arg("sinks")
        .arg("short")
        .output()
    {
        Ok(o) => o,
        Err(e) => failure(format!("Could not get `pactl list sinks short` output: {}", e).as_str()),
    };
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    let mut sinks: Vec<u8> = vec![];
    for line in output.split("\n") {
        if !line.contains(&needle) {
            continue;
        }

        let sink_tab_split: Vec<&str> = line.split("\t").collect();
        if sink_tab_split.is_empty() {
            eprintln!("Could not split output line: {}", line);
            continue;
        }
        let sink_id = match sink_tab_split[0].parse::<u8>() {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Could not parse sink id: {}: {}", sink_tab_split[0], e);
                continue;
            }
        };

        sinks.push(sink_id)
    }

    for id in sinks {
        if toggle {
            if verbose {
                println!("{} {} {} {}", PACTL, MUTE_COMMAND, id, TOGGLE_ARGUMENT);
            }

            let exit_status = Command::new(PACTL)
                .arg(MUTE_COMMAND)
                .arg(format!("{}", id))
                .arg(TOGGLE_ARGUMENT)
                .status();

            pactl_command_report(exit_status, id, "toggle");
        }

        if volume_change != 0 {
            if verbose {
                println!(
                    "{} {} {} {}",
                    PACTL, VOLUME_CHANGE_COMMAND, id, pactl_volume_change
                );
            }

            let exit_status = Command::new(PACTL)
                .arg(VOLUME_CHANGE_COMMAND)
                .arg(format!("{}", id))
                .arg(pactl_volume_change.as_str())
                .status();

            pactl_command_report(exit_status, id, "volume change");
        }
    }
}
