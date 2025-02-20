# archfetch
![1 Screen](https://github.com/wholos/archfetch/blob/main/archfetch2.png)

How to install!?
``` bash
sudo pacman -S nerd-fonts git rust cargo rust-analyzer
git clone https://github.com/wholos/archfetch
cd archfetch/
cargo new archfetch
sudo cp Cargo.toml archfetch/Cargo.toml
sudo cp main.rs archfetch/src/main.rs
cd archfetch/
cargo build --release
cargo install --path .
sudo cp archfetch /usr/local/bin/archfetch
```
Ready!

How to use!?
``` bash
archfetch
```
