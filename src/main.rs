//! Very simple keyboard layout switch utility
//! Primary purpose is to be invoked by shortcut (e.g. by window manager)

use std::process::Command;

fn main() {
    with_query_layout(|layout| {
        set_layout(determine_new_layout(layout));
    })
}

/// This is where the layout switching behavior can be configured
fn determine_new_layout(old: &str) -> &str {
    match old {
        "us" => "hu",
        _ => "us",
    }
}

fn with_query_layout(f: fn(&str)) {
    let out = Command::new("setxkbmap").arg("-query").output().unwrap();
    let s = std::str::from_utf8(&out.stdout).unwrap();
    let layout_idx = s.find("layout:").unwrap() + 7;
    let layout = s[layout_idx..].trim();
    f(layout)
}

fn set_layout(layout: &str) {
    Command::new("setxkbmap").arg(layout).status().unwrap();
}
