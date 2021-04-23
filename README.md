# cheza_coin

Miniscule implementation of a blockchain and a currency known as cheza_coin, transactions, proof of work and a merkle root implementation. Mining rewards and difficulty can be updated on the fly.

This is built for educational purposes, so do not take it too seriously.

## Usage

```shell
cargo run
```

## Notes

Note that setting the difficulty to large numbers can potentially "break" the program, depending on your specs of course. I recommend setting the difficulty to a number between 1-3.

Also, I have only compiled this on `x86_64-apple-darwin`, which requires installation of x-code tools to compile.

```shell
xcode-select --install
```
