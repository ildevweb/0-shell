pub fn mkdir(args: &mut Vec<&str>) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for &dir in args.iter() {
        match std::fs::create_dir(dir) {
            Ok(_) => println!("Directory '{}' created.", dir),
            Err(e) => eprintln!("mkdir: {}: {}", dir, e),
        }
    }
}