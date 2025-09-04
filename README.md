# Dev-header
CLI tool for generating **comment header**

## Install
Make sure you have Rust and Cargo installed. Follow the installation guide [here](https://rustup.rs/).
``` bash
$ cargo install --path .
```
## Quick Start
Some basic commands:
- List available headers:
```bash
$ header list -s
```
- Create a new header:
```bash
$ header new
```
- Get a header by name:
```bash
$ header get <name>
```
- Apply a header with input values:
```bash
$ header apply
```
For full usage and all commands, run:
```bash
$ header --help
```