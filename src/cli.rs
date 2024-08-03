use clap::Parser;
use crate::password;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    author = "haytty",
    about = "Generates random passwords.",
    long_about = "This program generates random passwords with the specified options."
)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 24,
        help = "Sets the length of the passwords to generate."
    )]
    length: u8,

    #[arg(long, help = "Includes uppercase letters in the password.")]
    upper: bool,

    #[arg(long, help = "Includes lowercase letters in the password.")]
    lower: bool,

    #[arg(long, help = "Includes symbols in the password.")]
    symbol: bool,

    #[arg(long, help = "Includes digits in the password.")]
    digit: bool,

    #[arg(short, long, default_value_t = 5, help = "Sets the number of passwords to generate.")]
    count: u8,
}

pub fn run() {
    let args = Args::parse();

    for _i in 0..args.count {
        let password = password::random_password(
            args.length,
            args.upper,
            args.lower,
            args.symbol,
            args.digit,
        );

        println!("{}", password);
    }
}