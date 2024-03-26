// # 49 Group Anagrams

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ana_groups = HashMap::new();

    for s in strs {
        let mut key = s.chars().collect::<Vec<char>>();
        key.sort_unstable();
        let val = ana_groups.entry(key).or_insert(vec![]);
        val.push(s);
    }

    // ana_groups.values().cloned().collect()
    // -- `cloned` allows to clone Vec<String> values from the `Values` iter (from `values()` call)
    // -- difference from `clone` -> `clone` will only clone entire `ana_groups` vec with refs to values (from the original `ana_groups`)

    // ana_groups.values().map(|val| val.clone()).collect()
    // -- can map into the values and clone them via iteration over `Values` iter

    ana_groups.into_values().collect()
    // -- can `take` the values (`into_values`) to own inner data of Vec from `ana_groups` (clones values within for iteration/collection)
}
