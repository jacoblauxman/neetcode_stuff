use neetcode_practice::factory_method_pattern::testing_vehicle_factory;
use neetcode_practice::insertion_sort;

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
}
