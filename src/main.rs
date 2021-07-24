use std::io::{Write, stdout};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{env, fs::*};

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

const REFRESH_INTERVAL: u64 = 100;

const START: [&str; HEIGHT] = ["000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000",
                               "000000000000000000000000000000"];

fn main() {

    // let mut map: [[bool; WIDTH]; HEIGHT] = [[true; WIDTH]; HEIGHT]; // Represent whether a cell is alive or not
    // let mut map: [[bool; WIDTH]; HEIGHT] = [[false, true, false, false, false, false, false, false, false, false],
    //                                         [false, false, true, false, false, false, false, false, false, false],
    //                                         [true, true, true, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false], 
    //                                         [false, false, false, false, false, false, false, false, false, false]];

    let mut map: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    print!("Enter txt file name (without .txt) to load : ");
    std::io::stdout().flush().unwrap();
    let mut filename: String = String::new();
    std::io::stdin().read_line(&mut filename).expect("Error while reading input");
    let mut path_to_file = env::current_dir().unwrap().to_str().unwrap().to_owned();
    path_to_file.push_str(&format!(r#"\{}"#, filename));
    path_to_file = path_to_file.replace("\n", "");
    path_to_file.pop();
    path_to_file.push_str(".txt");

    // let file: File = File::open(path_to_file).map_or_else(|e| {
    //     panic!();
    // }, |f| {
    //     f
    // });
    
    println!("{}", path_to_file);
    // println!("{}", read_to_string(&path_to_file).unwrap());

    let file_content = read_to_string(path_to_file).unwrap();

    let file_line: Vec<&str> = file_content.split("\n").collect();
    let file_map: Vec<Vec<char>> = file_line.into_iter().map(|line| {
        let char_array: Vec<char> = line.chars().filter_map(|e| {
            if e != '0' && e != '1' {
                None
            } else {
                Some(e)
            }
        }).collect();
        char_array
    }).collect();

    for line in &file_map {
        for c in line {
            print!("{}", c);
        }
        print!("\n");
        stdout().flush().unwrap();
    }

    let mut file_ok: bool = true;
    if file_map.len() != HEIGHT {
        file_ok = false;
        println!("Error, file must contain {} lines", HEIGHT);
        panic!();
    }
    for line in &file_map {
        println!("{}", line.len());
        if line.len() != WIDTH {
            file_ok = false;
            println!("Error, each file line must contain {} chars", WIDTH);
            panic!();
        }
    }

    // let file_copy = file_line.clone();

    // let chars: Vec<Vec<char>> = file_copy.into_iter().map(|line| {
    //     let char_line: Vec<char> = line.chars().collect();
    //     char_line
    // }).collect();

    if file_ok {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if file_map[i][j] == '0' {
                    map[i][j] = false;
                } else {
                    map[i][j] = true;
                }
            }
        }
    }

    // let mut start_config_ok: bool = true;
    // for string in START { // Check if START is well formated
    //     if string.len() == WIDTH {
    //         continue;
    //     } else {
    //         println!("Error, one line doesn't have {} chars", WIDTH);
    //         start_config_ok = false;
    //         break;
    //     }
    // }

    // if start_config_ok { // Resolving map from START
    //     for i in 0..HEIGHT - 1 {
    //         for j in 0..WIDTH - 1 {
    //             if START[i].chars().nth(j) == Some('0') {
    //                 map[i][j] = false;
    //             } else {
    //                 map[i][j] = true;
    //             }
    //         }
    //     }
    // }
    

    let separator: String = ['-'; WIDTH * 2].iter().map(|e| {
        String::from(e.clone())
    }).collect();

    let mut instant = std::time::Instant::now();

    loop {

        if instant.elapsed() >= Duration::from_millis(REFRESH_INTERVAL) {
            instant = Instant::now();

            println!("{}", separator);

            for line in map { // Start drawing cells on the terminal
                let mut cell_line = String::new();
                for cell in line {
                    if cell {
                        cell_line.push('*');
                    } else {
                        cell_line.push(' ');
                    }
                }
                let formated_cell_line: String = cell_line.chars().enumerate().map(|(i, e)| {
                    if i != WIDTH - 1 {
                        String::from(format!("{} ", e))
                    } else {
                        String::from(e)
                    }
                }).collect();
                println!("{}", formated_cell_line);
            }

            // Updating the cell array

            let buffer = map.clone(); // Keep a copie of the last update, we will make check on the buffer and write result on the map

            // Conditions : 
            // If cell is dead and have exactly 3 living cell next to it, the cell become alive
            // If cell is alive and have 2 or 3 living cell next to it, the cell stay alive, else, the cell die

            let mut i = 0; // Collumn counter
            let mut j = 0; // Row counter

            for line in buffer {
                for cell in line {

                    // println!("({}, {})", i, j);

                    
                    let mut living_neighbour_counter = 0;
                    let mut neighbourhood: [bool; 8] = [false; 8]; // (i-1, j-1) (i-1, j) (i-1, j+1) 
                                                                    // (i,   j-1) (current)(i,   j+1)
                                                                    // (i+1, j-1) (i+1, j) (i+1, j+1)

                    // If we cannot access buffer index (out of scope) we consider the cell dead (neighbourhood[n] = false)
                    if i != 0 && j != 0 {
                        neighbourhood[0] = buffer[i-1][j-1];
                    } else { neighbourhood[0] = false }
                    if i != 0 {
                        neighbourhood[1] = buffer[i-1][j];
                    } else { neighbourhood[1] = false }
                    if i != 0 && j != WIDTH - 1 {
                        neighbourhood[2] = buffer[i-1][j+1];
                    } else { neighbourhood[2] = false }
                    if j != 0 {
                        neighbourhood[3] = buffer[i][j-1];
                    } else { neighbourhood[3] = false }
                    // -----------------
                    // Current cell here
                    // -----------------
                    if j != WIDTH - 1 {
                        neighbourhood[4] = buffer[i][j+1];
                    } else { neighbourhood[4] = false }
                    if i != HEIGHT - 1 && j != 0 {
                        neighbourhood[5] = buffer[i+1][j-1];
                    } else { neighbourhood[5] = false }
                    if i != HEIGHT - 1 {
                        neighbourhood[6] = buffer[i+1][j];
                    } else { neighbourhood[6] = false }
                    if i != HEIGHT - 1 && j != WIDTH - 1 {
                        neighbourhood[7] = buffer[i+1][j+1];
                    } else { neighbourhood[7] = false }

                    for is_alive in neighbourhood {
                        if is_alive {
                            // We add one to the living counter for each living cell in the neighbourhood
                            living_neighbour_counter = living_neighbour_counter + 1;
                        }
                    }

                    if cell == false {
                        if living_neighbour_counter == 3 {
                            map[i][j] = true; // If there is 3 living cell around a dead cell, we make the cell living for the next iteration
                        }
                    } else { // If cell is living
                        if living_neighbour_counter == 2 || living_neighbour_counter == 3 {
                            map[i][j] = true; // Cell stay alive if there is 2 or 3 living cell around
                        } else {
                            map[i][j] = false;
                        }
                    }
                        
                    

                    j = j + 1;
                    if j == WIDTH {
                        j = 0;
                    }
                }
                i = i + 1;
            }

        }

    }
}
