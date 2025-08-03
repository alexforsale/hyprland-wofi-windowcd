use hyprland::data::Clients;
use hyprland::shared::HyprData;
use regex::Regex;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> hyprland::Result<()> {
    let clients: Clients = Clients::get()?;
    let mut windows: Vec<String> = Vec::new();
    let special = Regex::new(r"^special:.*").unwrap();

    //dbg!(&clients);
    for c in clients {
        if c.class != "" && !special.is_match(&c.workspace.name) {
            windows.push(c.class)
        }
    }
    // println!("Windows are {:?}", windows);
    let mut input: String = "".to_owned();
    for window in windows {
        input.push_str(&window);
        input.push_str("\n");
    }

    let mut wofi = Command::new("wofi")
        .arg("--show")
        .arg("dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = wofi.stdin.take().expect("Failed to open stdin");
    stdin.write_all(input.as_bytes())?;
    drop(stdin);

    let output = wofi.wait_with_output()?;

    let command_output = String::from_utf8_lossy(&output.stdout);
    // dbg!(&command_output.trim());

    let focused_window = format!("class:{}", command_output.trim());
    // this should also be done using Dispatch::call()
    // but I still haven't figure it out
    let focus_command = Command::new("hyprctl")
        .arg("dispatch")
        .arg("focuswindow")
        .arg(focused_window)
        .output()?;

    if focus_command.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&focus_command.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&focus_command.stderr));
    } else {
        eprintln!("Command failed with status: {}", focus_command.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&focus_command.stderr));
    }
    Ok(())
}
