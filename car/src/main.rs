const PAGE: &str = include_str!("index.html");

use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("0.0.0.0:3000").unwrap();
    for mut stream in listener.incoming().flatten() {
        println!("{:?}", stream);
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();

        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", _, "HTTP/1.1"] => {
                let response = execute_virsh_command(
                    "virsh --connect qemu:///system list --all | grep silver-nix",
                );
                send_html(&mut stream, response);
            }
            ["POST", resource, "HTTP/1.1"] => match resource {
                &"/reboot" => {
                    println!("asked for reboot");
                    let response = execute_virsh_command(
                        "virsh --connect qemu:///system reboot --domain silver-nix",
                    );

                    send_html(&mut stream, response);
                }
                &"/destroy" => {
                    println!("asked for shutdown");
                    let response = execute_virsh_command(
                        "virsh --connect qemu:///system destroy --domain silver-nix",
                    );

                    send_html(&mut stream, response);
                }
                &"/start" => {
                    println!("asked for reboot");
                    let response = execute_virsh_command(
                        "virsh --connect qemu:///system start --domain silver-nix",
                    );

                    send_html(&mut stream, response);
                }
                _ => {}
            },

            _ => {}
        }
    }
}

fn send_html(stream: &mut std::net::TcpStream, response: std::process::Output) {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream
        .write_all(b"Content-Type: text/html; charset=UTF-8\r\n")
        .unwrap();
    stream.write_all(b"\r\n").unwrap();
    stream.write_all(PAGE.as_bytes()).unwrap();
    stream.write_all(b"</br>").unwrap();
    stream
        .write_all(
            format!(
                "<span style='color:red'>{}</span></br><span style='color:green'>{}</span>",
                String::from_utf8_lossy(&response.stderr),
                String::from_utf8_lossy(&response.stdout)
            )
            .as_bytes(),
        )
        .unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap()
}

fn execute_virsh_command(command: &str) -> std::process::Output {
    std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command")
}
