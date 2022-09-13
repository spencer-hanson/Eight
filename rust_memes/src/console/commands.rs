pub fn wait_for_commands() {
    let mut done = false;
    while !done {
        let mut line = String::new();
        println!("Waiting for Input>");
        let _byt = std::io::stdin().read_line(&mut line).unwrap();
        line = String::from(line.trim());

        if line == "quit" || line == "exit" {
            println!("Exiting..");
            done = true;
        }
    }
}
