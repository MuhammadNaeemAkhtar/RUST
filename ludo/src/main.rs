use std::io;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let total_players: u32 = num_of_players();

    let mut players = Vec::new(); 
    let mut players_scores = HashMap::new();

    for player in 1..=total_players {
        println!("Enter Name of Player {}:", player);

        let player_name = String::from(user_input().trim());
        players.push(player_name.clone());

        players_scores.insert(player_name, 0);
    }

    println!("{:?}", players);
    println!("{:?}", players_scores);

    let mut turn: u32 = 0;
    let mut sixer;
    let mut player_num = 0;

    loop {
        turn = turn + 1;
        for player in &players {
            player_num = player_num + 1;
            let mut dice_role = rand::thread_rng().gen_range(1, 7);

            if dice_role == 6 {
                sixer = 0;
                loop {
                    sixer = sixer + 1;
                    dice_role = rand::thread_rng().gen_range(1, 7);
                    if sixer == 3 {
                        println!("\nAlas! Turn {} Dice Roll of Player {} - {} is 0 due to {} Sixers and Total {}\n", turn, player_num, player, sixer, players_scores.get(&player.clone()).unwrap() );
                        sixer = 0;
                        break;
                    } 
                    if dice_role != 6 && sixer < 3 {
                        let curr_player_score: u32 = players_scores.get(&player.clone()).unwrap() + dice_role + 6;
                        
                        if curr_player_score == 100 {
                            println!("\nCongratulations! Player {} has won on turn {}", player, turn);
                            return;
                        }

                        if curr_player_score > 100 {
                            println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}", turn, player_num, player, dice_role + 6, players_scores.get(&player.clone()).unwrap() );
                            break;
                        } else {
                            players_scores.insert(String::from(player.clone()), curr_player_score);
                            println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}", turn, player_num, player, dice_role + 6, curr_player_score );
                            break;
                        } 
                    }
                }
            } else {
                let curr_player_score: u32 = players_scores.get(&player.clone()).unwrap() + dice_role;

                if curr_player_score > 100 {
                    println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}", turn, player_num, player, dice_role, players_scores.get(&player.clone()).unwrap() ); 
                    continue;
                } else {
                    println!("Turn {} Dice Roll of Player {} - {} is {} and Total {}", turn, player_num, player, dice_role, curr_player_score ); 
                    players_scores.insert(String::from(player.clone()), curr_player_score);   
                }
                
                check_and_kickout(&mut players_scores, &players, &player, &curr_player_score);

                if curr_player_score == 100 {
                    println!("\nCongratulations! Player {} has won on turn {}", player, turn);
                    return;
                }
            }
        }
        player_num = 0;
        enter_to_continue();
    }    
}

fn user_input() -> String {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    input_text
}

fn enter_to_continue() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
}

fn check_and_kickout(map: &mut HashMap<String, u32>, players: &Vec<String>, player: &String, score: &u32) {
    for p in players {
        if p == player {
            continue;
        } else {
            if map.get(&p.to_string()).unwrap() == score {
                map.insert(p.to_string(), 0);
                println!("\nAlas! {} has been kicked by {} at score of {}\n", p, player, score);
            }
        }
    }
}

fn num_of_players() -> u32 {
    println!("Welcome to LUDO");
    println!("Enter Number of Players!");
    let mut total_players = user_input();

    loop {
        match total_players.trim().parse::<u32>() {
            Ok(_) => { break; },
            Err(..) => {    println!("Only Numbers Allowed, Try Again"); 
                            println!("Enter Number of Players!");
                            total_players = user_input() 
                        },
        };
    }
    total_players.trim().parse().unwrap()
}