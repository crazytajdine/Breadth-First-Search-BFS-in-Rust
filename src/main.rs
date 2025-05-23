use math::find_k;

mod math;

fn main() {
    let r = ["da", "d"];

    if let Some(res) = find_k(&r, &"da") {
        println!("{}", res);
    }
}
