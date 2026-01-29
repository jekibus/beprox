cd src-tauri && cargo check && cd ..
sudo bun tauri dev
sudo chown -R $(whoami) .
bun tauri build
open src-tauri/target/release/bundle/macos/BeProx.app