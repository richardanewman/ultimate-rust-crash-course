pub fn run_function_exercise() {
    let width = 4;
    let height = 7;
    let depth = 10;

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(w: i32, h: i32, d: i32) -> i32 {
    w * h * d
}
