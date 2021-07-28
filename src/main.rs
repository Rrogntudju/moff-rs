mod mon;
use mon::{get_current_d6, set_d6};

fn main() {
    if let Some(c) = get_current_d6() {
        let new = if c == 1 { 4 } else { 1 }; // Basculer
        set_d6(new);
    }
}
