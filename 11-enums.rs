fn perform_task() {
    println!("Process is Enabled.")
}

fn main() {
    enum Status {
        Enabled,
        Disabled,
        Paused,
        Stopped
    }

    let process_status: Status = Status::Enabled;

    match process_status {
        // Status::Enabled => println!("Process is Enabled."),
        Status::Enabled => perform_task(),
        Status::Disabled => println!("Process is Disabled."),
        Status::Paused => println!("Process is Paused."),
        Status::Stopped => println!("Process is Stopped."),
    }
}