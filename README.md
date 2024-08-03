# Random Password Generator Application

This command-line tool generates random passwords. It allows control over various options such as the password length,
the use of uppercase, lowercase, symbols, digits, and the number of passwords to generate.

## Installation

First, clone this repository and navigate into the directory:

`git clone https://github.com/haytty/random_password.git`
`cd random_password`

Next, use Cargo to build the application:

`cargo install --path .`

After the compilation process, the executable file will be located in the `target/release/` directory.

## Usage

`random_password [OPTIONS]`

- `--length`: Specifies the length of passwords to generate. The default is 24.
- `--upper`: Includes uppercase letters in the password.
- `--lower`: Includes lowercase letters in the password.
- `--symbol`: Includes symbols in the password.
- `--digit`: Includes digits in the password.
- `--count`: Sets the number of passwords to generate. The default is 5.

`Example: ./ramdom_password`

The command above generates 10 random passwords, each 12 characters long and contains a mix of upper and lowercase
letters, symbols, and digits.
