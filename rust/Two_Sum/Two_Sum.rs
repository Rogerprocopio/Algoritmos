use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_indices: HashMap<i32, i32> = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num; //Procurando o numero que somado complementa o target

        if let Some(&complement_index) = num_indices.get(&complement) {
            return vec![complement_index, index as i32];
        }

        num_indices.insert(num, index as i32);
    }

    // Retorna um vetor vazio se não encontrar uma solução
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(nums, target);
    println!("Indices dos números que somam {}: {:?}", target, result);
}

