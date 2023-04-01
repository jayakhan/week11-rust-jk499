use std::collections::HashSet;

fn main() {
    let word = "TEST";

    let mut combinations = HashSet::new();

    for i in 0..4 {
        for j in 0..4 {
            if j == i {
                continue;
            }
            for k in 0..4 {
                if k == i || k == j {
                    continue;
                }
                for l in 0..4 {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    let combination = vec![
                        word.chars().nth(i).unwrap(),
                        word.chars().nth(j).unwrap(),
                        word.chars().nth(k).unwrap(),
                        word.chars().nth(l).unwrap(),
                    ];
                    combinations.insert(combination);
                }
            }
        }
    }

    for combination in combinations {
        let word = combination.iter().collect::<String>();
        println!("{}", word);
    }
}
