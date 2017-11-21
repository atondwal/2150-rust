mod swap {

    pub fn swap<T:Copy>(x:&mut T,y:&mut T)
    {
        let tmp = *x;
        *x = *y;
        *y = tmp;
    }

}

fn main () {}
