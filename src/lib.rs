
pub mod utils;
pub mod context;
pub mod protocal;
pub struct Conf<'a> {
    pub host: &'a str,
    pub port: u32,
}
impl<'a> Conf<'a> {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}



#[cfg(test)]
mod test {

    #[test]
    fn name() {
        let x: i32;
    if true {
        x = 1;
        println!("{}", x);
    }
        // println!("{}", x);
        //
        let top = 1;
        println!("stack top:{:p}", &top);
        let top2 = 11;
        println!("stack top:{:p}", &top2);
        let top3 = 5;
        println!("stack top:{:p}", &top3);
        // let v = Box::new(crate::protocal::MetaMsg::new("1", "2"));
        let v = Box::new("a".to_string());
        println!("stack top:{:p} {:p} {}", v, &v, *&v);
        println!("{:X}", &v as *const Box<String> as usize);
    }



}


