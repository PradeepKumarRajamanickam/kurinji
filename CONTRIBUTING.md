# Code Contributions
## Code Formatting
Code needs to be formatted according to the rules in rustfmt.toml. Easiest way to achieve this is to 

run ```$ cargo +nightly fmt```

If you run into this error ```error[internal]: left behind trailing whitespace ``` 

run ```$ cargo fmt``` 

once before running the above command. This will remove trailing whitespaces
for you.

Note* You need nightly installed on you machine. 
You can use the below commands.

```$ rustup toolchain install nightly```

```$ rustup component add rustfmt --toolchain nightly ```
