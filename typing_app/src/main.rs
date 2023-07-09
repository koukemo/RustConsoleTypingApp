mod main_menu;
mod util {
    pub mod standard_input_reader;
}

mod enum_data {
    pub mod game_mode;
}


fn main() {
    use crate::main_menu::main_menu;

    let mut initiarize_ranking_vec = main_menu::create_ranking();

    loop {
        main_menu::display_mode();
        let selected_game_mode = main_menu::select_game_mode();

        main_menu::start_count_down(5);
        let result_time = main_menu::start_game(selected_game_mode);

        let result_ranking_vec = main_menu::register_ranking(&mut initiarize_ranking_vec, result_time);
        main_menu::display_ranking(result_ranking_vec);

        main_menu::is_continue_game();

    }
}
