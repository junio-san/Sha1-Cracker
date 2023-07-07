# Cryptea

![](https://imgur.com/a/l5bghhp)

Cryptea is a SHA1 hash cracker that attempts to find the original password from a given SHA1 hash and a wordlist.  
Also can encrypt messages using the `parse` keyword

## Usage
### Compiling

First you will need to clone this repository on your machine
```sh
git clone --depth=1 https://gitlab.com/juniosantos/Cryptea.git 
```

To compile by now you will need Rustc and cargo setup.
```sh
cargo build --release --locked
```

You may want to execute `cd ./target/release/` to run cryptea easier

### Executing

Cryptea has 3 options: 
- `find`
- - You need to pass 2 arguments: 
- - - Path where the contains the text
- - - Hash value
- - - optional: `--type` to select the hash function (default: sha256)
- - - Example: `./cryptea find ./wordlist.txt "3447a6779143e2dd8cbaad5922ec6afac4a0276a17544f8a3285a27722a1c943"`

- `parse`
- - Pass message to encrypt
- - - Also accepts `--type` flag
- - - Example: `./cryptea parse "hello" --type sha512`

- `example`
- - Runs a setup example to demonstrate how cryptea works
- - Example: `./cryptea example`

