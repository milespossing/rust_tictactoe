use itertools::join;
use std::num::ParseIntError;
use std::io::{stdin, stdout, Write};
use std::cmp::PartialEq;
use std::fmt;

#[derive(Copy,Clone,PartialEq)]
enum Player{
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Player::X => return write!(f, "X"),
            Player::O => return write!(f, "O"),
        };
    }
}

fn next_move(p: Player) -> Player {match p {
    Player::X => Player::O,
    Player::O => Player::X,
}}

struct Board{
    pub positions:[[Option<Player>;3];3]
}

impl Board {
    pub fn new()->Board{Board{positions:[[None;3];3]}}
}

fn render(b:&Board)->String{
    join(b.positions.iter().map(|row| join(row.iter().map(|col| match col{
        None => {" "},
        Some(Player::X) => {"X"},
        Some(Player::O) => {"O"}
    }),"|")),"\n-+-+-\n")
}

type Err = ParseIntError;

enum Response{
    Move((usize,usize)),
    Stop,
}

fn full_match(p1:Option<Player>, p2:Option<Player>, p3:Option<Player>)->Option<Player>{
    match(p1,p2,p3){
        (Some(p1),Some(p2),Some(p3)) => {if p1 == p2 && p2 == p3 {return Some(p1)}},
        (_,_,_) => return None
    }
    None
}

fn check_winner(b:&Board)-> Option<Player>{
    if let Some(s) = full_match(b.positions[0][0],b.positions[0][1],b.positions[0][2]) {return Some(s);}
    if let Some(s) = full_match(b.positions[1][0],b.positions[1][1],b.positions[1][2]) {return Some(s);}
    if let Some(s) = full_match(b.positions[2][0],b.positions[2][1],b.positions[2][2]) {return Some(s);}
    if let Some(s) = full_match(b.positions[0][0],b.positions[1][0],b.positions[2][0]) {return Some(s);}
    if let Some(s) = full_match(b.positions[0][1],b.positions[1][1],b.positions[2][1]) {return Some(s);}
    if let Some(s) = full_match(b.positions[0][2],b.positions[1][2],b.positions[2][2]) {return Some(s);}
    if let Some(s) = full_match(b.positions[0][0],b.positions[1][1],b.positions[2][2]) {return Some(s);}
    if let Some(s) = full_match(b.positions[2][0],b.positions[1][1],b.positions[0][2]) {return Some(s);}
    None
}

fn check_filled(b:&Board)->bool{
    b.positions.iter().all(|row| row.iter().all(|col| col.is_some()))
}

fn get_input()-> Result<Response,Err>{
    print!("Please enter location (Row,Col) as \"1,2\" or 'stop': ");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    if s == "stop" {return Ok(Response::Stop)}
    let coord = get_coord(&s)?;
    Ok(Response::Move(coord))
}

fn get_coord(s:&str)->Result<(usize,usize),Err>{
    let pair:Vec<&str> = s.split(",").collect();
    let row = pair[0].parse::<usize>()? - 1;
    let col = pair[1].parse::<usize>()? - 1;
    Ok((row,col))
}

fn game_loop(mut b:Board, p:Player){
    if let Some(winner) = check_winner(&b) {
        println!("{} won the game",winner);
        return;
    }
    if check_filled(&b)
    {
        println!("Tie! The board is full.");
        return;
    }
    let response = get_input();
    match response {
        Ok(Response::Move(t)) => {
            if let Some(_) = b.positions[t.0][t.1] {
                println!("Cannot move to {},{} as it is already occupied.",t.0 + 1,t.1 + 1);
                game_loop(b,p);
                return;
            }
            b.positions[t.0][t.1] = Some(p);
            println!("{}",render(&b));
            game_loop(b,next_move(p));
        },
        Ok(Response::Stop) => {
            println!("Game cancelled. Final board:");
            println!("{}",render(&b));
        },
        Err(err) => {
            println!("Error: {}",err);
            game_loop(b,p);
        }
    }
}

fn main() {
    let b = Board::new();
    game_loop(b,Player::X);
}
