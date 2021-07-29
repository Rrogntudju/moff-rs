mod mon;
use mon::{get_d6, set_d6};

fn main() {
    let new = match get_d6() {
        1 => 4, // ON => OFF
        _ => 1, // OFF => ON
    };
    set_d6(new);
}
