mod buckshot;

fn main() {
    let mut round = buckshot::create_round();
    while round.bullets > 0 {
        println!("{:?}", round);
        round.shoot(true).unwrap();
    }
    println!("{:?}", round);
}
