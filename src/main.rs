use std::sync::{Mutex, Arc};
use std::thread;
use rand::Rng;

struct Player {
    name: String,
    team: String,
    points: u32,
}

impl Player {
    fn new(name: &str, team: &str) -> Player {
        Player {
            name: String::from(name),
            team: String::from(team),
            points: 0,
        }
    }

    fn score(&mut self, points: u32) {
        self.points += points;
    }
}

fn main() {
    let player1 = Arc::new(Mutex::new(Player::new("Tim Duncan", "San Antonio Spurs"))); // arc is needed for the threading
    let player2 = Arc::new(Mutex::new(Player::new("Manu Ginobili", "San Antonio Spurs"))); 
    //mutex only allows one thread to access a players info at once, right?

    let player1_clone = Arc::clone(&player1); //increases arc reference count by 1 to  player1
    let handle1 = thread::spawn(move || { //make a thread - nice
        
        let top = rand::thread_rng().gen_range(3..=12);
        
        for _ in 0..top {
            let mut player = player1_clone.lock().unwrap(); //You must attempt to acquire the lock before using the data. -trpl
            player.score(2);
            println!("{} on the {} scores 2 points! Player's total points: {}", player.name, player.team, player.points);
        }
    });

    let player2_clone = Arc::clone(&player2);
    let handle2 = thread::spawn(move || { 
        for _ in 0..5 {
            let mut player = player2_clone.lock().unwrap(); 
            player.score(3);
            println!("{} on the {} scores 3 points! Player's total points: {}", player.name, player.team, player.points);
        }
    });

    handle2.join().unwrap();
    handle1.join().unwrap();
    
    let player1_final = player1.lock().unwrap();
    let player2_final = player2.lock().unwrap();

    println!("Game Over!");
    println!("Results:");
    println!("{}: {}", player1_final.name, player1_final.points);
    println!("{}: {}", player2_final.name, player2_final.points);
    
    /*println!("{}: {}", {
        let player1_again = player1.lock().unwrap();
        player1_guard.name
    }, {
        let player1_again = player1.lock().unwrap();
        player1_guard.points
    });*/
    
    //Originally I encountered a deadlock by doing this:
    //println!("{}: {}", player1.lock().unwrap().name, player1.lock().unwrap().points);

}
