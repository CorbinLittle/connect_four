
use colored::Colorize;
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Slot {
    Empty,
    Yellow,
    Red,
}
enum Result {
    RedWin,
    YelloWin,
    Draw,
}
#[derive(Clone)]
struct Board {
    board: Vec<Slot>,
    turn_counter: u32
}
impl Board {
    fn show(&self){
        let mut counter = 0;
        for i in self.board.iter(){
            match i {
                Slot::Empty => print!("0"),
                Slot::Yellow => print!("{}", "0".yellow()),
                Slot::Red => print!("{}", "0".red())
            }
            counter += 1;
            if counter % 7 == 0{
                println!()
            }
        }
    }
    fn make_move(&mut self, move_number: u32) -> bool{
        if move_number >0 && move_number <= 7{
            let move_column = self.board.iter().enumerate().filter(|x| if (move_number) % 7 > 0{(x.0 + 1) % 7 == (move_number) as usize} else{(x.0 + 1) % 7 == 0}).map(|x| x.1);
            let mut first_empty_slot: Option<u32> = None;
            for i in move_column.rev().enumerate(){
                match i.1 {
                    Slot::Empty => {first_empty_slot =  Some(6 - (i.0 as u32));
                    break},
                    _ => continue
                }
            }
            match first_empty_slot {
                None => return false,
                _ => ()
            };
            self.turn_counter += 1;
            match self.turn_counter % 2 {
                0 => self.board[(((first_empty_slot.unwrap() - 1) * 7) + (move_number - 1)) as usize] = Slot::Red,
                _ => self.board[(((first_empty_slot.unwrap() - 1) * 7) + (move_number - 1)) as usize] = Slot::Yellow
            }
            return true;
        }
        else{
            return false;
        }
    }
    fn number_of_empty_slots(&self) -> u32{
        let mut empty_slot_counter = 0;
        for i in self.board.iter(){
            match i{
                Slot::Empty => empty_slot_counter += 1,
                _ => ()
            };
        }
        return  empty_slot_counter;
    }
    fn check_result(&self) -> Option<Result>{
        let mut winning_edge_piece: Slot = Slot::Empty;
        let board = self.board.clone();
        for i in 0..42{
            if (i+1) % 7 > 0 && (i + 1) % 7 < 5{
                if board[i] == board[i + 1] && board[i + 1] == board[i + 2] && board[i +2] == board[i + 3]{
                    if board[i] != Slot::Empty{
                        winning_edge_piece = board[i]
                    }
                    
                }
            }
            if i + 24 <= 41{
                if board[i] == board[i + 8] && board[i + 8] == board[i + 16] && board[i + 16] == board[i + 24]{
                    if board[i] != Slot::Empty{
                        winning_edge_piece = board[i]
                    }
                }
            }
            if i + 18 <= 41 && ((i + 1) % 7 >=4  || i+1 %7 == 0 ){
                if board[i] == board[i + 6] && board[i + 6] == board[i + 12] && board[i + 12] == board[i + 18]{
                    if board[i] != Slot::Empty{
                        winning_edge_piece = board[i]
                    }
                }
            }
            if i < 21{
                if board[i] == board[i + 7] && board[i + 7] == board[i + 14] && board[i + 14] == board[i + 21]{
                    if board[i] != Slot::Empty{
                        winning_edge_piece = board[i]
                    }
                }
            }
        }
        match winning_edge_piece {
            Slot::Red => return Some(Result::RedWin),
            Slot::Yellow => return Some(Result::YelloWin),
            Slot::Empty => {
                if self.number_of_empty_slots() == 0{
                    return Some(Result::Draw);
                }
                else{
                    return None
                }
            }

        }
        
    }
}
fn clear_screen(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn main() {
    let mut board: Board = Board{board: [Slot::Empty;42].to_vec(), turn_counter: 0};
    loop{
        clear_screen();
        println!("1234567");
        board.show();
        let mut buffer_string = String::new();
        println!("Enter your move 1-7 :");
        std::io::stdin().read_line(&mut buffer_string).unwrap();
        let move_number: u32 = buffer_string.trim().parse().unwrap_or(100);
        let move_is_valid = board.make_move(move_number);
        if !move_is_valid{
            println!("invalid move");
            continue;
        }
        let result = board.check_result();
        match result{
            None => (),
            Some(Result::Draw) => {
                clear_screen();
                println!("Draw");
                board.show();
                break
            },
            Some(Result::RedWin) => {
                clear_screen();
                println!("Red Won");
                board.show();
                break
            },
            Some(Result::YelloWin) => {
                clear_screen();
                println!("Yellow won");
                board.show();
                break
            },
        }
        
        
    }
    board.check_result();

}
