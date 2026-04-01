use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use file_search::search::SearchPattern;

/// 测试文本样本
fn get_sample_texts() -> Vec<(&'static str, &'static str)> {
    vec![
        // (描述, 文本内容)
        ("single_word", "error"),
        ("multi_word", "error occurred in system"),
        (
            "long_line",
            "This is a very long line of text containing error message somewhere in the middle of the string",
        ),
        ("unicode", "错误发生在系统中 error が発生しました"),
        ("special_chars", "error@example.com warning:123 debug#456"),
    ]
}

/// 基准：字面量单字符匹配
fn bench_literal_single_char(c: &mut Criterion) {
    let pattern = SearchPattern::from_pattern("e", false).expect("Failed to create pattern");
    let texts = get_sample_texts();

    c.bench_function("literal_single_char", |b| {
        b.iter(|| {
            for (_, text) in black_box(&texts) {
                let _ = pattern.is_match(black_box(text));
            }
        })
    });
}

/// 基准：字面量多词匹配
fn bench_literal_multi_word(c: &mut Criterion) {
    let pattern = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");
    let texts = get_sample_texts();

    c.bench_function("literal_multi_word", |b| {
        b.iter(|| {
            for (_, text) in black_box(&texts) {
                let _ = pattern.is_match(black_box(text));
            }
        })
    });
}

/// 基准：简单正则匹配 - 数字模式
fn bench_regex_simple_digits(c: &mut Criterion) {
    let pattern = SearchPattern::from_pattern(r"\d+", true).expect("Failed to create pattern");
    let texts = vec![
        ("no_match", "error message"),
        ("single_digit", "error 4"),
        ("multi_digit", "error 404 not found"),
        ("multiple_numbers", "error 404 warning 500 info 200"),
    ];

    c.bench_function("regex_simple_digits", |b| {
        b.iter(|| {
            for (_, text) in black_box(&texts) {
                let _ = pattern.is_match(black_box(text));
            }
        })
    });
}

/// 基准：复杂正则匹配 - 交替模式
fn bench_regex_alternation(c: &mut Criterion) {
    let pattern =
        SearchPattern::from_pattern("error|warn|info", true).expect("Failed to create pattern");
    let texts = vec![
        ("error", "error occurred"),
        ("warn", "warning message"),
        ("info", "info logged"),
        ("none", "debug message"),
        ("multiple", "error and warning both present"),
    ];

    c.bench_function("regex_alternation", |b| {
        b.iter(|| {
            for (_, text) in black_box(&texts) {
                let _ = pattern.is_match(black_box(text));
            }
        })
    });
}

/// 基准：高级正则 - 带修饰符
fn bench_regex_complex_pattern(c: &mut Criterion) {
    let pattern = SearchPattern::from_pattern(r"(?:error|warning)\s+\d{3}", true)
        .expect("Failed to create pattern");
    let texts = vec![
        ("match1", "error 404"),
        ("match2", "warning 503"),
        ("no_match1", "error abc"),
        ("no_match2", "information 404"),
        ("complex", "error 404 not found"),
    ];

    c.bench_function("regex_complex_pattern", |b| {
        b.iter(|| {
            for (_, text) in black_box(&texts) {
                let _ = pattern.is_match(black_box(text));
            }
        })
    });
}

/// 基准：字面量 vs 正则性能对比 - 相同数据
fn bench_literal_vs_regex_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("literal_vs_regex");
    group.sample_size(30); // 保守配置，快速完成

    let test_cases = vec![
        ("simple", "error"),
        ("complex", r"error|warning|fatal"),
        ("advanced", r"(?:error|warning|info)\s*\d+"),
    ];

    for (name, pattern_str) in test_cases {
        let literal = SearchPattern::from_pattern(pattern_str, false)
            .expect("Failed to create literal pattern");
        let regex =
            SearchPattern::from_pattern(pattern_str, true).expect("Failed to create regex pattern");

        let text = "error 404 warning 503 info 200";

        group.bench_with_input(BenchmarkId::new("literal", name), &name, |b, _| {
            b.iter(|| literal.is_match(black_box(text)))
        });

        group.bench_with_input(BenchmarkId::new("regex", name), &name, |b, _| {
            b.iter(|| regex.is_match(black_box(text)))
        });
    }

    group.finish();
}

/// 基准：模式创建成本
fn bench_pattern_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_creation");

    group.bench_function("create_literal", |b| {
        b.iter(|| SearchPattern::from_pattern(black_box("error"), false))
    });

    group.bench_function("create_regex_simple", |b| {
        b.iter(|| SearchPattern::from_pattern(black_box(r"\d+"), true))
    });

    group.bench_function("create_regex_complex", |b| {
        b.iter(|| SearchPattern::from_pattern(black_box(r"(?:error|warning|fatal)\s+\d{3}"), true))
    });

    group.finish();
}

/// 基准：匹配类型识别
fn bench_pattern_type_identification(c: &mut Criterion) {
    let literal = SearchPattern::from_pattern("error", false).expect("Failed to create pattern");
    let regex = SearchPattern::from_pattern(r"\d+", true).expect("Failed to create pattern");

    c.bench_function("get_pattern_type_literal", |b| {
        b.iter(|| literal.pattern_type())
    });

    c.bench_function("get_pattern_type_regex", |b| {
        b.iter(|| regex.pattern_type())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(30);
    targets =
        bench_literal_single_char,
        bench_literal_multi_word,
        bench_regex_simple_digits,
        bench_regex_alternation,
        bench_regex_complex_pattern,
        bench_literal_vs_regex_comparison,
        bench_pattern_creation,
        bench_pattern_type_identification
);

criterion_main!(benches);
