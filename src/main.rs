pub mod goodstein;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() == 2);
    let arg: u64 = args[1].parse().unwrap();

    let mut a_number = goodstein::GNumber::new(arg);

    loop {
        if a_number.constituents.len() == 0 || a_number.sum_value == 0 {
            break;
        }

        dbg!(&a_number);
        a_number.do_goodstein();
        println!("the sum of the goodstein is {}", &a_number.sum_value);
        a_number.sum_over_power_factors();
        a_number.substract_one_and_restart();
    }
}
