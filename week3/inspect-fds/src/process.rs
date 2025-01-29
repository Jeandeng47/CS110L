use crate::open_file::OpenFile;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub pid: usize,
    pub ppid: usize,
    pub command: String,
}

impl Process {
    // Constructor for process
    pub fn new(pid: usize, ppid: usize, command: String) -> Process {
        Process { pid, ppid, command }
    }

    pub fn print(&self) {
        // MILESTONE 2
        // let cmd_name = self.command.split_whitespace().next().unwrap_or("Unknown");
        // println!("=======  \"{}\" (pid: {} ppid: {}) =======", 
        // cmd_name, self.pid, self.ppid);

        // MILESTONE 3
        if let Some(fds) = self.list_fds() {
            println!("Process {} has fds: {:?}", self.pid, fds);
        } else {
            println!("Process {} has no fds", self.pid);
        }
    }

    /// This function returns a list of file descriptor numbers for this Process, if that
    /// information is available (it will return None if the information is unavailable). The
    /// information will commonly be unavailable if the process has exited. (Zombie processes
    /// still have a pid, but their resources have already been freed, including the file
    /// descriptor table.)
  
    pub fn list_fds(&self) -> Option<Vec<usize>> {

        // Handle zombie process
        let status_path = format!("/proc/{}/status", self.pid);
        let status = std::fs::read_to_string(&status_path).ok()?;

        if status.contains("State:\tZ") {
            return None;
        }

        let path = format!("/proc/{}/fd", self.pid);

        // fs::read_dir() -> Result<ReadDir, std::io::Error>
        // ReadDir is an iterator over DirEntry
        let entries = fs::read_dir(&path).ok()?;

        // iterate the entries and parse them
        let mut fds = Vec::new();
        for entry in entries {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            if let Ok(fd) = file_name.to_string_lossy().parse::<usize>() {
                fds.push(fd);
            }
        }
        Some(fds)
    }

    /// This function returns a list of (fdnumber, OpenFile) tuples, if file descriptor
    /// information is available (it returns None otherwise). The information is commonly
    /// unavailable if the process has already exited.
    #[allow(unused)] // TODO: delete this line for Milestone 4
    pub fn list_open_files(&self) -> Option<Vec<(usize, OpenFile)>> {
        let mut open_files = vec![];
        for fd in self.list_fds()? {
            open_files.push((fd, OpenFile::from_fd(self.pid, fd)?));
        }
        Some(open_files)
    }
}

// Note:
// 1. Solution 1: close FDs
// To pass the test when running this test under IDE terminal
// Close the unnecessary file descriptors inherited from shell 
// For example, the fds shows [1, 2, 4, 5, 19, 20, 21],
// we need to close unnecessary fds: exec 19>&- 20>&- 21>&-
// and then execute: cargo test list_fds
//
// 2. Solution 2: run under local terminal
//

#[cfg(test)]
mod test {
    use crate::ps_utils;
    use std::process::{Child, Command};

    fn start_c_program(program: &str) -> Child {
        Command::new(program)
            .spawn()
            .expect(&format!("Could not find {}. Have you run make?", program))
    }

    #[test]
    fn test_list_fds() {
        let mut test_subprocess = start_c_program("./multi_pipe_test");
        let process = ps_utils::get_target("multi_pipe_test").unwrap().unwrap();

        assert_eq!(
            process
                .list_fds()
                .expect("Expected list_fds to find file descriptors, but it returned None"),
            vec![0, 1, 2, 4, 5]
        );
        let _ = test_subprocess.kill();
    }

    #[test]
    fn test_list_fds_zombie() {
        let mut test_subprocess = start_c_program("./nothing");

        let process = ps_utils::get_target("nothing").unwrap().unwrap();
        // Read process state from /proc/{pid}/status
        let status_path = format!("/proc/{}/status", process.pid);
        let status = std::fs::read_to_string(&status_path).unwrap_or_default();

        // Print process state for debugging
        println!("Process State:\n{}", status);

        // Check if process is marked as zombie (State: Z)
        if status.contains("State:\tZ") {
            assert!(
                process.list_fds().is_none(),
                "I Expected list_fds to return None for a zombie process"
            );
        } else {
            panic!("Process was not a zombie when checked");
        }

        // assert!(
        //     process.list_fds().is_none(),
        //     "Expected list_fds to return None for a zombie process"
        // );
        let _ = test_subprocess.kill();
    }
}
