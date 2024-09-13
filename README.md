cargo build --release; ./target/release/mon-fs encode --to-encode ./assets/ricky.webp
cargo build --release; ./target/release/mon-fs encode --to-encode ./assets/song.opus
cargo build --release; ./target/release/mon-fs decode --decode-to ./out/ --pc-screenshots ./assets/pic/ --python-script-path ./pc_screenshot_decoder/
