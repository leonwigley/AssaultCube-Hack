use libc::{PTRACE_ATTACH, PTRACE_DETACH, ptrace, waitpid};
use std::io;
use sysinfo::*;

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


fn infinite_ammo() {
    println!("Infinite ammo function called...");
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

    infinite_ammo();

    detach(main_pid_i32).expect("Failed to detach!");
    println!("Detached!");
}
