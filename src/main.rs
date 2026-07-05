use std::io::Result;

mod ascii;
mod terminal;

use ascii::{generate_ascii_art, get_char_map};
use terminal::scroll_ascii_art;

fn main() -> Result<()> {
    let char_map = get_char_map();
    let ascii_art = generate_ascii_art("わかるわ", &char_map);

    if ascii_art.is_empty() || ascii_art[0].is_empty() {
        return Ok(());
    }

    scroll_ascii_art(&ascii_art)
}
