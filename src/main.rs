mod functions;
use functions::format_and_trim;
use std::env::var;
use std::io::{Read, stdin, stdout, Write};
use std::path::Path;

fn add(raw_config_dir: &str, program: String) {
    let mut cfgpath = String::new();
    print!("\nPlease input the path to the config: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut cfgpath)
        .expect("Failed to read user input");
    let raw_config_path = format_and_trim(raw_config_dir, &program);
    let config_path = Path::new(&raw_config_path);
    if config_path.exists() {
        let mut config_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(config_path)
            .unwrap();
        write!(config_file, "{}", cfgpath).unwrap();
    } else {
        let mut config_file = std::fs::File::create(config_path).unwrap();
        config_file
            .write_all(cfgpath.as_bytes())
            .unwrap();
    }
}

fn modify(raw_config_dir: &str, program: String) {
    print!("We will attempt to use $EDITOR first, however please specify a backup: ");
    stdout().flush().unwrap();
    let mut backup_editor = String::new();
    stdin()
        .read_line(&mut backup_editor)
        .expect("Failed to read user input for the backup editor [crate::modify()])");
    let editor = var("EDITOR").unwrap_or_else(|_| backup_editor);
    let mut terminal = String::new();
    if editor.contains("vim") || editor.contains("nano") {
        print!("You appear to be using an a TUI editor, please input the terminal to be used: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut terminal)
            .expect("Failed to read user input for the terminal [crate::modify()]");
        terminal = terminal.trim().to_string();
    }
    let mut cfgpath = String::new();
    let cfgfilepath = format_and_trim(raw_config_dir, &program);
    let mut cfgfile = std::fs::File::open(cfgfilepath.trim())
        .expect("Unable to read the file [crate::modify()]");
    cfgfile.read_to_string(&mut cfgpath).expect("Could not read file [crate::modify()]");
    cfgpath = cfgpath.replace("\n", " ").trim().to_string();
    if !terminal.is_empty() {
        let command = format!("{} {}", editor, cfgpath);
        std::process::Command::new(terminal)
            .args(&["-e", command.as_str()])
            .output()
            .expect("Could not run command, a likely culprit is that your terminal isn't supported [crate::modify()]");
    } else {
        println!("Please note that if nothing happens, you may be using a TUI editor that I didn't detect");
        std::process::Command::new(editor)
            .arg(cfgpath)
            .output()
            .expect("Could not run command [crate::modify()]");
    }
}

fn main() {
    let user = var("USER").unwrap();
    let home = if user != "root" {
        var("HOME").unwrap()
    } else {
        // Most likely ran through sudo if the user is root.
        // TODO: Add more cases, for when sudo is not in use.
        let real_user = var("SUDO_USER").unwrap();
        ["/home/", real_user.as_str()].concat()
    };

    let raw_config_dir = format!("{}/.config/cfg", home);
    let config_dir = Path::new(&raw_config_dir);
    functions::ensure_directory(config_dir);

    let mut program = String::new();
    print!("Please input the name of the program: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut program)
        .expect("Failed to read user input for the program [crate::main()]");
    let args = std::env::args().collect::<Vec<String>>();
    let operation = &args[1];
    match operation.as_str() {
        "add" => add(&raw_config_dir, program),
        "rem" => println!("Used rem!"),
        "mod" => modify(&raw_config_dir, program),
        _ => { 
            println!("Support operations are add, rem, and mod.");
            std::process::exit(1);
        }
    }
}
