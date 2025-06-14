impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let nas = num.to_string();
        nas.chars()
            .find(|&c| c != '9')
            .map_or(num, |c| nas.replace(c, "9").parse().unwrap())
            - nas.replace(nas.get(0..1).unwrap(), "0").parse::<i32>().unwrap()
    }
}
