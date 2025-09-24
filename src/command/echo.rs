pub fn echo(args: Vec<&str>) {
    let output = args.join(" ");
    println!("{}", output);
}
