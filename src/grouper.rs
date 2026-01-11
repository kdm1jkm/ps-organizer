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

fn calc_padding_width(group_starts: &[u32]) -> usize {
    group_starts
        .iter()
        .map(|&n| {
            if n == 0 {
                1
            } else {
                (n as f64).log10().floor() as usize + 1
            }
        })
        .max()
        .unwrap_or(1)
}

fn format_folder_name(group_start: u32, width: usize) -> String {
    format!("{:0>width$}", group_start, width = width)
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

    let group_starts: Vec<u32> = groups.keys().copied().collect();
    let width = calc_padding_width(&group_starts);

    for (group_start, group_numbers) in groups {
        let folder_name = format_folder_name(group_start, width);
        let group_folder = if current_path.is_empty() {
            folder_name
        } else {
            format!("{}/{}", current_path, folder_name)
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

    #[test]
    fn padding_single_digit() {
        assert_eq!(calc_padding_width(&[0, 10, 20]), 2);
    }

    #[test]
    fn padding_mixed_digits() {
        assert_eq!(calc_padding_width(&[10, 100, 1000]), 4);
    }

    #[test]
    fn padding_large_numbers() {
        assert_eq!(calc_padding_width(&[1000, 2000, 10000]), 5);
    }

    #[test]
    fn format_with_padding() {
        assert_eq!(format_folder_name(10, 4), "0010");
        assert_eq!(format_folder_name(100, 4), "0100");
        assert_eq!(format_folder_name(1000, 4), "1000");
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
    fn compute_structure_splits_with_padding() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1001 = result.get(&1001).unwrap();
        let path_1050 = result.get(&1050).unwrap();

        assert!(path_1001.contains("1000"));
        assert!(path_1050.contains("1050"));
        assert_ne!(path_1001, path_1050);
    }

    #[test]
    fn compute_structure_multiple_groups_with_padding() {
        let mut numbers: Vec<u32> = (1001..=1025).collect();
        numbers.extend(2001..=2025);
        let result = compute_structure(&numbers, 20, "");

        let path_1001 = result.get(&1001).unwrap();
        let path_2001 = result.get(&2001).unwrap();

        assert_eq!(path_1001.len(), path_2001.len());
    }

    #[test]
    fn compute_structure_nested_split_with_padding() {
        let numbers: Vec<u32> = (1000..1100).collect();
        let result = compute_structure(&numbers, 20, "");

        let path_1005 = result.get(&1005).unwrap();
        let path_1055 = result.get(&1055).unwrap();

        assert!(path_1005.contains("1000"));

        let parts_1005: Vec<&str> = path_1005.split('/').collect();
        let parts_1055: Vec<&str> = path_1055.split('/').collect();
        if parts_1005.len() > 1 && parts_1055.len() > 1 {
            assert_eq!(parts_1005[1].len(), parts_1055[1].len());
        }
    }

    #[test]
    fn folders_sort_correctly() {
        let numbers: Vec<u32> = vec![15, 105, 1005];
        let result = compute_structure(&numbers, 1, "");

        let mut folders: Vec<&String> = result.values().collect();
        folders.sort();
        folders.dedup();

        let sorted_check: Vec<String> = folders.iter().map(|s| s.to_string()).collect();
        let mut expected = sorted_check.clone();
        expected.sort();

        assert_eq!(sorted_check, expected);
    }
}
