pub fn test() {
    println!("\n........str type exercise......");
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..=5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");
    println!("Success! {}", s1.len());

    let s2 = "hi,中国，你好";
    let s3 = s2; // 支持 Copy
    println!("{s2}, s3: {}", s3);
    println!("{:?}", s2.chars().nth(3));
    println!("{:?}", s2.char_indices().nth(3));
    println!("{:?}", s2.char_indices().nth(4));
    println!("{:?}", s2.char_indices().nth(5));
    println!("{:?}", s2.char_indices().nth(6));

    println!("{:?}", utf8_slice(s2, 3, 15));
    println!("{:?}", utf8_slice1(s2, 3, 15));
}

// [begin, end)
pub fn utf8_slice(s: &str, begin: usize, end: usize) -> &str {
    if begin > end {
        return "";
    }
    s.char_indices()
        .nth(begin)
        .and_then(|(start_pos, _)| {
            s.char_indices()
                .nth(end)
                .map_or(Some(&s[start_pos..]), |(end_pos, _)| {
                    Some(&s[start_pos..end_pos])
                })
        })
        .unwrap_or_default()
}

pub fn utf8_slice1(s: &str, begin: usize, end: usize) -> &str {
    if begin > end {
        return "";
    }
    s.char_indices()
        .nth(begin)
        .and_then(|(start_pos, _)| {
            if end >= utf8_len(s) {
                return Some(&s[start_pos..]);
            }
            s.char_indices()
                .nth(end)
                .map(|(end_pos, _)| &s[start_pos..end_pos])
        })
        .unwrap_or_default()
}

pub fn utf8_len(s: &str) -> usize {
    s.chars().count()
}
