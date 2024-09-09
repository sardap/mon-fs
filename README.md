cargo build --release; ./target/release/mon-fs encode --to-encode ./assets/p.webp
cargo build --release; ./target/release/mon-fs decode --decode-to ./out/ --pc-screenshots ./assets/pic/ --python-script-path ./pc_screenshot_decoder/
