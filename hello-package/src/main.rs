fn main() {

    use hello_package::add;
    let s = add(2, 3);
    println!("Hello Package - {}", s);

    use hello_package::front_of_house::hosting::add_to_waitlist;
    use hello_package::front_of_house::serving::serve_order;
    use hello_package::back_of_house::fix_incorrect_order;
    add_to_waitlist();
    fix_incorrect_order();
    serve_order();

}