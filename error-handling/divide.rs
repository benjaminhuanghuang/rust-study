#[derive[Debug]]
pub enum Res<T, E> {
    Thing(T),
    Error(E)
}



fn main() {
    let a = divide(4,5);
    let b = divide(4,0);

    printfln("a = {:?} b={:?}", a , b);
}


fn divide(a:i32, b:32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Can not divide by 0".to_string());
    }

    Res::Thing(a/b)
}