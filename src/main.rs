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
    //MIT License

    //Copyright (c) 2019-2020 The rsfetch contributors

    //Permission is hereby granted, free of charge, to any person obtaining a copy
    //of this software and associated documentation files (the "Software"), to deal
    //in the Software without restriction, including without limitation the rights
    //to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    //copies of the Software, and to permit persons to whom the Software is
    //furnished to do so, subject to the following conditions:
    //
    //The above copyright notice and this permission notice shall be included in all
    //copies or substantial portions of the Software.

    let hst = Command::new("hostname")
        .output()
        .expect("Could not find hostname.");
    let host = String::from_utf8(hst.stdout).unwrap().replace("\n", "");

    //Model
    let mdl = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .expect("Error: file /sys/devices/virtual/dmi/id/product_name not found.");
    v = mdl.split('\n').collect();
    let model = v[0].to_string();

    //Kernel
    //MIT License

    //Copyright (c) 2019-2020 The rsfetch contributors

    //Permission is hereby granted, free of charge, to any person obtaining a copy
    //of this software and associated documentation files (the "Software"), to deal
    //in the Software without restriction, including without limitation the rights
    //to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    //copies of the Software, and to permit persons to whom the Software is
    //furnished to do so, subject to the following conditions:
    //
    //The above copyright notice and this permission notice shall be included in all
    //copies or substantial portions of the Software.
    let krnl = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not find kernel version.");
    let kernl = String::from_utf8(krnl.stdout).unwrap().replace("\n", "");
    let kernel = kernl.clone();
    let uptime = format_uptime();
    println!(
        "{}@{}\n\rOS:     {}\n\rHOST:   {}\n\rKERNEL: {}\n\rUPTIME: {}\n\rSHELL:  {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██\n{}██{}██{}██{}██{}██{}██{}██{}██{reset}",
        user,
        host,
        distro,
        model,
        kernel,
        uptime,
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

fn format_uptime() -> String {
    //MIT License

    //Copyright (c) 2019-2020 The rsfetch contributors

    //Permission is hereby granted, free of charge, to any person obtaining a copy
    //of this software and associated documentation files (the "Software"), to deal
    //in the Software without restriction, including without limitation the rights
    //to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    //copies of the Software, and to permit persons to whom the Software is
    //furnished to do so, subject to the following conditions:
    //
    //The above copyright notice and this permission notice shall be included in all
    //copies or substantial portions of the Software.
    let mut sec: &str = &*fs::read_to_string("/proc/uptime").expect("File /proc/uptime not found.");
    sec = sec.split('.').collect::<Vec<&str>>()[0];
    let secs = sec.parse::<u64>().unwrap();
    let days = secs / 60 / 60 / 24;
    let hours = (secs / 60 / 60) % 24;
    let minutes = (secs / 60) % 60;
    let mut uptime = "".to_string();
    if days != 0 {
        uptime = format!("{} days, {} hours, {} mins", days, hours, minutes);
    } else {
        if hours != 0 {
            uptime = format!("{} hours, {} mins", hours, minutes);
        } else {
            uptime = format!("{} mins", minutes);
        }
    }
    uptime.to_string()
}
