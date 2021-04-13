# toy-blockchain-cli

Miniscule implementation of a blockchain with an unnamed currency, transactions, proof of work and a merkle root implementation

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
