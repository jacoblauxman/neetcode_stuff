pub fn insertion_sort(mut pairs: Vec<(u32, &str)>) -> Vec<Vec<(u32, &str)>> {
    let mut res = Vec::new();

    for i in 0..pairs.len() {
        let mut j = i as i32 - 1;

        while j >= 0 && pairs[j as usize].0 > pairs[(j + 1) as usize].0 {
            pairs.swap(j as usize, (j + 1) as usize);
            j = j - 1;
        }

        res.push(pairs.clone())
    }

    res
}
