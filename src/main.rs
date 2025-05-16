use colored::ColoredString;
use colored::Colorize;
extern crate term;
use std::io;
use std::process;

fn main() {
    // Required to write the formatted guess over the input-line, later.
    let mut t = term::stdout().unwrap();

    // Initialise everything.
    let mut chances = 10;
    //let mut reds = 0;
    //let mut whites = 0;
    // The secret is an array of digits because that's easier later on.
    let secret = [0; 4].map(|_| rand::random_range(1..=8));

    println!("I designed a secret code, consisting out of four digits of 1-8.");
    println!("You have {chances} chances to crack the code.");
    println!(
        "Every time I will tell you how many are {} and {}.",
        "the correct colour in the correct place".red(),
        "the correct colour but in the wrong place".white(),
    );
    println!("Please input your first guess:");

    // Keep going as long as wel have "lives" left...
    while chances > 0 {
        // Take the guess from standard input.
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        // They burnt one of their "lives" with this.
        chances -= 1;

        // No trailing newline or any other funny business.
        let guess = guess.trim();
        validate_guess(&guess);

        // Apparently we need to go up one line after input..?
        t.cursor_up().unwrap();
        // YEET DAT LINE!
        t.delete_line().unwrap();

        // Print the line as "{chances}  {formatted_input}  {reds}  {whites}".
        print!("{chances}  ");
        for c in guess.chars() {
            print!("{}", colour_number(c.to_digit(9).unwrap()));
        }
        println!("  {}  {}", "2".red(), "1".white());
    } // End while

    // If we haven't quit during the while-loop, they've burnt through all of their "lives".
    // Tell them they lost and show them what the code was to ensure fairness.
    println!("\n{}", "You lost!".red());

    print!("The code was ");
    for i in secret {
        print!("{}", colour_number(i));
    }
    // "Trailing" newline because most, if not all, shells expect it.
    println!();
}

fn validate_guess(guess: &str) {
    if guess.len() != 4 {
        eprintln!("\n{}", "Please input exactly four digits!".yellow());
        process::exit(1);
    }

    // Print an error and quit if any of the characters are not 1-8.
    if guess.chars().any(|c| c < '1' || c > '8') {
        eprintln!("\n{}", "Please only input numbers 1-8.".yellow());
        process::exit(1);
    };
}

fn colour_number(c: u32) -> ColoredString {
    match c {
        1 => "(1)".on_black(),
        2 => "(2)".on_yellow(),
        3 => "(3)".on_red(),
        4 => "(4)".on_green(),
        5 => "(5)".on_blue(),
        6 => "(6)".on_truecolor(255, 128, 0),
        7 => "(7)".on_purple(),
        8 => "(8)".on_white(),
        _ => panic!(
            "{} is not a valid number, only numbers 1-8 are accepted.",
            c
        ),
    }
}
