struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Construct MyStruct {}", n);
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct from Rust: n = {}", self.n);
    }
}

struct HasDroppables {
    x: MyStruct,
}

fn move_me(s: MyStruct) {
    println!("Move MyStruct from Rust: n = {}", s.n);
    println!("Func finish");
}

fn main() {
    println!("Start main");
    {
        let my_struct = MyStruct::new(3);
        assert_eq!(my_struct.n, 3);
    }

    let s = MyStruct::new(5);
    move_me(s);

    {
        let d = HasDroppables { x: MyStruct::new(7) };
    }

    println!("Finish main");
}
