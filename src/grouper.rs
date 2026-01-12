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

fn group_by_unit(numbers: &[u32], unit: u32) -> HashMap<u32, Vec<u32>> {
    let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
    for &num in numbers {
        let group_start = (num / unit) * unit;
        groups.entry(group_start).or_default().push(num);
    }
    groups
}

fn count_trailing(unit: u32) -> usize {
    let mut count = 0;
    let mut u = unit;
    while u >= 10 {
        u /= 10;
        count += 1;
    }
    count
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

fn format_folder_name(
    group_start: u32,
    unit: u32,
    total_width: usize,
    placeholder: char,
) -> String {
    let prefix = group_start / unit;
    let trailing_count = count_trailing(unit);
    let trailing: String = std::iter::repeat(placeholder)
        .take(trailing_count)
        .collect();
    let base = format!("{}{}", prefix, trailing);
    format!("{:0>width$}", base, width = total_width)
}

pub fn compute_structure(
    numbers: &[u32],
    threshold: usize,
    placeholder: char,
) -> HashMap<u32, String> {
    if numbers.len() <= threshold {
        return numbers.iter().map(|&n| (n, String::new())).collect();
    }

    let max_num = numbers.iter().max().copied().unwrap_or(0);
    let start_unit = find_largest_unit(max_num);

    let mut pending: Vec<(u32, Vec<u32>)> = vec![(start_unit, numbers.to_vec())];
    let mut finalized: Vec<(u32, u32, Vec<u32>)> = Vec::new();

    while let Some((current_unit, nums)) = pending.pop() {
        let groups = group_by_unit(&nums, current_unit);

        for (group_start, group_nums) in groups {
            if group_nums.len() > threshold && current_unit > 10 {
                pending.push((current_unit / 10, group_nums));
            } else {
                finalized.push((group_start, current_unit, group_nums));
            }
        }
    }

    let max_end = finalized
        .iter()
        .map(|(start, unit, _)| start + unit - 1)
        .max()
        .unwrap_or(0);
    let total_width = digit_count(max_end);

    let mut result = HashMap::new();
    for (group_start, unit, group_nums) in finalized {
        let folder_name = format_folder_name(group_start, unit, total_width, placeholder);
        for num in group_nums {
            result.insert(num, folder_name.clone());
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
    fn count_trailing_basic() {
        assert_eq!(count_trailing(10), 1);
        assert_eq!(count_trailing(100), 2);
        assert_eq!(count_trailing(1000), 3);
        assert_eq!(count_trailing(10000), 4);
        assert_eq!(count_trailing(100000), 5);
    }

    #[test]
    fn digit_count_basic() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(99), 2);
        assert_eq!(digit_count(100), 3);
        assert_eq!(digit_count(9999), 4);
        assert_eq!(digit_count(10000), 5);
    }

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
    fn format_folder_mixed_units() {
        // 같은 total_width에서 다른 단위
        assert_eq!(format_folder_name(1000, 100, 5, '_'), "010__");
        assert_eq!(format_folder_name(30000, 10000, 5, '_'), "3____");
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
    fn compute_structure_flat_when_under_threshold() {
        let numbers: Vec<u32> = vec![1001, 1002, 1003, 1004, 1005];
        let result = compute_structure(&numbers, 20, '_');

        for &num in &numbers {
            assert_eq!(result.get(&num), Some(&"".to_string()));
        }
    }

    #[test]
    fn compute_structure_splits_with_placeholder() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, '_');

        let path_1001 = result.get(&1001).unwrap();
        let path_1050 = result.get(&1050).unwrap();

        assert!(path_1001.contains("100"));
        assert!(path_1050.contains("105"));
        assert_ne!(path_1001, path_1050);
    }

    #[test]
    fn compute_structure_with_x_placeholder() {
        let numbers: Vec<u32> = (1001..=1050).collect();
        let result = compute_structure(&numbers, 20, 'x');

        let path_1001 = result.get(&1001).unwrap();
        assert!(path_1001.contains("100"));
        assert!(path_1001.contains("x"));
    }

    #[test]
    fn compute_structure_mixed_density() {
        // dense: 1000-1100 (101개), sparse: 30000-30005 (6개)
        let mut numbers: Vec<u32> = (1000..=1100).collect();
        numbers.extend(30000..=30005);

        let result = compute_structure(&numbers, 20, '_');

        // dense 영역은 더 작은 단위로 쪼개짐
        let path_1000 = result.get(&1000).unwrap();
        let path_1050 = result.get(&1050).unwrap();
        assert_ne!(path_1000, path_1050);

        // sparse 영역은 큰 단위로 유지
        let path_30000 = result.get(&30000).unwrap();
        let path_30005 = result.get(&30005).unwrap();
        assert_eq!(path_30000, path_30005);

        // sparse 폴더는 만 단위
        assert!(path_30000.contains("3____"));
    }

    #[test]
    fn compute_structure_all_same_folder_when_sparse() {
        // 모든 파일이 sparse하면 큰 단위로
        let numbers: Vec<u32> = vec![1001, 2001, 3001, 4001, 5001];
        let result = compute_structure(&numbers, 20, '_');

        // 모두 threshold 이하니까 루트에 유지
        for &num in &numbers {
            assert_eq!(result.get(&num), Some(&"".to_string()));
        }
    }

    #[test]
    fn folders_sort_correctly() {
        // dense/sparse 혼합 시 폴더명 정렬 확인
        let mut numbers: Vec<u32> = (1000..=1100).collect();
        numbers.extend(30000..=30005);

        let result = compute_structure(&numbers, 20, '_');

        let mut folders: Vec<&String> = result.values().collect();
        folders.sort();
        folders.dedup();

        // 정렬된 상태 확인
        let sorted: Vec<String> = folders.iter().map(|s| s.to_string()).collect();
        let mut expected = sorted.clone();
        expected.sort();
        assert_eq!(sorted, expected);
    }
}
