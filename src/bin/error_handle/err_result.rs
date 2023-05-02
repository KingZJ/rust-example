pub fn result_map_test() {
    let s = vec!["tofu", "90", "10"];

    let (numbers, errors): (Vec<_>, Vec<_>) = s
        .into_iter()
        .map(|item| item.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

pub fn test2() {
    let s = vec!["tofu", "90", "10"];

    let numbers: Vec<_> = s.into_iter().map(|item| item.parse::<i32>()).collect();
    println!("{:?}", numbers);
}

pub fn test3() {
    let s = vec!["121", "tofu", "90", "10"];

    // 找到 Result::Err 则停止遍历, 并返回 err
    let numbers: Result<Vec<_>, _> = s.into_iter().map(|item| item.parse::<i32>()).collect();
    println!("{:?}", numbers);
}

pub fn test4() {
    let s = vec!["121", "tofu", "90", "10"];

    println!("parse: {:?}", "121".parse::<i32>().ok());
    // 找到 Result::Err 则停止遍历, 并返回 err
    let numbers: Vec<_> = s
        .into_iter()
        .filter_map(|item| item.parse::<i32>().ok())
        .collect();
    println!("{:?}", numbers);
}
