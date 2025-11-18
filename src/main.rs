use sysinfo::*;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut found = false;

    // System
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // RAM and swap
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // AssaultCube PID (TODO: per os)
    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy();

        if name == "ac_client" || name == "AssaultCube" {
            found = true;

            println!("Found AssaultCube: PID = {}", pid);
            println!("  Name: {:?}", process.name());
            println!("  Cmd : {:?}", process.cmd());
            println!("  CPU : {}", process.cpu_usage());
            println!("  Mem : {}", process.memory());
        }
    }

    assert!(found, "AssaultCube not found!");

    // TODO:
    // Attach to the process safely
    // Memory reading/writing
}
