extern crate termion;

use std::env;
use std::fs;
use termion::color;

fn main() {
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
    let file = fs::read_to_string("/etc/os-release").expect("Your OS isn't supported yet.");
    let mut v: Vec<&str> = file.split('"').collect();
    let distro = v[1].to_ascii_uppercase();
    let shl = env::var("SHELL").unwrap();
    v = shl.split('/').collect();
    let shell = v[v.len() - 1];
    println!(
        "OS:    {}\n\rShell: {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██\n{}██{}██{}██{}██{}██{}██{}██{}██{reset}",
        distro,
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
