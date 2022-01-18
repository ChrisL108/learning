// * nested loops
fn nested_loops() {
    let mut count = 0;

    'outer_loop: loop {
        let mut remaining = 10;
        println!("count = {}", count);
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }

        count += 1
    }

    println!("End count = {}", count);
}

fn data_types() {
    // let spaces = "   ";
    // let spaces = spaces.len();
    let num: u8 = 199;
    println!("num: {}", num);
    let my_char = 'S';
    println!("my_char: {}", my_char);
    let tup: (i32, f32, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("tup.1: {}", tup.1);
    let my_arr: [u8; 5] = [199, 3, 4, 55, 6];
    println!("my_arr[1]: {}", my_arr[1]);

    let a = [88, 57, 0, 400, 550, 300];
    let mut index = String::new();

    println!("Enter an array index");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Expect a number");
    let selected = a[index];
    println!("Selected element: {}", selected);
}