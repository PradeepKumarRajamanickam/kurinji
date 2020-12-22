# Code Contributions
1. Fork it!
2. Create your feature branch: git checkout -b my-new-feature
3. Test it: cargo test
4. Format it: cargo +nightly fmt
5. Commit your changes: git commit -am 'Add some feature'
6. Push to the branch: git push origin my-new-feature
7. Submit a pull request

## Know Issues
Q. ```error[internal]: left behind trailing whitespace ``` when running
```cargo +nightly fmt```

A. Run ```$ cargo fmt``` once before running the above command. This will remove trailing whitespaces
for you. Note* You need nightly installed on you machine. 
You can use the below commands.

```$ rustup toolchain install nightly```

```$ rustup component add rustfmt --toolchain nightly ```


Credit: copied gist from [https://github.com/rust-lang/rustup/blob/master/CONTRIBUTING.md] :)
