use terminal_size::{terminal_size, Width};

pub fn center_text(text: &str) {
    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    };
    let padding = (terminal_width - text.len()) / 2;
    println!("{:padding$} {text} \n", "", padding = padding);
}
