# 🦀 Rust Project Cheat Sheet

## 🛠 Build & Run
- **Run (Debug):** `cargo run`
- **Build (Release):** `cargo build --release`
- **Check:** `cargo check`
- **Test:** `cargo test`

## 📦 Version Management
Use `make` commands to manage versions (syncs Rust, Python, Installer).

- **Check Version:** `make version-status`
- **Sync Config:** `make version-sync`
- **Bump Patch (1.0.0 -> 1.0.1):** `make version-bump-patch`
- **Bump Minor (1.0.0 -> 1.1.0):** `make version-bump-minor`
- **Set Version:** `make version-set v=1.2.3`

## 🧹 Clean
- **Clean Build:** `cargo clean`

## 📚 Documentation
- **Open Docs:** `cargo doc --open`
