@echo off
chcp 65001 >nul
title Excel Splitter Tauri - Build Script
echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║     Excel Splitter Tauri - Windows Build Script              ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

:: Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed!
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    pause
    exit /b 1
)

:: Check if Tauri CLI is installed
where cargo-tauri >nul 2>nul
if %errorlevel% neq 0 (
    echo [INFO] Installing Tauri CLI...
    cargo install tauri-cli
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to install Tauri CLI!
        pause
        exit /b 1
    )
)

echo [INFO] Rust version:
cargo --version
echo.
echo [INFO] Tauri CLI version:
cargo tauri --version
echo.

:: Clean previous builds
echo [INFO] Cleaning previous builds...
cargo clean

:: Build release version
echo.
echo [INFO] Building Tauri release version...
echo [INFO] This may take 10-30 minutes for the first time...
echo.
cargo tauri build

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Build failed!
    echo.
    echo Common issues:
    echo 1. Make sure Visual Studio 2022 or Build Tools is installed
    echo 2. Make sure "Desktop development with C++" workload is selected
    echo 3. Make sure WebView2 is installed (Windows 10/11 has it built-in)
    echo.
    pause
    exit /b 1
)

echo.
echo [INFO] Build successful!
echo.

:: Show file info
echo [INFO] Build output:
echo.
dir target\release\excel-splitter.exe /q
echo.
dir target\release\bundle\msi\*.msi /q 2>nul

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Build Complete!                             ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
echo Output files:
echo   EXE: target\release\excel-splitter.exe
echo   MSI: target\release\bundle\msi\
echo.
echo You can now distribute this application!
echo No Rust or other dependencies required on target computers.
echo.

:: Create distribution folder
echo [INFO] Creating distribution package...
if not exist "dist" mkdir dist
copy target\release\excel-splitter.exe dist\
copy README_Tauri.md dist\
if exist target\release\bundle\msi\*.msi (
    copy target\release\bundle\msi\*.msi dist\
)

echo.
echo Distribution package created in: dist\
echo.

pause
