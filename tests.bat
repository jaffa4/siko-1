

call ./build

mkdir -p comp
mkdir -p rust_comp

cd siko_tester
cargo run -- D:\siko\siko.exe ../std ../comp ../rust_comp ../tests/success/ ../tests/fail/ win
