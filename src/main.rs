extern crate pretty_env_logger;

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    let vs: Vec<_> = std::env::args().skip(1).collect();
    let prog_name = get_prog_name();
    if vs.len() != 1 {
        eprintln!("Usage: {} n", prog_name);
        std::process::exit(1);
    }
    let n: u32 = match vs[0].parse() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("not a positive integer: {}", e);
            std::process::exit(1);
        }
    };

    println!("{}", fib(n));
}

fn get_prog_name() -> String {
    let prog_name = match std::env::current_exe() {
        Ok(path_buf) => path_buf,
        Err(e) => panic!("no current_exe: {}", e),
    };
    let prog_name = prog_name.file_name().unwrap().to_str().unwrap();
    prog_name.to_string()
}

fn fib(n: u32) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        debug!("{}", a);
        let t = b;
        b = a + b;
        a = t;
    }
    a
}

#[test]
fn test_fib() {
    let cases = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
        (49, 7778742049),
    ];
    for (i, expected) in cases {
        assert_eq!(expected, fib(i));
    }
}
