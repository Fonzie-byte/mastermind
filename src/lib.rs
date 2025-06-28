use colored::{ColoredString, Colorize};
use std::io::{Read, Write, stdin, stdout};
use std::{io, panic};

/// Explains the game, returns how many chances the player has, and generates and returns the secret.
///
/// # Example
/// ```
/// use the_1a2b_game::setup;
///
/// let (mut chances, secret) = setup(15);
/// assert_eq!(chances, 15);
/// assert_eq!(secret.len(), 4);
/// ```
pub fn setup(chances: u32) -> (u32, [u32; 4]) {
    println!("I devised a secret code, consisting out of four digits of 1-8.");
    println!("You have {chances} chances to crack the code.");
    println!(
        "Every time I will tell you how many are {} and {}.",
        "the correct colour in the correct place (A)".red(),
        "the correct colour but in the wrong place (B)".white(),
    );
    println!("Please input your first guess:");

    (
        // Return the amount of chances for deduplication's sake
        chances,
        // Generate and return a secret.
        [0; 4].map(|_| rand::random_range(1..=8)),
    )
}

/// Parses an inputted guess from the terminal and returns it.
pub fn parse_guess() -> [u32; 4] {
    // Keep trying until `validate_guess` gives no errors.
    loop {
        // We need it as a "container", first.
        let mut guess = String::new();

        // Keep trying until `read_line` gives no errors.
        loop {
            // This reads it from CLI input.
            match io::stdin().read_line(&mut guess) {
                Ok(_) => break,
                Err(error) => eprintln!("Error reading your input: {error}\nPlease try again."),
            }
        }

        // No trailing newline or any other funny business.
        let guess: Vec<char> = guess.trim().chars().collect();

        // Validate and return.
        let guess = validate_guess(&guess);
        if guess == [9, 9, 9, 9] {
            println!("What is your guess?");
        } else {
            return guess;
        }
    }
}

/// Checks if the guess input is valid, by checking its length and contents.
fn validate_guess(guess: &[char]) -> [u32; 4] {
    if guess.len() != 4 {
        eprintln!("\n{}", "Please input exactly four digits!".yellow());

        // Return something impossible for me to figure out something went wrong.
        // There has to be a better way to do this.
        return [9, 9, 9, 9];
    }

    // Print an error and quit if any of the characters are not 1-8.
    if guess.iter().any(|c| *c < '1' || *c > '8') {
        eprintln!("\n{}", "Please only input numbers 1-8.".yellow());

        return [9, 9, 9, 9];
    };

    // Convert the slice to [u32; 4] and return it.
    let mut array = [0; 4];
    for (i, &c) in guess.iter().enumerate() {
        array[i] = c.to_digit(9).unwrap();
    }

    array
}

/// Evaluates the guess against the secret
///
/// # Sanity checks
/// ```
/// use the_1a2b_game::evaluate;
///
/// let secret = [1, 2, 3, 4];
/// let guess = [4, 4, 4, 4];
/// assert_eq!(evaluate(secret, guess), (1, 0), "Test for 1234 failed.");
///
/// let secret = [2, 2, 2, 2];
/// let guess = [1, 2, 3, 4];
/// assert_eq!(evaluate(secret, guess), (1, 0), "Test for 2222 failed.");
///
/// let secret = [6, 8, 6, 3];
/// let guess = [3, 3, 3, 3];
/// assert_eq!(evaluate(secret, guess), (1, 0), "Test for 6863 failed.");
///
/// let secret = [6, 8, 6, 3];
/// let guess = [6, 6, 3, 3];
/// assert_eq!(evaluate(secret, guess), (2, 1), "Second test for 6863 failed.");
///
/// let secret = [1, 8, 4, 4];
/// let guess = [7, 8, 1, 1];
/// assert_eq!(evaluate(secret, guess), (1, 1), "Test for 1844 failed.");
///
/// let secret = [8, 1, 8, 5];
/// let guess = [8, 1, 1, 5];
/// assert_eq!(evaluate(secret, guess), (3, 0), "Test for 8185 failed.");
/// ```
pub fn evaluate(secret: [u32; 4], guess: [u32; 4]) -> (u32, u32) {
    let mut secret_copy = secret;
    let mut guess_copy = guess;

    // First check for As and remove them so they won't become Bs, too.
    let mut a = 0;
    for (i, num) in guess_copy.into_iter().enumerate() {
        // This is also a good spot to print their guess in a pretty format!
        print!("{}", colour_number(num));

        if num == secret_copy[i] {
            a += 1;
            secret_copy[i] = 0;
            guess_copy[i] = 9;
        }
    }

    // Secondly, check for Bs and remove them so they won't be counted double.
    let mut b = 0;
    for (i, num) in guess_copy.into_iter().enumerate() {
        if let Some(pos) = secret_copy.iter().position(|&x| x == num) {
            b += 1;
            secret_copy[pos] = 0;
            guess_copy[i] = 9;
        }
    }

    (a, b)
}

/// Colours the number if it's one of the valid inputs.
///
/// # Examples
///
/// ```
/// use colored::Colorize;
/// use the_1a2b_game::colour_number;
///
/// assert_eq!("(1)".on_black(), colour_number(1));
/// assert_eq!("(2)".on_yellow(), colour_number(2));
/// assert_eq!("(3)".on_red(), colour_number(3));
/// assert_eq!("(4)".on_green(), colour_number(4));
/// assert_eq!("(5)".on_blue(), colour_number(5));
/// assert_eq!("(6)".on_truecolor(255, 128, 0), colour_number(6));
/// assert_eq!("(7)".on_purple(), colour_number(7));
/// assert_eq!("(8)".on_white(), colour_number(8));
///
/// // Can't pass <1 || >8
/// let result = std::panic::catch_unwind(|| colour_number(0));
/// assert!(result.is_err());
/// let result = std::panic::catch_unwind(|| colour_number(9));
/// assert!(result.is_err());
/// ```
pub fn colour_number(c: u32) -> ColoredString {
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

/// Waits for the player to press the Enter/Return-key.
/// Credit goes to [u/K900_](https://old.reddit.com/r/rust/comments/8tfyof/noob_question_pause/e177530/)
pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to quit...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validate_guess() {
        use super::*;

        // First the happy flow
        let guess = ['1', '2', '3', '4'];
        validate_guess(&guess);

        let long_guess = ['1', '2', '3', '4', '5'];
        assert_eq!(validate_guess(&long_guess), [9, 9, 9, 9]);

        let short_guess = ['1', '2', '3'];
        assert_eq!(validate_guess(&short_guess), [9, 9, 9, 9]);

        let illegal_character_guess = ['1', '2', '3', '🦀'];
        assert_eq!(validate_guess(&illegal_character_guess), [9, 9, 9, 9]);
    }
}
