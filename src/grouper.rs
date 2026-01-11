use std::collections::HashMap;

pub fn select_grouping_unit(numbers: &[u32], threshold: usize) -> u32 {
    let units = [10000, 1000, 100, 10];

    for &unit in &units {
        let groups = count_groups(numbers, unit);
        if groups.values().all(|&count| count <= threshold) {
            return unit;
        }
    }

    10
}

fn count_groups(numbers: &[u32], unit: u32) -> HashMap<u32, usize> {
    let mut groups: HashMap<u32, usize> = HashMap::new();
    for &num in numbers {
        let group_start = (num / unit) * unit;
        *groups.entry(group_start).or_insert(0) += 1;
    }
    groups
}

pub fn compute_structure(
    numbers: &[u32],
    threshold: usize,
    current_path: &str,
) -> HashMap<u32, String> {
    let mut result = HashMap::new();

    if numbers.len() <= threshold {
        for &num in numbers {
            result.insert(num, current_path.to_string());
        }
        return result;
    }

    let unit = select_grouping_unit(numbers, threshold);

    let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
    for &num in numbers {
        let group_start = (num / unit) * unit;
        groups.entry(group_start).or_default().push(num);
    }

    for (group_start, group_numbers) in groups {
        let group_folder = if current_path.is_empty() {
            group_start.to_string()
        } else {
            format!("{}/{}", current_path, group_start)
        };

        if group_numbers.len() > threshold {
            let sub_structure = compute_structure(&group_numbers, threshold, &group_folder);
            result.extend(sub_structure);
        } else {
            for &num in &group_numbers {
                result.insert(num, group_folder.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder_for_number(number: u32, unit: u32) -> String {
        let group_start = (number / unit) * unit;
        group_start.to_string()
    }

    #[test]
    fn folder_for_number_1234_unit_1000() {
        assert_eq!(folder_for_number(1234, 1000), "1000");
    }

    #[test]
    fn folder_for_number_1999_unit_1000() {
        assert_eq!(folder_for_number(1999, 1000), "1000");
    }

    #[test]
    fn folder_for_number_2000_unit_1000() {
        assert_eq!(folder_for_number(2000, 1000), "2000");
    }

    #[test]
    fn folder_for_number_123_unit_100() {
        assert_eq!(folder_for_number(123, 100), "100");
    }

    #[test]
    fn folder_for_number_15_unit_10() {
        assert_eq!(folder_for_number(15, 10), "10");
    }

    #[test]
    fn select_unit_for_small_range() {
        let numbers: Vec<u32> = (1001..=1010).collect();
        assert_eq!(select_grouping_unit(&numbers, 20), 10000);
    }

    #[test]
    fn select_unit_for_spread_across_thousands() {
        let numbers: Vec<u32> = vec![1001, 2001, 3001, 4001, 5001];
        assert_eq!(select_grouping_unit(&numbers, 2), 1000);
    }

    #[test]
    fn compute_structure_flat_when_under_threshold() {
        let numbers: Vec<u32> = vec![1001, 1002, 1003, 1004, 1005];
        let result = compute_structure(&numbers, 20, "");

        for &num in &numbers {
            assert_eq!(result.get(&num), Some(&"".to_string()));
        }
    }

    #[test]
    fn compute_structure_splits_when_over_threshold() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1001 = result.get(&1001).unwrap();
        let path_1050 = result.get(&1050).unwrap();

        assert!(path_1001.contains("1000"));
        assert!(path_1050.contains("1050"));
        assert_ne!(path_1001, path_1050);
    }

    #[test]
    fn compute_structure_multiple_groups() {
        let mut numbers: Vec<u32> = (1001..=1025).collect();
        numbers.extend(2001..=2025);
        let result = compute_structure(&numbers, 20, "");

        assert!(result.get(&1001).unwrap().starts_with("1"));
        assert!(result.get(&2001).unwrap().starts_with("2"));
    }

    #[test]
    fn compute_structure_nested_split() {
        // 1000~1099에 100개 파일, threshold 20이면 1000/10, 1000/20, ... 으로 분할
        let numbers: Vec<u32> = (1000..1100).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1005 = result.get(&1005).unwrap();
        let path_1055 = result.get(&1055).unwrap();

        assert!(path_1005.contains("1000"));
        assert!(path_1055.contains("1050") || path_1055.contains("50"));
    }
}
