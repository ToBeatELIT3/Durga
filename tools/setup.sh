#!/bin/bash
echo "[*] Preparing Install"
mkdir ~/.durga
echo "[*] Starting Install..."
git clone https://github.com/ToBeatELIT3/Durga ~/.durga/Durga
cd ~/.durga/Durga
echo "[*] Buillding Durga_Conf"
sudo gcc ~/.durga/Durga/tools/durga_conf.c -o /bin/durga_conf
echo "[*] Building Durga"
cargo build --release && sudo mv target/release/durga /bin/durga
echo "[*] Done!"

