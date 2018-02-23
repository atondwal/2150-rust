trait List<T:PartialEq> {
    fn insert(&mut self,i32,T) -> bool;
    //fn remove(&mut self,i32) -> Option<T>;
    fn length(&self) -> i32;
    fn find(&self,T) -> Option<i32>;
}

type DoublyLinkedList<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item : T,
    right: DoublyLinkedList<T>,
    left : DoublyLinkedList<T>,
}

impl<T:PartialEq> List<T> for DoublyLinkedList<T> {

    // Need to use unsafe or Arc because we can't have two pointers
    // to the same object
    fn insert(&mut self, i:i32, x:T) -> bool {
        if i == 0 {
            let new_node = Box::new(Node{
                item : x,
                right: self.take(),
                left : None,
            });
            new_node.right.as_ref().map(|x| x.left = Some(new_node));
            *self = Some(new_node);
            true
        } else {
            unimplemented!()
        }
    }
    fn length(&self) -> i32
    {
        match *self {
            None => 0,
            Some(ref l) => 1+l.right.length()
        }
    }

    fn find(&self, x:T) -> Option<i32>
    {
        self.as_ref().and_then(|l|
            if l.item == x {
                Some(0)
            } else {
                l.right.find(x).map(|i| i+1)
            })
    }
}

fn main() {}
