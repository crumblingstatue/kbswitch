//! Very simple keyboard layout switch utility
//! Primary purpose is to be invoked by shortcut (e.g. by window manager)

#![no_main]

use std::{os::unix::process::CommandExt, process::Command};

#[no_mangle]
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
    let Ok(out) = Command::new("setxkbmap").arg("-query").output() else {
        return;
    };
    let Ok(s) = std::str::from_utf8(&out.stdout) else {
        return;
    };
    let Some(layout_idx) = s.find("layout:") else {
        return;
    };
    let layout_idx = layout_idx + 7;
    let layout = s[layout_idx..].trim();
    f(layout)
}

fn set_layout(layout: &str) {
    let _ = Command::new("setxkbmap").arg(layout).exec();
}
