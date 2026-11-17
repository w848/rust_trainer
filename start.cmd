@echo off
rem Double-click to start Rust Trainer: builds it if needed and opens the browser.
cd /d "%~dp0"
set PATH=%USERPROFILE%\.cargo\bin;%PATH%
cargo build --release --quiet || (pause & exit /b 1)
target\release\rust_trainer.exe
