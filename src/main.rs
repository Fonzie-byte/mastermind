extern crate term;
use colored::ColoredString;
use colored::Colorize;
use std::io;
use std::process;

fn main() {
    // Required to write the formatted guess over the input-line, later.
    let mut t = term::stdout().unwrap();

    // Initialise everything.
    let mut chances = 10;
    // The secret is an array of digits because that's easier later on.
    let secret = [0; 4].map(|_| rand::random_range(1..=8));

    println!("I designed a secret code, consisting out of four digits of 1-8.");
    println!("You have {chances} chances to crack the code.");
    println!(
        "Every time I will tell you how many are {} and {}.",
        "the correct colour in the correct place (A)".red(),
        "the correct colour but in the wrong place (B)".white(),
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
        let guess: Vec<char> = guess.trim().chars().collect();
        validate_guess(&guess);

        // Apparently we need to go up one line after input..?
        t.cursor_up().unwrap();
        // Afterwards I can YEET DAT LINE!
        t.delete_line().unwrap();

        // Print the line as "{chances}  {formatted_input}  {reds}  {whites}".
        print!("{chances}  ");
        // Reset these before every evaluation.
        let mut reds = 0;
        let mut whites = 0;

        // Evaluate!
        let mut i = 0;
        let mut secret_copy = secret.clone();
        for c in guess {
            let num = c.to_digit(9).unwrap();

            if num == secret_copy[i] {
                reds += 1;
                secret_copy[i] = 0;
            } else if secret_copy.contains(&num) {
                whites += 1;
            }

            print!("{}", colour_number(num));
            i += 1;
        } // End evaluation

        println!(
            "  {}  {}",
            format!("A{}", reds).red(),
            format!("B{}", whites).white()
        );

        if reds == 4 {
            println!("\n{}", "You cracked the code!".green());
            process::exit(0);
        }
    } // End while

    // If we haven't quit during the while-loop, they've burnt through all of their "lives".
    // Tell them they lost and show them what the code was to ensure fairness.
    println!("\n{}", "You lose!".red());

    print!("The code was ");
    for i in secret {
        print!("{}", colour_number(i));
    }

    // "Trailing" newline because most, if not all, shells expect it.
    println!();
}

/// Checks if the guess input is valid.
fn validate_guess(guess: &[char]) {
    if guess.len() != 4 {
        eprintln!("\n{}", "Please input exactly four digits!".yellow());
        process::exit(1);
    }

    // Print an error and quit if any of the characters are not 1-8.
    if guess.iter().any(|c| *c < '1' || *c > '8') {
        eprintln!("\n{}", "Please only input numbers 1-8.".yellow());
        process::exit(1);
    };
}

/// Colours the number if it's one of the valid inputs.
///
/// # Examples
///
/// ```
/// assert_eq!("(1)".on_black(), colour_number(1));
/// assert_eq!("(2)".on_yellow(), colour_number(2));
/// assert_eq!("(3)".on_red(), colour_number(3));
/// assert_eq!("(4)".on_green(), colour_number(4));
/// assert_eq!("(5)".on_blue(), colour_number(5));
/// assert_eq!("(6)".on_truecolor(255, 128, 0), colour_number(6));
/// assert_eq!("(7)".on_purple(), colour_number(7));
/// assert_eq!("(8)".on_white(), colour_number(8));
///
/// #[should_panic]
/// colour_number(0);
/// #[should_panic]
/// colour_number(9);
/// ```
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
