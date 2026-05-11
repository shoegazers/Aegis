use sysinfo::{ProcessRefreshKind, RefreshKind, Signal, System};

pub fn find_and_kill_process(target_name: &str) {
    let refresh_kind = RefreshKind::nothing().with_processes(ProcessRefreshKind::everything());

    let mut sys = System::new_with_specifics(refresh_kind);

    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for process in sys.processes().values() {
        let name = process.name().to_string_lossy();

        if name.eq_ignore_ascii_case(target_name) {
            println!("Found {} (PID: {}). Killing...", name, process.pid());

            // Try graceful kill first
            if !process.kill_with(Signal::Kill).unwrap_or(false) {
                println!("Failed to kill process {}", name);
            }
        }
    }
}
