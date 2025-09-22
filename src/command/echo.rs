pub fn echo(args: Vec<&str>) {
    let result = args.join(" ");  // Join the arguments with spaces
    println!("{}", result);  // Print the result
}
