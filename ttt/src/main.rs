use std::{cell::RefCell, rc::Rc};

fn main() {
    let ccc=Some(Rc::new(RefCell::new(Student{ id: 1})));

    ccc.clone().unwrap().borrow_mut().id=11;

    println!("{}",ccc.unwrap().borrow().id);

    

}

struct Student{
    id:u128,
}
