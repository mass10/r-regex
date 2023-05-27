# r-regex
Regex CLI Utility

# Getting Started

Install

```sh
cargo install --git https://github.com/mass10/r-regex --branch main
```

Or download single binary file.

```sh
wget https://github.com/mass10/r-regex/releases/latest/download/r-regex
```

# Examples

```sh
# Shows 4567
r-regex --string "123-4567" --regex "([0-9]+)$"
```
