// This program is part of my Rust learing
use crate::utils::Runner;

mod game;
mod kub;
mod rbe;
mod rw;
mod utils;

use crate::rbe::hello::formatted_print;
use crate::rw::io::dir::list_dir;

/// Add simple games example
fn add_simple_games_examples(container: &mut Vec<Runner>) {
    // number guessing game
    container.push(Runner {
        is_run: false,
        run: game::guessing_game,
    });
}

/// Add Rust by Example
fn add_rust_by_examples(container: &mut Vec<Runner>) {
    // Hello::Formated print
    container.push(Runner {
        is_run: true,
        run: formatted_print,
    });
}

/// Add Rust Basic Examples
fn add_rust_basic_examples(container: &mut Vec<Runner>) {
    container.push(Runner {
        is_run: true,
        run: formatted_print,
    });
    container.push(Runner {
        is_run: true,
        run: kub::boxes::show_a_squre_box,
    });
    container.push(Runner {
        is_run: true,
        run: kub::pallet::list_pallet_size,
    });
    container.push(Runner {
        is_run: true,
        run: list_dir,
    });
}

fn main() {
    let mut topic_items: Vec<utils::Runner> = Vec::with_capacity(10);

    // topic configurations
    add_rust_by_examples(&mut topic_items);
    add_simple_games_examples(&mut topic_items);
    add_rust_basic_examples(&mut topic_items);

    // example code execution
    for topic in topic_items {
        if topic.is_run {
            (topic.run)();
        }
    }
}
