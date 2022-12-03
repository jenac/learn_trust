#[allow(unused)]
pub fn fix_incorrect_order() {
    println!("Fixing incorrect order");
    cook_order();
    super::front_of_house::serving::serve_order();
    crate::front_of_house::serving::serve_order();
}
fn cook_order() {}
