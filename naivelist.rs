use std::rc::Rc;

#[derive(Clone)]
enum List<T:Clone> {
    Nil(),
    Cons(T,Rc<List<T>>)
}

fn length<T:Clone>(l : Rc<List<T>>) -> i32
{
    use List::*;
    match *l {
        Nil() => 0,
        Cons(_,ref xs) => length(xs.clone()) +1
    }
}

fn find<T:Clone + PartialEq>(l : Rc<List<T>>, x0:T) -> Option<i32>
{
    use List::*;
    match *l {
        Nil() => None,
        Cons(ref x,ref xs) =>
            if *x == x0 {
                Some(0)
            } else {
                find(xs.clone(),x0).map(|i| i+1)
            }
    }
}

fn insert<T:Clone>(l : Rc<List<T>>, i:i32, x0:T) -> List<T>
{
    use List::*;
    if i == 0 {
        Cons(x0,l.clone())
    } else {
        match *l {
            Nil() => Nil(),
            Cons(ref x, ref xs) => Cons(x.clone(),Rc::new(insert(xs.clone(), i-1,x0)))
        }
    }
}

fn remove<T:Clone>(l : Rc<List<T>>, i:i32) -> List<T>
{
    use List::*;
    if i == 0 {
        match *l {
            Nil() => Nil(),
            Cons(ref x, ref xs) => (**xs).clone()
        }
    } else {
        match *l {
            Nil() => Nil(),
            Cons(ref x, ref xs) => remove(xs.clone(),i)
        }
    }
}

fn main() {}
