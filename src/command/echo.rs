pub fn echo(args: Vec<&str>) {
    let output = args.join(" ");
    println!("{}", output);
}

pub fn echo_command(args: Vec<&str>) {
    echo(args);
}