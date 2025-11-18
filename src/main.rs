use sysinfo::*;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // System info
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    //   Target OS
    #[cfg(target_os = "windows")]
    const TARGETS: &[&str] = &["ac_client.exe"];
    #[cfg(target_os = "linux")]
    const TARGETS: &[&str] = &["linux_64_client"];
    #[cfg(target_os = "macos")]
    const TARGETS: &[&str] = &["AssaultCube"];

    let targets = TARGETS;

    // Detect matching processes
    let mut main_pid: Option<Pid> = None;
    let mut max_cpu: f32 = 0.0;

    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy();

        if targets.contains(&name.as_ref()) {
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

    println!(
        "\nSelected main AssaultCube PID = {} with CPU {}",
        main_pid, max_cpu
    );

    // Attach WIP

    // Read WIP

    // Write WIP
}
