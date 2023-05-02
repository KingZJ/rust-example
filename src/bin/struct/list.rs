use std::fmt;

#[derive(Debug)]
pub struct List(pub Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arr = &self.0;
        write!(f, "[")?; // 等同于 try!(write!(f, "["))  报错之间抛出错误，终止执行此方法中的后续代码
        for (count, v) in arr.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}
