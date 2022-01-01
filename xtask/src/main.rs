use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::mpsc::channel;

fn main() {
    let task = std::env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("serve") => serve(),
        _ => help()
    }
}

fn help() {
    eprintln!(
        "{}",
        "
Tasks:

serve           compiles and serves project on a development serve
        ".trim()
    );
}

fn serve() {
    let mut spawn_server = spawn_serve_server();
    let mut spawn_client = spawn_serve_client();
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal handler"))
        .expect("Could not set Ctrl-C handler");
    rx.recv().expect("Could not receive from channel.");
    spawn_client.kill().expect("Could not kill trunk");
    spawn_server.kill().expect("Could not kill trunk");
}

fn spawn_serve_client() -> Child {
    let mut command = Command::new("trunk");
    command.current_dir(project_root())
        .arg("serve")
        .arg("--port 8080")
        .arg("--dist")
        .arg(target_dist_dir())
        .arg("--")
        .arg(project_root().join("client/src/index.html"))
        .spawn()
        .expect("Failed serve_client")
}

fn spawn_serve_server() -> Child {
    let mut command = Command::new("cargo");
    command.current_dir(project_root())
        .arg("run")
        .arg("-p server")
        .spawn()
        .expect("Failed serve_server")
}


fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn target_dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

