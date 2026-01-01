use std::collections::HashMap;

fn main() {
    let input = tools::read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    let mut total_answer = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse the line
        let parts: Vec<&str> = line.split_whitespace().collect();
        // parts[0] is the label (ignored), parts[1..last] are coeffs, parts[last] is goal
        if parts.len() < 2 { continue; }

        let goal_str = parts.last().unwrap();
        let goal = parse_vector(goal_str);

        let coeff_strs = &parts[1..parts.len() - 1];
        let raw_coeffs: Vec<Vec<usize>> = coeff_strs.iter()
            .map(|s| parse_indices(s))
            .collect();

        // Convert raw indices to dense vectors (0s and 1s) based on goal dimension
        let dim = goal.len();
        let coeffs: Vec<Vec<i64>> = raw_coeffs.iter().map(|indices| {
            let mut vec = vec![0; dim];
            for &idx in indices {
                if idx < dim {
                    vec[idx] = 1;
                }
            }
            vec
        }).collect();

        let subanswer = solve_single(&coeffs, &goal);

        println!("Line {}/{}: answer {}", i + 1, lines.len(), subanswer);
        total_answer += subanswer;
    }

    println!("{}", total_answer);
}

// -----------------------------------------------------------------------------
// Logic
// -----------------------------------------------------------------------------

fn solve_single(coeffs: &[Vec<i64>], goal: &[i64]) -> u64 {
    let patterns = generate_patterns(coeffs);
    let mut memo = HashMap::new();

    let result = solve_recursive(goal, &patterns, &mut memo);

    // If result is effectively infinite (no solution), treat as 0 or panic
    // depending on AoC rules. The Python code added the infinity value,
    // but usually, we want the sum of valid costs.
    // Here we return the result if found, otherwise we assume 0 for "impossible".
    if result >= u64::MAX / 2 { 0 } else { result }
}

fn solve_recursive(
    goal: &[i64],
    patterns: &HashMap<Vec<i64>, u64>,
    memo: &mut HashMap<Vec<i64>, u64>
) -> u64 {
    // Base case: if goal is all 0s, cost is 0
    if goal.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&val) = memo.get(goal) {
        return val;
    }

    let mut min_cost = u64::MAX;

    for (pattern, pattern_cost) in patterns {
        // Check conditions:
        // 1. pattern[i] <= goal[i]
        // 2. pattern[i] % 2 == goal[i] % 2 (Same parity)
        let is_compatible = goal.iter().zip(pattern.iter()).all(|(&g, &p)| {
            p <= g && (g - p) % 2 == 0
        });

        if is_compatible {
            // Calculate new goal: (goal - pattern) / 2
            let new_goal: Vec<i64> = goal.iter().zip(pattern.iter())
                .map(|(&g, &p)| (g - p) / 2)
                .collect();

            let sub_res = solve_recursive(&new_goal, patterns, memo);

            if sub_res != u64::MAX {
                // formula: cost + 2 * sub_cost
                // Check for overflow just in case
                if let Some(doubled) = sub_res.checked_mul(2) {
                    if let Some(total) = pattern_cost.checked_add(doubled) {
                        if total < min_cost {
                            min_cost = total;
                        }
                    }
                }
            }
        }
    }

    memo.insert(goal.to_vec(), min_cost);
    min_cost
}

/// Generates a map of {SumVector -> MinButtonsCount}
/// Replaces itertools.combinations by iterating 0..2^N (Power Set)
fn generate_patterns(coeffs: &[Vec<i64>]) -> HashMap<Vec<i64>, u64> {
    let mut out: HashMap<Vec<i64>, u64> = HashMap::new();
    let num_buttons = coeffs.len();
    let num_vars = if num_buttons > 0 { coeffs[0].len() } else { 0 };

    // Iterate all subsets via bitmask
    // 1 << N is 2^N
    let limit = 1 << num_buttons;

    // Iterating by length (population count) ensures we find smallest cost first
    // if we want to mimic the Python structure strictly, but since we put everything
    // into a map, we can just iterate linear and use min().
    for mask in 0..limit {
        let mut current_sum = vec![0; num_vars];
        let mut cost = 0;

        for i in 0..num_buttons {
            if (mask >> i) & 1 == 1 {
                cost += 1;
                for (k, val) in coeffs[i].iter().enumerate() {
                    current_sum[k] += val;
                }
            }
        }

        // Store the minimum cost for this specific vector pattern
        out.entry(current_sum)
            .and_modify(|c| *c = (*c).min(cost))
            .or_insert(cost);
    }

    out
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn parse_vector(s: &str) -> Vec<i64> {
    // Expects formats like "(1,2,3)" or "{1,2,3}"; be robust to spaces
    s.trim()
        .trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}')
        .split(',')
        .map(|n| n.trim())
        .filter_map(|n| n.parse::<i64>().ok())
        .collect()
}

fn parse_indices(s: &str) -> Vec<usize> {
    // Expects "(1,2,3)" -> returns [1, 2, 3]
    s.trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|n| n.trim())
        .filter_map(|n| n.parse::<usize>().ok())
        .collect()
}
