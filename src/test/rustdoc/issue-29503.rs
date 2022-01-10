use std::fmt;

// @has issue_29503/trait.MyTrait.html
pub trait MyTrait {
    fn my_string(&self) -> String;
}

// @has - "//div[@id='implementors-list']//div[@id='impl-MyTrait-for-T']//h3[@class='code-header in-band']" "impl<T> MyTrait for T where T: Debug"
impl<T> MyTrait for T where T: fmt::Debug {
>>>>>>> 083cf2a97a8... rustdoc: Add more semantic information to impl ids
    fn my_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn main() {}
