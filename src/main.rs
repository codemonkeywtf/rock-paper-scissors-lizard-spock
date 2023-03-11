use rand::Rng;
use std::io;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Action {
    Kryten,
    Rimmer,
    Cat,
    Dave,
    Holy,
}

impl std::str::FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Action,()> {
        match s {
            "kryten" => Ok(Action::Kryten),
            "rimmer" => Ok(Action::Rimmer),
            "cat" => Ok(Action::Cat),
            "dave" => Ok(Action::Dave),
            "holy" => Ok(Action::Holy),
            _ => Err(()),
        }
    }
}

enum Result {
    Win,
    Lose,
    Tie,
}

impl Result {
    fn as_str(&self) -> &'static str {
        match self {
            Result::Win => "You win!",
            Result::Lose => "You lose!",
            Result::Tie => "It's a tie!",
        }
    }
}

fn get_player_action() -> Action {
    loop {
        let mut input = String::new();
        println!("Enter you action (kryten, rimmer, cat, dave, or holy):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let action = input.trim().to_lowercase().parse();
        if let Ok(action) = action {
            return action;
        }
        println!("Invalid action. Please try again.");
    }
}

fn get_ai_action() -> Action {
    let actions = [
        Action::Kryten,
        Action::Rimmer,
        Action::Cat,
        Action::Dave,
        Action::Holy,
    ];
    let index = rand::thread_rng().gen_range(0..actions.len());
    actions[index]
}

fn get_result(player_action: Action, ai_action: Action) -> Result {
    if player_action == ai_action {
        Result::Tie
    } else if (player_action == Action::Kryten && (ai_action == Action::Rimmer || ai_action == Action::Dave))
        || (player_action == Action::Rimmer && (ai_action == Action::Cat || ai_action == Action::Holy))
        || (player_action == Action::Cat && (ai_action == Action::Dave || ai_action == Action::Kryten))
        || (player_action == Action::Dave && (ai_action == Action::Holy || ai_action == Action::Rimmer))
        || (player_action == Action::Holy && (ai_action == Action::Kryten || ai_action == Action::Cat))
        {
            Result::Win
        } else {
            Result::Lose
        }
}

fn main() {
    let mut player_wins = 0;
    let mut ai_wins = 0;

    println!("Welcome to Rock-Paper-Scissors-Lizard-Spock with Red Dwarf characters!");
    println!("You will be playing against an AI that chooses actions randomly.");
    println!("The first player to win 3 games wins the match.");
    println!("");

    loop {
        let player_action = get_player_action();
        let ai_action = get_ai_action();
        let result = get_result(player_action, ai_action);

        println!("You chose {:#?}.", player_action);

        println!("The AI chose {:#?}.", ai_action);
        println!("{}", result.as_str());

        match result {
            Result::Win => player_wins += 1,
            Result::Lose => ai_wins += 1,
            Result::Tie => (),
        }

        println!("Player wins: {}", player_wins);
        println!("AI wins: {}", ai_wins);
        println!("");

        if player_wins >= 3 {
            println!("Congratulations: You won the match!");
            break;
        } else if ai_wins >= 3 {
            println!("Sorry, you lost the match. Better luck next time!");
            break;
        }
    }
}
