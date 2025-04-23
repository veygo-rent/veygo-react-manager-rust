use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tokio::time;

const REPO_URL: &str = "https://github.com/veygo-rent/veygo-react.git";
const CLONE_DIR: &str = "target/veygo-react";

fn get_commit_id() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(CLONE_DIR)
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn clone_or_pull_repo() {
    if Path::new(CLONE_DIR).exists() {
        let _ = Command::new("git")
            .arg("pull")
            .arg("-q")
            .current_dir(CLONE_DIR)
            .status();
    } else {
        let _ = Command::new("git")
            .arg("clone")
            .arg(REPO_URL)
            .arg(CLONE_DIR)
            .status();
    }
}

#[tokio::main]
async fn main() {
    clone_or_pull_repo();
    // Run `npm install`
    let _ = Command::new("npm")
        .arg("install")
        .current_dir(CLONE_DIR)
        .status();

    // Run `npm run build`
    let _ = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(CLONE_DIR)
        .status();
    
    let mut current_commit = get_commit_id().unwrap_or_default();
    let monitor_handle = {
        tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_secs(60)).await;
                clone_or_pull_repo();
                if let Some(new_commit) = get_commit_id() {
                    if new_commit != current_commit {
                        println!("New commit {} found. Rebuilding...", new_commit);
                        current_commit = new_commit;

                        // Run `npm install`
                        let _ = Command::new("npm")
                            .arg("install")
                            .current_dir(CLONE_DIR)
                            .status();

                        // Run `npm run build`
                        let _ = Command::new("npm")
                            .arg("run")
                            .arg("build")
                            .current_dir(CLONE_DIR)
                            .status();
                    }
                }
            }
        })
    };

    monitor_handle.await.unwrap();
}
