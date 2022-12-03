
pub mod front_of_house;
pub mod back_of_house; 

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[allow(unused)]
fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
