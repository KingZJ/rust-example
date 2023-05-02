pub struct Person {
    pub job: Option<Job>,
}

#[derive(Clone, Copy)]
pub struct Job {
    pub phone: Option<Phone>,
}

#[derive(Clone, Copy)]
pub struct Phone {
    pub area_code: Option<u8>,
    pub number: u64,
}

impl Person {
    pub fn work_phone_area_code(&self) -> Option<u8> {
        /// `?` 运算符快速解开 Option, 注意所有权问题，会导致部分移动
        self.job?.phone?.area_code
    }
}

pub fn next_birthday(cur_age: Option<u8>) -> Option<String> {
    // ? 解开Option, Some(v) 将 v 赋值 给变量，否则程序返回None
    let next_age: u8 = cur_age?;
    Some(format!("Next year I will be {}", next_age))
}
