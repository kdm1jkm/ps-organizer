use std::collections::HashMap;

fn find_largest_unit(max_num: u32) -> u32 {
    if max_num == 0 {
        return 10;
    }
    let mut unit = 10;
    while unit * 10 <= max_num {
        unit *= 10;
    }
    unit
}

fn next_unit(current_unit: u32, threshold: usize) -> u32 {
    let next = current_unit / 10;
    if next <= threshold as u32 {
        threshold as u32
    } else {
        next
    }
}

fn group_by_unit(numbers: &[u32], unit: u32) -> HashMap<u32, Vec<u32>> {
    let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
    for &num in numbers {
        let group_start = (num / unit) * unit;
        groups.entry(group_start).or_default().push(num);
    }
    groups
}

fn digit_count(n: u32) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn format_folder_name(group_start: u32, width: usize) -> String {
    format!("{:0>width$}", group_start, width = width)
}

pub fn compute_structure(
    numbers: &[u32],
    threshold: usize,
    current_path: &str,
) -> HashMap<u32, String> {
    if numbers.len() <= threshold {
        return numbers
            .iter()
            .map(|&n| (n, current_path.to_string()))
            .collect();
    }

    let max_num = numbers.iter().max().copied().unwrap_or(0);
    let unit = find_largest_unit(max_num);

    compute_structure_with_unit(numbers, threshold, current_path, unit)
}

fn compute_structure_with_unit(
    numbers: &[u32],
    threshold: usize,
    current_path: &str,
    unit: u32,
) -> HashMap<u32, String> {
    if numbers.len() <= threshold {
        return numbers
            .iter()
            .map(|&n| (n, current_path.to_string()))
            .collect();
    }

    let groups = group_by_unit(numbers, unit);

    let mut sorted_groups: Vec<_> = groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut remaining = numbers.len();
    let mut groups_to_split: Vec<u32> = Vec::new();

    for (group_start, group_nums) in &sorted_groups {
        if remaining <= threshold {
            break;
        }
        groups_to_split.push(*group_start);
        remaining -= group_nums.len();
    }

    let max_end = groups_to_split
        .iter()
        .map(|&start| start + unit - 1)
        .max()
        .unwrap_or(0);
    let width = digit_count(max_end);

    let mut result = HashMap::new();

    for (group_start, group_nums) in sorted_groups {
        if groups_to_split.contains(&group_start) {
            let folder_name = format_folder_name(group_start, width);
            let new_path = if current_path.is_empty() {
                folder_name
            } else {
                format!("{}/{}", current_path, folder_name)
            };

            let next = next_unit(unit, threshold);
            if next < unit {
                let sub = compute_structure_with_unit(&group_nums, threshold, &new_path, next);
                result.extend(sub);
            } else {
                for num in group_nums {
                    result.insert(num, new_path.clone());
                }
            }
        } else {
            for num in group_nums {
                result.insert(num, current_path.to_string());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_largest_unit_basic() {
        assert_eq!(find_largest_unit(0), 10);
        assert_eq!(find_largest_unit(9), 10);
        assert_eq!(find_largest_unit(99), 10);
        assert_eq!(find_largest_unit(100), 100);
        assert_eq!(find_largest_unit(999), 100);
        assert_eq!(find_largest_unit(1000), 1000);
        assert_eq!(find_largest_unit(9999), 1000);
        assert_eq!(find_largest_unit(10000), 10000);
        assert_eq!(find_largest_unit(45000), 10000);
        assert_eq!(find_largest_unit(100000), 100000);
        assert_eq!(find_largest_unit(1234567), 1000000);
    }

    #[test]
    fn next_unit_basic() {
        assert_eq!(next_unit(1000, 20), 100);
        assert_eq!(next_unit(100, 20), 20);
        assert_eq!(next_unit(100, 50), 50);
        assert_eq!(next_unit(1000, 333), 333);
        assert_eq!(next_unit(10000, 333), 1000);
    }

    #[test]
    fn digit_count_basic() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(99), 2);
        assert_eq!(digit_count(999), 3);
        assert_eq!(digit_count(9999), 4);
        assert_eq!(digit_count(99999), 5);
    }

    #[test]
    fn format_folder_name_basic() {
        assert_eq!(format_folder_name(0, 5), "00000");
        assert_eq!(format_folder_name(1000, 4), "1000");
        assert_eq!(format_folder_name(100, 4), "0100");
        assert_eq!(format_folder_name(30000, 5), "30000");
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
    fn compute_structure_nested_when_dense() {
        let numbers: Vec<u32> = (1000..=1050).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1000 = result.get(&1000).unwrap();
        let path_1025 = result.get(&1025).unwrap();

        assert!(path_1000.starts_with("1000"));
        assert!(path_1025.starts_with("1020") || path_1025.starts_with("1000"));
    }

    #[test]
    fn compute_structure_mixed_density_with_padding() {
        let mut numbers: Vec<u32> = (100..=105).collect();
        numbers.extend(1000..=1005);
        numbers.extend(30000..=30002);

        let result = compute_structure(&numbers, 10, "");

        let path_100 = result.get(&100).unwrap();
        let path_1000 = result.get(&1000).unwrap();
        let path_30000 = result.get(&30000).unwrap();

        assert!(!path_100.is_empty(), "path_100 should have folder");
        assert!(!path_1000.is_empty(), "path_1000 should have folder");
        assert_eq!(path_30000, "");
    }

    #[test]
    fn compute_structure_threshold_333() {
        let numbers: Vec<u32> = (10000..=11000).collect();
        let result = compute_structure(&numbers, 333, "");

        let path_10000 = result.get(&10000).unwrap();
        let path_10500 = result.get(&10500).unwrap();

        assert!(path_10000.starts_with("10000"));
        assert!(path_10500.starts_with("10000"));
    }

    #[test]
    fn folders_use_start_number() {
        let numbers: Vec<u32> = (1000..=1100).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1000 = result.get(&1000).unwrap();
        assert!(path_1000.chars().next().unwrap().is_ascii_digit());
    }

    #[test]
    fn same_level_folders_have_same_length() {
        let mut numbers: Vec<u32> = (1000..=1100).collect();
        numbers.extend(30000..=30100);

        let result = compute_structure(&numbers, 20, "");

        let path_1000 = result.get(&1000).unwrap();
        let path_30000 = result.get(&30000).unwrap();

        assert!(!path_1000.is_empty());
        assert!(!path_30000.is_empty());

        let root_1000 = path_1000.split('/').next().unwrap();
        let root_30000 = path_30000.split('/').next().unwrap();

        assert_eq!(root_1000.len(), root_30000.len());
    }
}
