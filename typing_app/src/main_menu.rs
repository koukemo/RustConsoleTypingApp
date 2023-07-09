pub mod main_menu {
    use crate::enum_data::game_mode::GameMode;
    use crate::util::standard_input_reader::StandardInputReader;
    use std::process;
    use std::thread;
    use std::time::Duration;
    use std::time::Instant;
    use rand::seq::SliceRandom;

    pub fn display_mode() {
        println!("Please select game modes.");
        println!("1: Easy");
        println!("2: Normal");
        println!("3: Hard");
    }

    pub fn select_game_mode() -> GameMode {
        let input_number = StandardInputReader::input_number("> ");

        match input_number {
            1 => GameMode::EASY(String::from("EASY")),
            2 => GameMode::NORMAL(String::from("NORMAL")),
            3 => GameMode::HARD(String::from("HARD")),
            _ => {
                println!("Start Normal mode.");
                return GameMode::NORMAL(String::from("NORMAL"));
            }
        }
    }

    pub fn start_count_down(seconds: u32) {
        for remaining in (0..seconds).rev() {
            println!("{}..........", remaining + 1);
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn start_game(game_mode: GameMode) -> u64 {
        println!("Start!");

        let mut _question_count = 0;
        let mut question_vec: Vec<&str> = Vec::new();
        
        let easy_vec: Vec<&str> = vec![
            "byte", "int", "long", "bool", "void", 
            "if", "else", "case", "for", "do", 
            "try", "this", "base", "new", "enum", 
            "as", "is", "true", "null", "goto" 
        ];
        let normal_vec: Vec<&str> = vec![
            "short", "float", "double", "switch", "default", 
            "while", "continue", "break", "return", "catch", 
            "finally", "throw", "using", "class", "public", 
            "protected", "private", "sealed", "static", "abstract", 
            "volatile", "false", "const"
        ];
        let hard_vec: Vec<&str> = vec![
            "interface", "namespace", "stackalloc", "unchecked"
        ];

        let start = Instant::now();

        match game_mode {
            GameMode::EASY(_mode) => {
                _question_count = 5;
                question_vec.extend(easy_vec);
            }
            GameMode::NORMAL(_mode) => {
                _question_count = 10;
                question_vec.extend(easy_vec);
                question_vec.extend(normal_vec);
            }
            GameMode::HARD(_mode) => {
                _question_count = 20;
                question_vec.extend(easy_vec);
                question_vec.extend(normal_vec);
                question_vec.extend(hard_vec);
            }
        }

        let mut random = rand::thread_rng();
        question_vec.shuffle(&mut random);
        let mut counter = 0;

        for question in question_vec {
            if counter == _question_count {
                break;
            }

            loop {
                let message = format!("{}: ", question);
                let answer = StandardInputReader::input_string(&message);

                if question == answer {
                    println!("OK!");
                    break;
                } else {
                    println!("MISS...");
                }
            }

            counter += 1;
        }

        let stop = start.elapsed();
        let stop_millseconds = stop.as_millis();
        let millseconds_u64: u64 = stop_millseconds as u64;

        println!("finished. time = {}[ms]", millseconds_u64);

        return millseconds_u64;
    }

    pub fn create_ranking() -> Vec<u64> {
        let default_ranking_vec: Vec<u64> = vec![999_999_999_999; 10];
        let mut ranking_vec: Vec<u64> = Vec::new();

        ranking_vec.extend(default_ranking_vec);

        return ranking_vec;
    }

    pub fn register_ranking(ranking_vec: &mut Vec<u64>, result_time: u64) -> Vec<u64> {
        ranking_vec.push(result_time);
        ranking_vec.sort();
        ranking_vec.pop();

        ranking_vec.clone()
    }

    pub fn display_ranking(ranking_vec: Vec<u64>) {
        let mut index = 1;

        println!("{:>4}  {:>15}", "Rank", "Time");
        println!("---------------------");

        for ranking in ranking_vec {
            println!("{:>4}: {:>15}", index, ranking);
            index += 1;
        }
    }

    pub fn is_continue_game() {
        loop {
            println!("Continue? (1: Yes / 2: No)");

            let input_number = StandardInputReader::input_number("> ");

            match input_number {
                1 => {
                    println!("");
                    return
                }
                2 => {
                    println!("Quit the typing application.");
                    process::exit(0);
                }
                _ => {
                    println!("Please input correct number");
                }
            }
        }
    }
}