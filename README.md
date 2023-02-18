# Reverse-Proxy
Rust client for reverse proxy

# Steps to run the code
1. `cargo build`
2. `cargo run`


This should create and endpoint like Listening on http://127.0.0.1:3000
If one makes a get request to it, it should interanlly call the `https://blockstream.info/api/blocks/{block}` api to get the result 

