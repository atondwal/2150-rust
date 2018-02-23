use std::rc::Rc;

trait List<T:PartialEq> {
    fn insert(Rc<Self>,i32,T) -> Option<Rc<Self>>;
    fn remove(&self,i32) -> Rc<Self>;
    fn length(&self) -> i32;
    fn find(&self,T) -> Option<i32>;
}

enum ImmutableList<T:Copy> {
    Cons(T,Rc<ImmutableList<T>>),
    Nil(),
}

impl<T:PartialEq + Copy> List<T> for ImmutableList<T> {

    fn length(&self) -> i32 {
        use ImmutableList::*;
        match *self {
            Nil() => 0,
            Cons(_,ref xs) => xs.length() +1
        }
    }

    fn find(&self, x0:T) -> Option<i32>
    {
        use ImmutableList::*;
        match *self {
            Nil() => None,
            Cons(ref x,ref xs) => if x0 == *x { Some(0) } else { xs.find(x0).map(|i| i+1) }
        }
    }

    fn remove(&self,i:i32) -> Rc<Self> {
        use ImmutableList::*;
        if i==0 {
            match *self {
                Nil() => return Rc::new(Nil()),
                Cons(_,ref xs) => return xs.clone()
            }
        };
        match *self {
            Nil() => Rc::new(Nil()),
            Cons(x,ref xs) => Rc::new(Cons(x,xs.remove(i-1)))
        }
    }

    fn insert(foo : Rc<Self>, i:i32,x0:T) -> Option<Rc<Self>> {
        use ImmutableList::*;
        if i==0 {
            return Some(Rc::new(Cons(x0,foo.clone())));
        };
        match *foo {
            Nil() => None,
            Cons(x,ref xs) => Self::insert(xs.clone(),i-1,x0).map(|l| Rc::new(Cons(x,l)))
        }
    }
}

fn main () {}
 
