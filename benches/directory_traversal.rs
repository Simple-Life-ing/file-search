use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use file_search::config::RuntimeConfig;
use file_search::executor::run;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// 创建具有指定文件数量的临时目录
fn create_test_directory(file_count: usize) -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let dir_path = dir.path().to_path_buf();
    
    let content = "error message\nwarning alert\ninfo log\ndebug trace\n";
    
    for i in 0..file_count {
        let file_path = dir_path.join(format!("file_{:06}.txt", i));
        fs::write(&file_path, content).expect("Failed to write file");
    }
    
    (dir, dir_path)
}

/// 创建嵌套目录结构
fn create_nested_directory(depth: usize, files_per_dir: usize) -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let root = dir.path().to_path_buf();
    let content = "error message\nwarning alert\ninfo log\n";
    
    // 创建嵌套目录结构
    fn create_nested(base: &PathBuf, depth: usize, files_per_dir: usize, content: &str) {
        if depth == 0 {
            return;
        }
        
        for i in 0..files_per_dir {
            let file_path = base.join(format!("file_{:03}.txt", i));
            let _ = fs::write(&file_path, content);
        }
        
        let next_dir = base.join(format!("subdir_{}", depth));
        let _ = fs::create_dir(&next_dir);
        create_nested(&next_dir, depth - 1, files_per_dir, content);
    }
    
    create_nested(&root, depth, files_per_dir, content);
    (dir, root)
}

/// 创建运行时配置
fn create_config(directory: PathBuf, threads: usize) -> RuntimeConfig {
    RuntimeConfig {
        directory,
        keyword: "error".to_string(),
        threads,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: false,
    }
}

/// 基准：小目录遍历（100个文件）
fn bench_traverse_small_directory(c: &mut Criterion) {
    let (_dir, path) = create_test_directory(100);
    let config = create_config(path, 1);
    
    c.bench_function("traverse_small_100_files", |b| {
        b.iter(|| {
            let _ = run(black_box(&config));
        })
    });
}

/// 基准：中等目录遍历（1K个文件）
fn bench_traverse_medium_directory(c: &mut Criterion) {
    let (_dir, path) = create_test_directory(1_000);
    let config = create_config(path, 1);
    
    c.bench_function("traverse_medium_1k_files", |b| {
        b.iter(|| {
            let _ = run(black_box(&config));
        })
    });
}

/// 基准：大目录遍历（10K个文件）
fn bench_traverse_large_directory(c: &mut Criterion) {
    let mut group = c.benchmark_group("traverse_large");
    group.sample_size(10); // 减少样本，因为这是重操作
    
    let (_dir, path) = create_test_directory(10_000);
    let config = create_config(path, 1);
    
    group.bench_function("traverse_large_10k_files", |b| {
        b.iter(|| {
            let _ = run(black_box(&config));
        })
    });
    
    group.finish();
}

/// 基准：并发度影响 - 不同线程数
fn bench_thread_count_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_count_impact");
    group.sample_size(10);
    
    let (_dir, path) = create_test_directory(1_000);
    
    for thread_count in &[1, 2, 4, 8] {
        let config = create_config(path.clone(), *thread_count);
        
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            thread_count,
            |b, _| {
                b.iter(|| {
                    let _ = run(black_box(&config));
                })
            },
        );
    }
    
    group.finish();
}

/// 基准：嵌套目录遍历性能
fn bench_nested_directory_traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_traversal");
    group.sample_size(10);
    
    for (depth, files) in &[(3, 5), (4, 3), (5, 2)] {
        let (_dir, path) = create_nested_directory(*depth, *files);
        let config = create_config(path, 1);
        
        group.bench_with_input(
            BenchmarkId::new("depth", format!("d{}_f{}", depth, files)),
            &(depth, files),
            |b, _| {
                b.iter(|| {
                    let _ = run(black_box(&config));
                })
            },
        );
    }
    
    group.finish();
}

/// 基准：并发加速比分析 - 固定工作量
fn bench_speedup_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("speedup_analysis");
    group.sample_size(10);
    
    let (_dir, path) = create_test_directory(2_000);
    
    // 收集不同线程数的性能数据
    for thread_count in &[1, 2, 4, 8] {
        let config = create_config(path.clone(), *thread_count);
        
        group.bench_with_input(
            BenchmarkId::new("speedup", format!("{}_threads", thread_count)),
            thread_count,
            |b, _| {
                b.iter(|| {
                    let _ = run(black_box(&config));
                })
            },
        );
    }
    
    group.finish();
}

/// 基准：文件数量vs性能的关系
fn bench_file_count_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_count_scaling");
    group.sample_size(10);
    
    for file_count in &[100, 500, 1_000, 5_000] {
        let (_dir, path) = create_test_directory(*file_count);
        let config = create_config(path, 4);
        
        group.bench_with_input(
            BenchmarkId::new("file_count", format!("{}_files", file_count)),
            file_count,
            |b, _| {
                b.iter(|| {
                    let _ = run(black_box(&config));
                })
            },
        );
    }
    
    group.finish();
}

/// 基准：include/exclude过滤性能影响
fn bench_filter_rules_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter_rules");
    group.sample_size(10);
    
    let (_dir, path) = create_test_directory(1_000);
    
    // 配置1：无过滤
    let config_no_filter = RuntimeConfig {
        directory: path.clone(),
        keyword: "error".to_string(),
        threads: 4,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: false,
    };
    
    // 配置2：包含特定扩展
    let config_with_include = RuntimeConfig {
        directory: path.clone(),
        keyword: "error".to_string(),
        threads: 4,
        verbose: false,
        include_ext: vec!["txt".to_string()],
        exclude_ext: vec![],
        use_regex: false,
    };
    
    // 配置3：排除特定扩展
    let config_with_exclude = RuntimeConfig {
        directory: path.clone(),
        keyword: "error".to_string(),
        threads: 4,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec!["log".to_string(), "tmp".to_string()],
        use_regex: false,
    };
    
    group.bench_function("no_filter", |b| {
        b.iter(|| {
            let _ = run(black_box(&config_no_filter));
        })
    });
    
    group.bench_function("with_include_filter", |b| {
        b.iter(|| {
            let _ = run(black_box(&config_with_include));
        })
    });
    
    group.bench_function("with_exclude_filter", |b| {
        b.iter(|| {
            let _ = run(black_box(&config_with_exclude));
        })
    });
    
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_traverse_small_directory,
        bench_traverse_medium_directory,
        bench_traverse_large_directory,
        bench_thread_count_impact,
        bench_nested_directory_traversal,
        bench_speedup_analysis,
        bench_file_count_scaling,
        bench_filter_rules_impact
);

criterion_main!(benches);
