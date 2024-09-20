fn main() {
    let our_args: Vec<_> = std::env::args_os().skip(1).collect();

    let status = std::process::Command::new(&our_args[0])
        .args(&our_args[1..])
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait on child process")
        .code()
        .expect("Subprocess terminated abnormally");

    std::process::exit(status);
}
