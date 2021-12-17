
use djangohashers::{make_password, check_password, };

fn main() {

    let password = "password";

    println!();
    let dj_hash = make_password(password);
    let valid_hash = check_password(password, &dj_hash);

    println!("{:?}", dj_hash);
    println!("{:?}", valid_hash);



}