# Brute-Force-Rust
This Program is a brute-force program to guess a password.
This program is first created to compare between the speed of brute-forcing in Rust programming language and other programming languages.

## How To Use
To test a password, define the allowed characters, by modifying the `allowed_chars` constant, then change the `password` variable and enter the password you want to test.
The maximum length of the password depends on another constant `MAX_LENGTH` 

## IMPORTANT
Run the program with the `--release` flag to get the best performance, example:
```cargo run --release```
