mod mon;
use mon::{get_d6, set_d6};

fn main() {
    let new = match get_d6() {
        4 => 1, // OFF => ON
        _ => 4, // ON => OFF
    };
    set_d6(new);
}
