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

fn calc_total_width(group_starts: &[u32], unit: u32) -> usize {
    let max_group = group_starts.iter().max().copied().unwrap_or(0);
    let max_end = max_group + unit - 1;
    if max_end == 0 {
        1
    } else {
        (max_end as f64).log10().floor() as usize + 1
    }
}

fn format_folder_name(
    group_start: u32,
    unit: u32,
    total_width: usize,
    placeholder: char,
) -> String {
    let prefix = group_start / unit;
    let trailing_count = (unit as f64).log10().floor() as usize;
    let trailing: String = std::iter::repeat(placeholder)
        .take(trailing_count)
        .collect();
    let base = format!("{}{}", prefix, trailing);
    format!("{:0>width$}", base, width = total_width)
}

pub fn compute_structure(
    numbers: &[u32],
    threshold: usize,
    current_path: &str,
    placeholder: char,
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
    let width = calc_total_width(&group_starts, unit);

    for (group_start, group_numbers) in groups {
        let folder_name = format_folder_name(group_start, unit, width, placeholder);
        let group_folder = if current_path.is_empty() {
            folder_name
        } else {
            format!("{}/{}", current_path, folder_name)
        };

        if group_numbers.len() > threshold {
            let sub_structure =
                compute_structure(&group_numbers, threshold, &group_folder, placeholder);
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
    fn format_folder_basic() {
        assert_eq!(format_folder_name(1000, 1000, 4, '_'), "1___");
        assert_eq!(format_folder_name(2000, 1000, 4, '_'), "2___");
    }

    #[test]
    fn format_folder_with_front_padding() {
        assert_eq!(format_folder_name(1000, 1000, 5, '_'), "01___");
        assert_eq!(format_folder_name(10000, 1000, 5, '_'), "10___");
    }

    #[test]
    fn format_folder_unit_100() {
        assert_eq!(format_folder_name(100, 100, 3, '_'), "1__");
        assert_eq!(format_folder_name(200, 100, 3, '_'), "2__");
    }

    #[test]
    fn format_folder_unit_10() {
        assert_eq!(format_folder_name(10, 10, 2, '_'), "1_");
        assert_eq!(format_folder_name(20, 10, 2, '_'), "2_");
    }

    #[test]
    fn format_folder_with_x_placeholder() {
        assert_eq!(format_folder_name(1000, 1000, 4, 'x'), "1xxx");
        assert_eq!(format_folder_name(100, 100, 3, 'x'), "1xx");
        assert_eq!(format_folder_name(10, 10, 2, 'x'), "1x");
    }

    #[test]
    fn calc_width_for_groups() {
        assert_eq!(calc_total_width(&[1000, 2000], 1000), 4);
        assert_eq!(calc_total_width(&[1000, 9000], 1000), 4);
        assert_eq!(calc_total_width(&[1000, 10000], 1000), 5);
        assert_eq!(calc_total_width(&[10, 20, 90], 10), 2);
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
        let result = compute_structure(&numbers, 20, "", '_');

        for &num in &numbers {
            assert_eq!(result.get(&num), Some(&"".to_string()));
        }
    }

    #[test]
    fn compute_structure_splits_with_placeholder() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, "", '_');

        let path_1001 = result.get(&1001).unwrap();
        let path_1050 = result.get(&1050).unwrap();

        assert!(path_1001.contains("100_"));
        assert!(path_1050.contains("105_"));
        assert_ne!(path_1001, path_1050);
    }

    #[test]
    fn compute_structure_with_x_placeholder() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, "", 'x');

        let path_1001 = result.get(&1001).unwrap();
        assert!(path_1001.contains("100x"));
    }

    #[test]
    fn folders_sort_correctly_with_placeholder() {
        let numbers: Vec<u32> = vec![15, 105, 1005];
        let result = compute_structure(&numbers, 1, "", '_');

        let mut folders: Vec<&String> = result.values().collect();
        folders.sort();
        folders.dedup();

        let sorted_check: Vec<String> = folders.iter().map(|s| s.to_string()).collect();
        let mut expected = sorted_check.clone();
        expected.sort();

        assert_eq!(sorted_check, expected);
    }
}
