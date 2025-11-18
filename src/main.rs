use sysinfo::*;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // system
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // RAM and swap
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // AssaultCube PID
    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().to_lowercase();

        if name.contains("cube") {
            println!("Found AssaultCube: PID = {}", pid);
            println!("  Name: {:?}", process.name());
            println!("  Cmd : {:?}", process.cmd());
            println!("  CPU : {}", process.cpu_usage());
            println!("  Mem : {}", process.memory());
        }
    }
}
