use clap::Parser;
use std::process::{Command, exit};
use std::fs;
use std::error::Error;

/// This program clones a git project, copies it to a server, and sets up a remote origin.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The project name
    project: String,

    /// The server address
    server: String,

    /// Specify the name of the local remote.
    #[arg(short, value_name = "remote name", default_value = "origin")]
    remote: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let project = &args.project;
    let server = &args.server;
    let remote_name = &args.remote;

    if project == "." {
        eprintln!("Error: Use this tool from the parent directory.");
        exit(1);
    }
    // Clone the project repository as a bare repository
    if !run_command(&["git", "clone", "--bare", project, &format!("{}.git", project)]) {
        eprintln!("Error: Failed to clone repository.");
        exit(1);
    }

    // Securely copy the bare repository to the server
    if !run_command(&["scp", "-r", &format!("{}.git", project), &format!("{}:{}.git", server, project)]) {
        eprintln!("Error: Failed to copy repository to server.");
        exit(1);
    }

    // Remove the local bare repository
    if let Err(e) = fs::remove_dir_all(&format!("{}.git", project)) {
        eprintln!("Error: Failed to remove local repository: {}", e);
        exit(1);
    }

    // Add remote origin and push to server
    if !run_command(&["sh", "-c", &format!("cd {} && git remote add {} {}:{} && git push --set-upstream {} master", remote_name, project, server, project, remote_name)]) {
        eprintln!("Error: Failed to add remote origin or push to server.");
        exit(1);
    }

    Ok(())
}

fn run_command(cmd: &[&str]) -> bool {
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .status();

    match status {
        Ok(s) if s.success() => true,
        _ => false,
    }
}
