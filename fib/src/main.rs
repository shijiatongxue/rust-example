fn fib_loop(n: u32) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    loop {
        print(&mut a, &mut b);

        i += 1;
        if i >= n {
            break;
        }
    }
}

fn fib_for(n: u32) {
    let mut a = 1;
    let mut b = 1;
    for _ in 2..n {
        print(&mut a, &mut b);
    }
}

fn fib_while(n: u32) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;

    while i < n {
        print(&mut a, &mut b);

        i += 1;
    }
}

fn print(a: &mut u32, b: &mut u32) {
    let tmp = *a;
    *a = *b;
    *b = tmp + *b;
    println!("next value is {}", b);
}

fn main() {
    let n = 10;
    fib_for(n);
    fib_loop(n);
    fib_while(n);
}
