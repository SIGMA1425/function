fn main() {
    println!("double(x) is {}", double(23));
}

//fn(変数名:型)->返り値の型{}
fn double(x:u32) -> u32{
    x * 2
    //返り値はセミコロンを記述しない
}
