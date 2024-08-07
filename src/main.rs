use hausarbeit::game::Game;
use hausarbeit::game_result::GameResult;
use hausarbeit::random_agent::RandomAgent;

const NUM_GAMES: u32 = 1;
const DEPTH: u32 = 3;

fn main() {
    let mut wins = [0, 0, 0];

    for num_game in 0..NUM_GAMES {
        let mut agent1 = RandomAgent::new();
        let mut agent2 = RandomAgent::new();

        if num_game % 2 == 0 {
            let mut game = Game::new(&mut agent1, &mut agent2);
            let result = game.play();
            match result {
                GameResult::Win(player) => wins[player as usize] += 1,
                GameResult::Draw => wins[2] += 1,
                _ => {}
            }
        } else {
            let mut game = Game::new(&mut agent2, &mut agent1);
            let result = game.play();
            match result {
                GameResult::Win(player) => wins[((player as usize) + 1) % 2] += 1,
                GameResult::Draw => wins[2] += 1,
                _ => {}
            }
        }
    }

    println!("Player 1 wins: {}", wins[0]);
    println!("Player 2 wins: {}", wins[1]);
    println!("Draws: {}", wins[2]);
}
