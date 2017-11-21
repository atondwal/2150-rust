/* list.rs
 * This implements a Linked List with the following API
 * - INSERT(LIST,INDEX,ELEM)
 * - REMOVE(INDEX,LIST)
 * - LENGTH(LIST)
 * - FIND(ELEM,LIST)
 */

#![feature(box_patterns)]

trait List<T> {
    fn insert(&mut self, i32, T) -> bool;
    fn remove(&mut self, i32) -> Option<T>;
    fn length(& self) -> i32;
    fn find(& self, T) -> Option<i32>;
}

enum LinkedList<T> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

impl<T:PartialEq> List<T> for LinkedList<T> {

    fn insert(&mut self, i: i32, x:T ) -> bool
    {
        use LinkedList::*;
        use std::mem;
        if i == 0 {
            *self = Cons(x,Box::new(mem::replace(self, Nil)));
            return true;
        }
        match *self {
            Nil => false,
            Cons(_,ref mut l) => l.insert(i-1,x)
        }
    }

    fn remove(&mut self, i:i32) -> Option<T>
    {
        use LinkedList::*;
        use std::mem;
        if i == 0 {
            match mem::replace(self, Nil) {
                 Nil => return None,
                 Cons(x,box l) => {*self = l; return Some(x);}
            }
        } else {
            match *self {
                Nil => return None,
                Cons(_,ref mut l) => l.remove(i-1)
            }
        }
    }

    fn length(& self) -> i32
    {
        use LinkedList::*;
        match *self {
            Nil => 0,
            Cons(_,ref l) => 1+l.length()
        }
    }

    fn find(& self, x:T) -> Option<i32>
    {
        use LinkedList::*;
        match *self {
            Nil => None,
            Cons(ref x0,ref l) =>
                if x == *x0 {
                    Some(0)
                } else {
                    l.find(x).map(|x| x+1)
                }
        }
    }
}

fn pop<T>(l: &mut List<T>) -> Option<T>
{
    l.remove(0)
}

fn push<T>(l: &mut List<T>, x:T) -> bool
{
    l.insert(0,x)
}

fn main()
{
        let list : &mut LinkedList<i32> = &mut LinkedList::Nil;

        // Check empty list behaves right
        assert_eq!(pop(list), None);

        // Populate list
        push(list,1);
        push(list,2);
        push(list,3);

        // Check normal removal
        assert_eq!(pop(list), Some(3));
        assert_eq!(pop(list), Some(2));

        // Push some more just to make sure nothing's corrupted
        push(list,4);
        push(list,5);

        // Check normal removal
        assert_eq!(pop(list), Some(5));
        assert_eq!(pop(list), Some(4));

        // Check exhaustion
        assert_eq!(pop(list), Some(1));
        assert_eq!(pop(list), None);
}
