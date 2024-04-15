//! Very simple keyboard layout switch utility
//! Primary purpose is to be invoked by shortcut (e.g. by window manager)

#![no_main]

use std::{
    ffi::OsStr,
    os::unix::{ffi::OsStrExt, process::CommandExt},
    process::Command,
};

#[no_mangle]
fn main() {
    with_query_layout(|layout| {
        set_layout(determine_new_layout(layout));
    })
}

/// This is where the layout switching behavior can be configured
fn determine_new_layout(old: &[u8]) -> &[u8] {
    match old {
        b"us" => b"hu",
        _ => b"us",
    }
}

fn trim_bytestring(bs: &[u8]) -> &[u8] {
    let mut begin = 0;
    let mut end = 0;
    enum State {
        Init,
        Meat,
    }
    let mut state = State::Init;
    for (i, byte) in bs.iter().enumerate() {
        match state {
            State::Init => {
                if !byte.is_ascii_whitespace() {
                    state = State::Meat;
                    begin = i;
                }
            }
            State::Meat => {
                if byte.is_ascii_whitespace() {
                    end = i;
                    break;
                }
            }
        }
    }
    &bs[begin..end]
}

fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn with_query_layout(f: fn(&[u8])) {
    let Ok(out) = Command::new("setxkbmap").arg("-query").output() else {
        return;
    };
    let Some(layout_idx) = memmem(&out.stdout, b"layout:") else {
        return;
    };
    let layout_idx = layout_idx + 7;
    let layout = trim_bytestring(&out.stdout[layout_idx..]);
    f(layout)
}

fn set_layout(layout: &[u8]) {
    let _ = Command::new("setxkbmap")
        .arg(<OsStr as OsStrExt>::from_bytes(layout))
        .exec();
}
