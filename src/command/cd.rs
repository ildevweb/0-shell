pub fn cd(args: &[&str]) {
    use std::env;

    if args.is_empty() {
        eprintln!("cd: missing argument");
        return;
    }

    let target = args[0];
    match env::set_current_dir(target) {
        Ok(_) => (),
        Err(e) => eprintln!("cd: {}: {}", target, e),
    }
}