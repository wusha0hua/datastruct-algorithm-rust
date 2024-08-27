trait Mut<T, R> {
    fn recu(&self, o: & dyn Mut<T, R>, t: T) -> R;
}

impl<T, R, F> Mut<T, R> for F
where
    F: Fn(&dyn Mut<T, R>, T) -> R,
{
    
    fn recu(&self, o: & dyn Mut<T, R>, t: T) -> R {
        self(o, t)
    }
}

// 定义Y组合子：Y = λf. (λx.x x) (λx. f(x x))
pub fn y<T, R, F>(f: &F, t: T) -> R
where
    F: Fn(&dyn Fn(T) -> R, T) -> R,
{
    (&|x: &dyn Mut<T, R>, t| x.recu(x, t))(&|x: & dyn Mut<T, R>, t| f(&|t| x.recu(x, t), t), t)
}

