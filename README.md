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

### print 4567

```sh
r-regex --string "123-4567" --regex "([0-9]+)$"
```

### print 1.0.0

```sh
r-regex --string "refs/tags/v1.0.0" --regex "/v([0-9.]+)$"
```
