use std::env;

mod open_file;
mod process;
mod ps_utils;

use ps_utils::get_target; // import get_target

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <name or pid of target>", args[0]);
        std::process::exit(1);
    }
    
    let target = &args[1];
    match get_target(target) {
        // If encounter an error, presents error msg and exits
        Err(err) => {
            eprintln!("Error occurred while searching for the target process: {}", err);
            std::process::exit(1);
        }

        // If no matching process, exit
        Ok(None) => {
            eprintln!("No matching process found for target: {}", target);
            std::process::exit(1);
        }

        // If a process is found, print its PID
        Ok(Some(process)) => {
            process.print();
            let child_processes = 
                ps_utils::get_child_processes(process.pid)
                .expect("Failed to get child processes.");
            for child in child_processes {
                child.print();
            }
        }
    }
    
}

// $ cargo run bash
// Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
// Running `target/debug/inspect-fds bash`
// Found process with PID: 779

// $ cargo run non-exist
// Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
// Running `target/debug/inspect-fds non-exist`
// No matching process found for target: non-exist

#[cfg(test)]
mod test {
    use std::process::{Child, Command};

    fn start_c_program(program: &str) -> Child {
        Command::new(program)
            .spawn()
            .expect(&format!("Could not find {}. Have you run make?", program))
    }

    #[test]
    fn test_exit_status_valid_target() {
        let mut subprocess = start_c_program("./multi_pipe_test");
        assert_eq!(
            Command::new("./target/debug/inspect-fds")
                .args(&[&subprocess.id().to_string()])
                .status()
                .expect("Could not find target/debug/inspect-fds. Is the binary compiled?")
                .code()
                .expect("Program was unexpectedly terminated by a signal"),
            0,
            "We expected the program to exit normally, but it didn't."
        );
        let _ = subprocess.kill();
    }

    #[test]
    fn test_exit_status_invalid_target() {
        assert_eq!(
            Command::new("./target/debug/inspect-fds")
                .args(&["./nonexistent"])
                .status()
                .expect("Could not find target/debug/inspect-fds. Is the binary compiled?")
                .code()
                .expect("Program was unexpectedly terminated by a signal"),
            1,
            "Program exited with unexpected return code. Make sure you handle the case where \
            ps_utils::get_target returns None and print an error message and return status \
            1."
        );
    }
}
