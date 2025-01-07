use gcd_core::gcd::compute_gcd;
struct CliArgs{
    function: String,
    m: u64,
    n: u64
}

fn main() {
    let function = std::env::args().nth(1).expect("no pattern given");
    let m = std::env::args().nth(2).expect("No m value given");
    let n = std::env::args().nth(3).expect("No n value given");

    let args = CliArgs{
        function,
        m: m.parse().unwrap(),
        n: n.parse().unwrap()
    };
    let gcd = compute_gcd(args.n, args.m);
    println!("The gcd the numbers {} and {} is {}", args.m, args.n, gcd);

}
