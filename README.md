# Chip-8 Emulator in Rust
As an attempt to familiarise myself with the Rust programming language and the world of emulation i've decided to dive head-first into
developing this chip-8 emulator. It is still under development and not in a complete state, audio is missing and compatibility is not
100% guaranteed, as proven by some tests in the tests/ directory failing.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license) 

## Installation
Get yourself a fresh copy of Rust [from here](https://www.rust-lang.org/tools/install)
1. Clone the repository:
```bash
 git clone https://github.com/0ro8lu/chip_8_emulator_rust.git
```

2. Build the thing:
```bash
  cargo build
 ```

## Usage
To run the project simply
```bash
  cargo run -- "path/to/chip-8/rom"
```

## Contributing
1. Fork the repository.
2. Create a new branch: `git checkout -b feature-name`.
3. Make your changes.
4. Push your branch: `git push origin feature-name`.
5. Create a pull request.

## License
This project is licensed under the [MIT License](LICENSE).
