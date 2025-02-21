# lefetch
![1 Screen](https://github.com/wholos/archfetch/blob/main/lefetch.png)

How to install!?
``` bash
sudo pacman -S git rust cargo rust-analyzer
git clone https://github.com/wholos/lefetch
cd lefetch/
cargo new lefetch
sudo cp Cargo.toml lefetch/Cargo.toml
sudo cp main.rs lefetch/src/main.rs
cd lefetch/
cargo build --release
cargo install --path .
sudo cp lefetch /usr/local/bin/lefetch
```
Ready!

How to use!?
``` bash
lefetch
```
