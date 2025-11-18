use libc::{PTRACE_ATTACH, PTRACE_DETACH, PTRACE_POKEDATA, ptrace, waitpid};
use std::fs;
use std::io;
use std::{thread, time::Duration};
use sysinfo::*;

// Base addresses (module offsets)
pub const LOCAL_PLAYER: usize = 0x0017E0A8;
pub const ENTITY_LIST: usize = 0x018AC04;
pub const FOV: usize = 0x018A7CC;
pub const PLAYER_COUNT: usize = 0x018AC0C;

// Player struct offsets
pub const POS_X: usize = 0x2C;
pub const POS_Y: usize = 0x30;
pub const POS_Z: usize = 0x28;

pub const HEAD_X: usize = 0x4;
pub const HEAD_Y: usize = 0xC;
pub const HEAD_Z: usize = 0x8;

pub const CAMERA_X: usize = 0x34;
pub const CAMERA_Y: usize = 0x38;

// Ammo offsets
pub const AMMO_ASSAULT_RIFLE: usize = 0x140;
pub const AMMO_SUBMACHINE: usize = 0x138;
pub const AMMO_SNIPER: usize = 0x13C;
pub const AMMO_SHOTGUN: usize = 0x134;
pub const AMMO_PISTOL: usize = 0x12C;
pub const AMMO_GRENADE: usize = 0x144;

// Fire rate offsets
pub const FAST_FIRE_ASSAULT: usize = 0x164;
pub const FAST_FIRE_SNIPER: usize = 0x160;
pub const FAST_FIRE_SHOTGUN: usize = 0x158;

// Other player properties
pub const AUTO_SHOOT: usize = 0x204;
pub const HEALTH: usize = 0xEC;
pub const ARMOR: usize = 0xF0;
pub const PLAYER_NAME: usize = 0x205;

fn get_module_base(pid: i32, module_name: &str) -> Option<usize> {
    let maps = fs::read_to_string(format!("/proc/{}/maps", pid)).ok()?;
    for line in maps.lines() {
        if line.contains(module_name) {
            println!("Found module line: {}", line);
            let addr_str = line.split('-').next()?;
            return usize::from_str_radix(addr_str, 16).ok();
        }
    }
    None
}

unsafe fn write_i32(pid: i32, addr: usize, value: i32) -> io::Result<()> {
    let word = value as libc::c_long;
    unsafe {
        if ptrace(
            PTRACE_POKEDATA,
            pid,
            addr as *mut libc::c_void,
            word as *mut libc::c_void,
        ) == -1
        {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn attach(pid: i32) -> io::Result<()> {
    unsafe {
        if ptrace(
            PTRACE_ATTACH,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        ) == -1
        {
            return Err(io::Error::last_os_error());
        }
        waitpid(pid, std::ptr::null_mut(), 0);
    }
    Ok(())
}

fn detach(pid: i32) -> io::Result<()> {
    unsafe {
        if ptrace(
            PTRACE_DETACH,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        ) == -1
        {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn hack(pid: i32, local_player_ptr: usize) {
    loop {
        let health_address = local_player_ptr + HEALTH;
        let ammo_address = local_player_ptr + AMMO_ASSAULT_RIFLE;

        unsafe {
            write_i32(pid, health_address, 999_999).expect("Failed to write health");
            write_i32(pid, ammo_address, 999_999).expect("Failed to write ammo");
        }

        println!("Infinite health & ammo applied!");
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Target OS
    #[cfg(target_os = "linux")]
    const TARGETS: &[&str] = &["linux_64_client"];
    let targets = TARGETS;

    // Matching processes
    let mut main_pid: Option<Pid> = None;
    let mut max_cpu: f32 = 0.0;

    for (pid, process) in sys.processes() {
        let name_str = process.name().to_string_lossy();

        if targets.contains(&name_str.as_ref()) {
            println!("Found candidate: PID = {}", pid);
            println!("  Name: {:?}", process.name());
            println!("  Cmd : {:?}", process.cmd());
            println!("  CPU : {}", process.cpu_usage());
            println!("  Mem : {}", process.memory());

            if process.cpu_usage() > max_cpu {
                max_cpu = process.cpu_usage();
                main_pid = Some(*pid);
            }
        }
    }

    let main_pid = main_pid.expect("AssaultCube not found!");
    let main_pid_i32 = main_pid.as_u32() as i32;

    println!(
        "\nSelected main AssaultCube PID = {} with CPU {}",
        main_pid, max_cpu
    );

    attach(main_pid_i32).expect("Failed to attach!");
    println!("Attached!");

    let base_address = get_module_base(main_pid_i32, "linux_64_client").expect("Module not found");
    let local_player_ptr = base_address + LOCAL_PLAYER;
    hack(main_pid_i32, local_player_ptr);

    detach(main_pid_i32).expect("Failed to detach!");
    println!("Detached!");
}
