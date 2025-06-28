use colored::Colorize;
use std::process;
use the_1a2b_game::*;

fn main() {
    // Required to write the formatted guess over the input-line, later.
    let mut t = term::stdout().unwrap();

    let (mut chances, secret) = setup(10);

    // Keep going as long as wel have "lives" left...
    while chances > 0 {
        let guess = parse_guess();

        // They burnt one of their "lives" by guessing.
        chances -= 1;

        // Apparently we need to go up one line after input..?
        t.cursor_up().unwrap();
        // Afterwards we can YEET DAT LINE!
        t.delete_line().unwrap();

        // Print the line as "{chances}  {formatted_input}  {As}  {Bs}".
        print!("{chances} ");

        let (a, b) = evaluate(secret, guess);

        println!(
            "  {}  {}",
            format!("A{}", a).red(),
            format!("B{}", b).white(),
        );

        // If all four are completely correct, the player has won!
        if a == 4 {
            println!("\n{}", "You cracked the code!".green());
            process::exit(0);
        }
    } // End while

    // If we haven't quit during the while-loop, they've burnt through all of their "lives".
    // Tell them they lost...
    println!("\n{}", "You lose!".red());

    // ...and show them what the code was to ensure fairness.
    print!("The code was ");
    for i in secret {
        print!("{}", colour_number(i));
    }

    // "Trailing" newline because most, if not all, shells expect it.
    println!();
}
