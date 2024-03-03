use neetcode_practice::factory_method_pattern::testing_vehicle_factory;
use neetcode_practice::insertion_sort;
// use neetcode_practice::squares_of_a_sorted_array::sorted_squares;

fn main() {
    println!("\n'ello, werld\n\n");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let insertion_sort_test =
        insertion_sort(vec![(3, "cat"), (5, "banana"), (1, "fred"), (2, "ben")]);
    println!(
        "RESULTS FROM INSERTION SORT RUN: \n {:?}",
        insertion_sort_test
    );

    println!("\n-- -- -- -- -- -- -- -- \n");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("factory method pattern results: \n");
    testing_vehicle_factory();

    // println!("\n -- -- -- -- -- -- -- --\n");
    // let mut test = vec![-22, -14, -5, -3, -2, 1, 2, 4, 5, 9, 13, 21, 23];
    // println!("Here is test arr: {:?}", test);
    // println!("\n:: aaaand here's results of running `sorted_squares` (in place!) ::\n");
    // // println!("{:?}", sorted_squares(&mut test));
    // sorted_squares(&mut test);
    // println!("{:?}", test);
}
