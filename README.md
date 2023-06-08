# Sha1-Cracker

This program is a simple SHA1 hash cracker that attempts to find the original password from a given SHA1 hash and a wordlist.

## Usage
> Compiler

To compile the program, run it with the following command:
```sh
cargo build --release
```
> Run

To use the program, run it with the following command:
```sh
./target/release/sha1_cracker <wordlist.txt> <sha1_hash>
```
- **<wordlist.txt>**: The path to a text file containing a list of common passwords to check against the hash.
- **<sha1_hash>**: The SHA1 hash to crack.

## Requirements

The program requires the **sha1** and **hex** crates to be installed. You can add these dependencies to your **Cargo.toml** file:
```sh
[dependencies]
sha1 = "0.6"
hex = "0.4"
```

## How it Works

1-  The program first checks the number of command-line arguments. If the number of arguments is not exactly 3, it displays a usage message and exits.

2-  It extracts the SHA1 hash from the second argument and checks if it has a valid length of 40 characters.

3-  It opens the wordlist file specified in the first argument.

4-  It reads the wordlist file line by line and compares the SHA1 hash of each password with the provided hash.

5-  If a match is found, it prints the password and exits.

6-  If no match is found after checking all passwords in the wordlist, it displays a message indicating that the password was not found.

## Example

Suppose you have a wordlist file named **passwords.txt** containing a list of common passwords and you want to crack the SHA1 hash **5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8**. You can run the program with the following command:
```sh
sha1_cracker passwords.txt 5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8
```
If the program finds a matching password in the wordlist, it will display a message similar to:
```sh
Password found: password123
```
If no match is found, it will display:
```sh
password not found in wordlist :(
```

