fn main() {
    println!("I am the help in hello-package");
    use hello_package::add;
    println!("{}+{}={}", 18, 188, add(18, 188));
}
