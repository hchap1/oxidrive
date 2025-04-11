pub fn format_raw_size(size: usize) -> String {
    let mut size = size as f32;
    let mut idx: usize = 0;

    while size > 1024f32 && idx < 8 {
        size /= 1024f32;
        idx += 1;
    }

    let suffix = match idx {
        0 => "B",
        1 => "K",
        2 => "M",
        3 => "G",
        4 => "T",
        5 => "P",
        6 => "E",
        7 => "Z",
        8 => "Y",
        _ => "?"
    };

    format!("{size:.2}{suffix}")
}
