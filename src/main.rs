use std::{env, fs, process::Command};

fn main() {
    let mut v: Vec<&str>;
    let distro = if env::consts::OS == "linux" {
        get_distro()
    }
    else {
        env::consts::OS.to_string().to_ascii_uppercase()
    };
    let result = parse_args(&distro);
    let distro_art = match result {
        | Ok(s) => s.to_ascii_uppercase().replace("_", " "),
        | Err(()) => "".to_string()
    };
    // Get DE
    let de: String;
    let decheck: bool = env::var("DESKTOP_SESSION").is_err();
    if decheck {
        de = "NOT FOUND".to_string();
    }
    else {
        de = env::var("DESKTOP_SESSION")
            .unwrap()
            .to_string()
            .to_ascii_uppercase();
    }

    // Shell
    let shl: String;
    let shell: String;
    let shcheck: bool = env::var("SHELL").is_err();
    if shcheck {
        shell = "NOT FOUND".to_string();
    }
    else {
        shl = env::var("SHELL").unwrap().to_string();
        v = shl.split('/').collect();
        shell = v[v.len() - 1].to_ascii_uppercase();
    }
    // Get username
    let hme = env::var("HOME").unwrap();
    v = hme.split('/').collect();
    let user = v[v.len() - 1];

    // Get hostname
    let host = fs::read_to_string("/proc/sys/kernel/hostname")
        .expect("Error: file /proc/sys/kernel/hostname does not exist.")
        .replace("\n", "");

    // Model
    let mdl = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .expect("Error: file /sys/devices/virtual/dmi/id/product_name not found.");
    v = mdl.split('\n').collect();
    let model = v[0].to_ascii_uppercase().to_string();

    // Kernel
    // MIT License

    // Copyright (c) 2019-2020 The rsfetch contributors

    // Permission is hereby granted, free of charge, to any person obtaining a copy
    // of this software and associated documentation files (the "Software"), to deal
    // in the Software without restriction, including without limitation the rights
    // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    // copies of the Software, and to permit persons to whom the Software is
    // furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in all
    // copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    // SOFTWARE.
    let krnl = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not find kernel version. Uname is a dependency of XEFETCH");
    let kernl = String::from_utf8(krnl.stdout).unwrap().replace("\n", "");
    let kernel = kernl.clone();
    let uptime = format_uptime();
    // Get arch
    let arch: String = env::consts::ARCH.to_string();
    // Get CPU
    let comp = fs::read_to_string("/proc/cpuinfo").expect("Error: /proc/cpuinfo does not exist.");
    v = comp.split("\n").collect();
    // Split apart the lines and read line #4
    let cpuq = v[4].to_string();
    v = cpuq.split(":").collect();
    let mut cpu = v[1].to_string();
    // Get the model
    cpu = cpu[1 ..]
        .replace("(R)", "")
        .replace("Core", "")
        .replace("(TM) ", "")
        .replace("CPU ", "")
        .replace(" Processor", "")
        .to_string();

    // Get packages
    let pkgs = get_pkgs();
    if &distro_art[..] != "" {
        output(
            user.to_string(),
            host,
            model,
            distro,
            distro_art,
            arch,
            kernel.to_string(),
            uptime,
            shell,
            de,
            cpu,
            pkgs
        );
    }
}

fn parse_args(distro: &String) -> Result<String, ()> {
    let args: Vec<String> = env::args().collect();
    let mut l = false; // Use a different logo art.
    let mut help = false; // Decide whether to print the help message or not.
    let mut logo: String = distro.to_string();
    for c in args {
        if l == true {
            l = false;
            logo = c.clone();
        }
        if &c[..] == "-l" || &c[..] == "--logo" {
            l = true;
        }
        if &c[..] == "-h" || &c[..] == "--help" {
            help = true;
            println!(
                "Usage: xefetch [Options]\n\
                 Options:\n\
                      -h or --help: Prints this message.\n\
                      -l or --logo: Changes the logo art. Ex: xefetch -l void_linux\n"
            );
        }
    }
    if help {
        Err(())
    }
    else {
        Ok(logo)
    }
}

fn get_distro() -> String {
    // OS
    let file = fs::read_to_string("/etc/os-release")
        .expect("Your OS isn't supported yet. Please add a /etc/os-release to use XEFETCH.");
    let mut v: Vec<&str> = file.split('\n').collect();
    let mut distro: String;
    let mut distro2: String = v[0].to_string();
    let mut i = 0;
    while i < (v.len() - 1) {
        let os = v[i].to_string();
        let os2: String = os[0 .. 5].to_string();
        if os2 == "NAME=".to_string() {
            distro2 = v[i].to_string()
        }
        i += 1;
    }
    v = distro2.split("=").collect();
    distro = v[1].to_ascii_uppercase();
    let vc: Vec<char> = distro.chars().collect();
    if vc[0] == '"' {
        distro = distro.replace("\"", "").to_string();
    }
    if &distro[..] == "VOID" || &distro[..] == "GENTOO" {
        distro = format!("{} LINUX", distro);
    }
    distro
}

fn format_uptime() -> String {
    // MIT License

    // Copyright (c) 2019-2020 The rsfetch contributors

    // Permission is hereby granted, free of charge, to any person obtaining a copy
    // of this software and associated documentation files (the "Software"), to deal
    // in the Software without restriction, including without limitation the rights
    // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    // copies of the Software, and to permit persons to whom the Software is
    // furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in all
    // copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    // SOFTWARE.

    let mut sec: &str = &*fs::read_to_string("/proc/uptime").expect("File /proc/uptime not found.");
    sec = sec.split('.').collect::<Vec<&str>>()[0];
    let secs = sec.parse::<u64>().unwrap();
    let days = secs / 60 / 60 / 24;
    let hours = (secs / 60 / 60) % 24;
    let minutes = (secs / 60) % 60;
    let uptime: String;
    if days != 0 {
        uptime = format!("{} DAYS, {} HOURS, {} MINS", days, hours, minutes);
    }
    else {
        if hours != 0 {
            uptime = format!("{} HOURS, {} MINS", hours, minutes);
        }
        else {
            uptime = format!("{} MINS", minutes);
        }
    }
    uptime.to_string()
}

fn get_pkgs() -> String {
    // Convert to a &str
    let mut pkg: Vec<String> = Vec::new();

    // XBPS
    match Command::new("xbps-query").arg("-l").output() {
        | Ok(_) => {
            let pkgx = Command::new("xbps-query").arg("-l").output().expect("");
            let pkgsx = String::from_utf8(pkgx.stdout).unwrap();
            let pkgxs: Vec<&str> = pkgsx.split("\n").collect();
            pkg.push(format!("{pgk}(xbps), ", pgk = (pkgxs.len() - 1)));
        },
        | Err(_why) => {}
    }
    // APK
    match Command::new("apk").arg("info").output() {
        | Ok(_) => {
            let pkga = Command::new("apk").arg("info").output().expect("");
            let pkgsa = String::from_utf8(pkga.stdout).unwrap();
            let pkgas: Vec<&str> = pkgsa.split("\n").collect();
            pkg.push(format!("{pgk}(apk), ", pgk = (pkgas.len() - 1)));
        },
        | Err(_why) => {}
    }

    // Flatpak
    match Command::new("flatpak").arg("list").output() {
        | Ok(_) => {
            let pkgf = Command::new("flatpak").arg("list").output().expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(flatpak), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // Apt
    match Command::new("apt").arg("--installed").output() {
        | Ok(_) => {
            let pkgf = Command::new("apt").arg("--installed").output().expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(apt), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // Dnf
    match Command::new("dnf").arg("list").arg("installed").output() {
        | Ok(_) => {
            let pkgf = Command::new("dnf")
                .arg("list")
                .arg("installed")
                .output()
                .expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(dnf), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // pacman
    match Command::new("pacman").arg("-Q").arg("-q").output() {
        | Ok(_) => {
            let pkgf = Command::new("pacman")
                .arg("-Q")
                .arg("-q")
                .output()
                .expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pacman), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // portage
    match Command::new("qlist").arg("-l").output() {
        | Ok(_) => {
            let pkgf = Command::new("qlist").arg("-l").output().expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(portage), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // Zypper
    match Command::new("zypper")
        .arg("se")
        .arg("--installed-only")
        .output()
    {
        | Ok(_) => {
            let pkgf = Command::new("zypper")
                .arg("se")
                .arg("--installed-only")
                .output()
                .expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(zypper), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // nix
    match Command::new("nix-env")
        .arg("-qa")
        .arg("--installed")
        .arg("\"*\"")
        .output()
    {
        | Ok(_) => {
            let pkgf = Command::new("nix-env")
                .arg("-qa")
                .arg("--installed")
                .arg("\"*\"")
                .output()
                .expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(nix), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }

    // snapd(ew)
    match Command::new("snap").arg("list").output() {
        | Ok(_) => {
            let pkgf = Command::new("snap").arg("list").output().expect("");
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(snapd), ", pgk = (pkgfs.len() - 1)));
        },
        | Err(_why) => {}
    }
    // Return list
    let mut pkgs: String = pkg.into_iter().collect::<String>();
    let mut v: Vec<char> = pkgs.chars().collect();
    v.remove(v.len() - 2);
    pkgs = v.into_iter().collect();
    pkgs.to_ascii_uppercase()
}

fn output(
    user: String, host: String, model: String, distro: String, distro_art: String, arch: String,
    kernel: String, uptime: String, shell: String, de: String, cpu: String, pkgs: String
) {
    // colors
    let black = format!("{}{}", 27 as char, "[38;5;0m");
    let lblack = format!("{}{}", 27 as char, "[38;5;8m");
    let red = format!("{}{}", 27 as char, "[38;5;1m");
    let lred = format!("{}{}", 27 as char, "[38;5;9m");
    let green = format!("{}{}", 27 as char, "[38;5;2m");
    let lgreen = format!("{}{}", 27 as char, "[38;5;10m");
    let yellow = format!("{}{}", 27 as char, "[38;5;3m");
    let lyellow = format!("{}{}", 27 as char, "[38;5;11m");
    let blue = format!("{}{}", 27 as char, "[38;5;4m");
    let lblue = format!("{}{}", 27 as char, "[38;5;12m");
    let magenta = format!("{}{}", 27 as char, "[38;5;5m");
    let lmagenta = format!("{}{}", 27 as char, "[38;5;13m");
    let cyan = format!("{}{}", 27 as char, "[38;5;6m");
    let lcyan = format!("{}{}", 27 as char, "[38;5;14m");
    let white = format!("{}{}", 27 as char, "[38;5;7m");
    let lwhite = format!("{}{}", 27 as char, "[38;5;15m");
    let reset = format!("{}{}", 27 as char, "[39m");
    let bold = format!("{}{}", 27 as char, "[1m");
    let nbold = format!("{}{}", 27 as char, "[0m");

    match &distro_art[..] {
        | "VOID LINUX" => {
            print!(
                "{}{}   dMMMMMMb      {}{}{}@{}{}{}\n\r dMMV`  `*VMb    OS:{}{}     {} {}",
                bold, lgreen, user, nbold, reset, bold, lgreen, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{} `V  ,mm.  VMA   HOST:{}{}   {}\n\r{}{}A,  (MMMM)  `V   KERNEL:{}{} {}",
                bold, lgreen, nbold, reset, model, bold, lgreen, nbold, reset, kernel
            );
            print!(
                "\n\r{}{}VMA  `**'  A,    UPTIME:{}{} {}\n\r{}{} PMAm-__-AMMP    SHELL:{}{}  {}",
                bold, lgreen, nbold, reset, uptime, bold, lgreen, nbold, reset, shell
            );
            print!(
                "\n\r{}{}   PMMMMMMP      DE:{}{}     {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}CPU:{}{}    {}",
                bold, lgreen, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, lgreen, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                lgreen,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },
        | "ARCH LINUX" => {
            print!(
                "{}{}       A         {}{}{}@{}{}{}\n\r      AMA        OS:{}{}     {} {}",
                bold, blue, user, nbold, reset, bold, blue, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{}     `*<MA       HOST:{}{}   {}\n\r{}{}    AY>,MMA      KERNEL:{}{} {}",
                bold, blue, nbold, reset, model, bold, blue, nbold, reset, kernel
            );
            print!(
                "\n\r{}{}   AMMY`YMMA     UPTIME:{}{} {}\n\r{}{}  AMM0   0A>*    SHELL:{}{}  {}",
                bold, blue, nbold, reset, uptime, bold, blue, nbold, reset, shell
            );
            print!(
                "\n\r{}{} AM-*     *-MA   DE:{}{}     {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}CPU:{}{}    {}",
                bold, blue, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, blue, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                blue,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },
        | "LINUX MINT" => {
            print!(
                "{}{}MMMMMMMMMMMMo    {}{}{}@{}{}{}\n\rM   _       `o   OS:{}{}     {} {}",
                bold, lgreen, user, nbold, reset, bold, lgreen, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{}MMo M  m\"m\"m M   HOST:{}{}   {}\n\r{}{}  M M  M M M M   KERNEL:{}{} {}",
                bold, lgreen, nbold, reset, model, bold, lgreen, nbold, reset, kernel
            );
            print!(
                "\n\r{}{}  M M  \" \" # M   UPTIME:{}{} {}\n\r{}{}  o `\"\"\" **` M   SHELL:{}{}  {}",
                bold, lgreen, nbold, reset, uptime, bold, lgreen, nbold, reset, shell
            );
            print!(
                "\n\r{}{}   *MMMMMMMMMM   DE:{}{}     {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}CPU:{}{}    {}",
                bold, lgreen, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, lgreen, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                lgreen,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },

        | "GENTOO LINUX" => {
            print!(
                "{}{}     .----.      {}{}{}@{}{}{}\n\r   .`   _  `.    OS:{}{}     {} {}",
                bold, magenta, user, nbold, reset, bold, magenta, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{}   Vm, (_)`\"`.   HOST:{}{}   {}\n\r{}{}    `V.   ` ,V   KERNEL:{}{} {}",
                bold, magenta, nbold, reset, model, bold, magenta, nbold, reset, kernel
            );
            print!(
                "\n\r{}{}    .`   `,/V`   UPTIME:{}{} {}\n\r{}{}   / ` ` AV`     SHELL:{}{}  {}",
                bold, magenta, nbold, reset, uptime, bold, magenta, nbold, reset, shell
            );
            print!(
                "\n\r{}{}   `<mmd*\"       DE:     {}{}{}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{}{} CPU:{}{}    {}",
                bold, magenta, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, magenta, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{}{} PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                magenta,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },

        | "ALPINE LINUX" => {
            print!(
                "{}{}   AMMMMMMMA     {}{}{}@{}{}{}\n\r  AMM/\\MM?MMA    OS:{}{}     {} {}",
                bold, blue, user, nbold, reset, bold, blue, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{} AMM/  \\/ \\MMA   HOST:{}{}   {}\n\r{}{} MM/ ,  \\  \\MM   KERNEL:{}{} {}",
                bold, blue, nbold, reset, model, bold, blue, nbold, reset, kernel
            );
            print!(
                "\n\r{}{} V/ /|   \\  \\V   UPTIME:{}{} {}\n\r{}{}  VMMMMMMMMMV    SHELL:{}{}  {}",
                bold, blue, nbold, reset, uptime, bold, blue, nbold, reset, shell
            );
            print!(
                "\n\r{}{}   VMMMMMMMV     DE:     {}{}{}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{}{} CPU:{}{}    {}",
                bold, blue, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, blue, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{}{} PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                blue,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },

        | "MX LINUX" => {
            print!(
                "{}{}       VA  A`    {}{}{}@{}{}{}\n\r        V,M`     OS:{}{}     {} {}",
                bold, lwhite, user, nbold, reset, bold, lwhite, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{}      A.,MA      HOST:{}{}   {}\n\r{}{}     AMM* VA     KERNEL:{}{} {}",
                bold, lwhite, nbold, reset, model, bold, lwhite, nbold, reset, kernel
            );
            print!(
                "\n\r{}{}    AMMMA  VA    UPTIME:{}{} {}\n\r{}{}   AMMMMMA.AMA   SHELL:{}{}  {}",
                bold, lwhite, nbold, reset, uptime, bold, lwhite, nbold, reset, shell
            );
            print!(
                "\n\r{}{}  AMMMMMMMMMMMA  DE:{}{}     {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{}{} CPU:{}{}    {}",
                bold, lwhite, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, lwhite, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        },

        | _ => {
            print!(
                "{}{}      ,aa,       {}{}{}@{}{}{}\n\r     ;A00A;      OS:{}{}     {} {}",
                bold, lwhite, user, nbold, reset, bold, lwhite, host, reset, nbold, distro, arch
            );
            print!(
                "\n\r{}{}     AV{}<>{}VA,     HOST:{}{}   {}\n\r{}{}    AMV  VMA     KERNEL:{}{} {}",
                bold,
                lwhite,
                yellow,
                lwhite,
                nbold,
                reset,
                model,
                bold,
                lwhite,
                nbold,
                reset,
                kernel
            );
            print!(
                "\n\r{}{}    MV    VM,    UPTIME:{}{} {}\n\r{}{}   J\\{}M;,,;M{}/L,   {}SHELL:{}{}  {}",
                bold,
                lwhite,
                nbold,
                reset,
                uptime,
                bold,
                yellow,
                lwhite,
                yellow,
                lwhite,
                nbold,
                reset,
                shell
            );
            print!(
                "\n\r{}{}   VMV{}\"\"\"\"{}VMV    {}DE:{}{}     {}\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}CPU:{}{}    {}",
                bold, yellow, lwhite, yellow, lwhite, nbold, reset, de, black, red, green, yellow, blue, magenta, cyan, white, lwhite, bold, nbold, reset, cpu,
            );
            print!(
                "\n\r{}██{}██{}██{}██{}██{}██{}██{}██{} {}PKGS:{}{}   {}\n\r{}{}{}{}",
                lblack,
                lred,
                lgreen,
                lyellow,
                lblue,
                lmagenta,
                lcyan,
                lwhite,
                lwhite,
                bold,
                nbold,
                reset,
                pkgs,
                bold,
                green,
                reset,
                nbold,
            );
        }
    }
}
