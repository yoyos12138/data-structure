use std::{cell::RefCell, rc::Rc};



fn main() {
    let mut list1=Zlist::new();
    
    for i in 0..8 {
        list1.insert_tail(i);
        list1.show();
    }
    
    list1.show();
    list1.insert_tail(2);
    list1.show();
}

struct Zlist {
    length: u128,
    next: Option<Rc<RefCell<Znode>>>,
}

struct Znode {
    value: u128,
    next: Option<Rc<RefCell<Znode>>>,
}

impl Zlist {
    fn new() -> Self {
        Zlist {
            length: 0,
            next: None,
        }
    }
    
    fn insert_head(&mut self, n: u128) {
        let new_node = Some(Rc::new(RefCell::new(Znode {
            value: n,
            next: self.next.take(),
        })));
        self.next = new_node;
        self.length += 1;
    }
    fn insert_tail(&mut self,n:u128){
        if self.length==0 {
            self.insert_head(n);
        }else {
            let mut node0=Some(Rc::new(RefCell::new(Znode{ 
                value: 1, 
                next: self.next.clone(), 
            })));
            while let Some(_) = node0.clone().unwrap().borrow().next {
                node0=node0.clone().unwrap().borrow().next.clone();
            }
            node0.clone().unwrap().borrow_mut().next=Some(Rc::new(RefCell::new(Znode{ value: n, next: None })));
            self.length+=1;    
        }
    }
    fn show(&self){
        print!("[[{}]:[",self.length);
        if self.length != 0 {
            let mut next_node = self.next.clone();
            while let Some(node) = next_node {
                print!("{} ", node.borrow().value);
                next_node = node.borrow().next.clone();
            }
            println!("]");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Zlist;
    use rand::{thread_rng, Rng};
    #[test]
    fn add_head() {
        let mut list1 = Zlist::new();
        let n: u128 = rand::thread_rng().gen_range(0..u128::MAX);
        for i in 0..n {
            list1.insert_head(i);
        }
        assert_eq!(n,list1.length)
    }
}