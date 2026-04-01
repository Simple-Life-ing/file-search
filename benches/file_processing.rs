use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use file_search::search::{SearchPattern, search_in_file};
use std::fs;
use tempfile::TempDir;

/// 生成测试文件内容
fn generate_test_content(line_count: usize, pattern: &str) -> String {
    let mut content = String::new();
    let patterns = vec!["error", "warning", "info", "debug"];

    for i in 0..line_count {
        let p = patterns[i % patterns.len()];
        if i % 20 == 0 {
            content.push_str(&format!("line {}: {} message found\n", i, pattern));
        } else {
            content.push_str(&format!("line {}: {} event occurred\n", i, p));
        }
    }
    content
}

/// 创建临时文件并返回路径
fn create_temp_file(line_count: usize, pattern: &str) -> (TempDir, String) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("test.txt");
    let content = generate_test_content(line_count, pattern);
    fs::write(&file_path, content).expect("Failed to write file");
    let path_str = file_path.to_string_lossy().to_string();
    (dir, path_str)
}

/// 基准：搜索小文件（字面量）
fn bench_search_small_file_literal(c: &mut Criterion) {
    let (dir, path) = create_temp_file(100, "error");
    let pattern = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");

    c.bench_function("search_small_file_100_literal", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：搜索中等文件（字面量）
fn bench_search_medium_file_literal(c: &mut Criterion) {
    let (dir, path) = create_temp_file(10_000, "error");
    let pattern = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");

    c.bench_function("search_medium_file_10k_literal", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：搜索大文件（字面量）
fn bench_search_large_file_literal(c: &mut Criterion) {
    let (dir, path) = create_temp_file(100_000, "error");
    let pattern = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");

    c.bench_function("search_large_file_100k_literal", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：搜索小文件（正则）
fn bench_search_small_file_regex(c: &mut Criterion) {
    let (dir, path) = create_temp_file(100, "regex");
    let pattern =
        SearchPattern::from_pattern(r"error|warning", true).expect("Failed to create pattern");

    c.bench_function("search_small_file_100_regex", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：搜索中等文件（正则）
fn bench_search_medium_file_regex(c: &mut Criterion) {
    let (dir, path) = create_temp_file(10_000, "regex");
    let pattern =
        SearchPattern::from_pattern(r"error|warning", true).expect("Failed to create pattern");

    c.bench_function("search_medium_file_10k_regex", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：搜索大文件（正则）
fn bench_search_large_file_regex(c: &mut Criterion) {
    let (dir, path) = create_temp_file(100_000, "regex");
    let pattern =
        SearchPattern::from_pattern(r"error|warning", true).expect("Failed to create pattern");

    c.bench_function("search_large_file_100k_regex", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&pattern)))
    });

    drop(dir);
}

/// 基准：不同匹配率对性能的影响
fn bench_match_rate_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_rate_impact");
    group.sample_size(20); // 减少样本数，加快测试

    for (rate_name, match_rate) in &[("0%", 0), ("10%", 10), ("50%", 50), ("100%", 100)] {
        let (dir, path) = create_temp_file(1000, "test");
        let pattern = if *match_rate == 0 {
            SearchPattern::from_pattern("nomatch", false).expect("Failed to create pattern")
        } else if *match_rate == 100 {
            SearchPattern::from_pattern("event", false).expect("Failed to create pattern")
        } else {
            SearchPattern::from_pattern("error", false).expect("Failed to create pattern")
        };

        group.bench_with_input(
            BenchmarkId::new("match_rate", rate_name),
            rate_name,
            |b, _| b.iter(|| search_in_file(black_box(&path), black_box(&pattern))),
        );

        drop(dir);
    }

    group.finish();
}

/// 基准：文件大小vs性能线性关系
fn bench_file_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_size_scaling");
    group.sample_size(20);

    for line_count in &[100, 1_000, 10_000] {
        let (dir, path) = create_temp_file(*line_count, "error");
        let pattern =
            SearchPattern::from_pattern("error", false).expect("Failed to create pattern");

        group.bench_with_input(
            BenchmarkId::new("file_size", format!("{}_lines", line_count)),
            line_count,
            |b, _| b.iter(|| search_in_file(black_box(&path), black_box(&pattern))),
        );

        drop(dir);
    }

    group.finish();
}

/// 基准：字面量 vs 正则在文件处理中的性能对比
fn bench_literal_vs_regex_in_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_literal_vs_regex");
    group.sample_size(20);

    let (dir, path) = create_temp_file(10_000, "comparison");
    let literal = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");
    let regex = SearchPattern::from_pattern(r"error", true).expect("Failed to create pattern");

    group.bench_function("file_literal_simple", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&literal)))
    });

    group.bench_function("file_regex_simple", |b| {
        b.iter(|| search_in_file(black_box(&path), black_box(&regex)))
    });

    drop(dir);
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(20);
    targets =
        bench_search_small_file_literal,
        bench_search_medium_file_literal,
        bench_search_large_file_literal,
        bench_search_small_file_regex,
        bench_search_medium_file_regex,
        bench_search_large_file_regex,
        bench_match_rate_impact,
        bench_file_size_scaling,
        bench_literal_vs_regex_in_file
);

criterion_main!(benches);
