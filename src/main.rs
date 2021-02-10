extern crate termion;

use std::env;
use std::fs;
use std::process::Command;
use termion::color;

fn main() {
    //colors
    let black = color::Fg(color::Black);
    let lblack = color::Fg(color::LightBlack);
    let red = color::Fg(color::Red);
    let lred = color::Fg(color::LightRed);
    let green = color::Fg(color::Green);
    let lgreen = color::Fg(color::LightGreen);
    let yellow = color::Fg(color::Yellow);
    let lyellow = color::Fg(color::LightYellow);
    let blue = color::Fg(color::Blue);
    let lblue = color::Fg(color::LightBlue);
    let magenta = color::Fg(color::Magenta);
    let lmagenta = color::Fg(color::LightMagenta);
    let cyan = color::Fg(color::Cyan);
    let lcyan = color::Fg(color::LightCyan);
    let white = color::Fg(color::White);
    let lwhite = color::Fg(color::LightWhite);

    //OS
    let file = fs::read_to_string("/etc/os-release").expect("Your OS isn't supported yet.");
    let mut v: Vec<&str> = file.split('"').collect();
    let distro = v[1].to_ascii_uppercase();

    let shl = env::var("SHELL").unwrap();
    v = shl.split('/').collect();
    let shell = v[v.len() - 1].to_ascii_uppercase();

    //Get username
    let hme = env::var("HOME").unwrap();
    v = hme.split('/').collect();
    let user = v[v.len() - 1];
    //Get Host name
    let hst = Command::new("hostname")
        .output()
        .expect("Could not find hostname.");
    let host = String::from_utf8(hst.stdout).unwrap().replace("\n", "");

    //Model
    let mdl = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .expect("Error: file /sys/devices/virtual/dmi/id/product_name not found.");
    v = mdl.split('\n').collect();
    let model = v[0].to_string();
    println!(
        "{}@{}\n\rOS:    {}\n\rHOST:  {}\n\rSHELL: {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██\n{}██{}██{}██{}██{}██{}██{}██{}██{reset}",
        user,
        host,
        distro,
        model,
        shell,
        black,
        red,
        green,
        yellow,
        blue,
        magenta,
        cyan,
        white,
        lblack,
        lred,
        lgreen,
        lyellow,
        lblue,
        lmagenta,
        lcyan,
        lwhite,
        reset = color::Fg(color::Reset),
    );
}
