// Import the std::process module to use Command
use std::{path::{PathBuf, Path}, process::Command};

use crate::parser::scan_dir;

// Define a custom error type that wraps a String message
#[derive(Debug)]
pub struct DockerError(String);

// Implement the std::error::Error trait for DockerError
impl std::error::Error for DockerError {}

// Implement the std::fmt::Display trait for DockerError
impl std::fmt::Display for DockerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Docker error: {}", self.0)
    }
}

// Define a function that takes a docker image name as a parameter
// and returns a result of either a vector of filenames or a DockerError
pub async fn list_files_in_docker_image(image: &str, path: PathBuf) -> Result<(), DockerError> {
    // Create a Command object to run docker commands
    let mut cmd = Command::new("docker");

    // Use the "create" subcommand to create a container from the image
    // without starting it
    cmd.arg("create").arg(image);

    // Execute the command and get the output
    let output = cmd.output().map_err(|e| DockerError(e.to_string()))?;

    // Check if the command was successful
    if !output.status.success() {
        // Return an error with the command's stderr
        return Err(DockerError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Get the container ID from the output
    let container_id = String::from_utf8(output.stdout)
        .map_err(|e| DockerError(e.to_string()))?
        .trim()
        .to_string();

    // Create another Command object to run docker commands
    let mut cmd = Command::new("docker");
    let cmd = cmd.current_dir(".");

    // Create a tmp folder to keep our docker-files
    create_tmp_folder(".")
    .expect("Could not create a temporary folder for the docker files. Try creating it yourself:\n./tmp/docker-files\n");

    // Use the "cp" subcommand to copy all files from the container
    // to a temporary directory on the host
    cmd.arg("cp")
        .arg(format!(
            "{}:/{}",
            container_id,
            path.to_str().expect("Path contains non-unicode characters")
        ))
        .arg("./tmp/docker-files");

    // Execute the command and get the output
    let output = cmd.output().map_err(|e| DockerError(e.to_string()))?;

    // Check if the command was successful
    if !output.status.success() {
        // Return an error with the command's stderr
        return Err(DockerError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    scan_dir(Path::new("./tmp/docker-files")).await;
    cleanup().map_err(|e| DockerError(e.to_string()) )?;

    // docker stop
    let mut cmd = Command::new("docker");
    cmd.arg("stop")
        .arg(container_id.clone());

    // Execute the command and get the output
    let _output = cmd.output().map_err(|e| DockerError(e.to_string()))?;

    // docker remove
    let mut cmd = Command::new("docker");
    cmd.arg("rm")
        .arg(container_id);

    // Execute the command and get the output
    let _output = cmd.output().map_err(|e| DockerError(e.to_string()))?;
    Ok(())
   

    // // Create another Command object to run shell commands
    // let mut cmd = Command::new("sh");

    // // Use the "-c" argument to run a shell command that lists all files
    // // in the temporary directory and removes the directory prefix
    // cmd.arg("-c")
    //     .arg(format!("cd ./tmp/docker-files/{} && ls -F", path.to_str().unwrap()));

    // // Execute the command and get the output
    // let output = cmd.output().map_err(|e| DockerError(e.to_string()))?;

    // // Check if the command was successful
    // if !output.status.success() {
    //     // Return an error with the command's stderr
    //     return Err(DockerError(
    //         String::from_utf8_lossy(&output.stderr).to_string(),
    //     ));
    // }

    // // Get the filenames from the output as a vector of strings
    // let filenames = String::from_utf8(output.stdout)
    //     .map_err(|e| DockerError(e.to_string()))?
    //     .lines()
    //     .map(|s| s.to_string())
    //     .collect();

    // // Return the filenames vector as Ok value
    // Ok(filenames)
}

fn create_tmp_folder(path: &str) -> std::io::Result<()> {
    let tmp_path = format!("{}/tmp/docker-files", path);
    std::fs::create_dir_all(tmp_path)?;
    Ok(())
}

fn cleanup() -> Result<(), std::io::Error> {
    std::fs::remove_dir_all("./tmp/docker-files")
}
