use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use file_search::config::RuntimeConfig;
use file_search::executor::run;
use std::fs;
use tempfile::TempDir;

/// 创建完整的测试场景
struct TestScenario {
    _temp_dir: TempDir,
    directory: std::path::PathBuf,
}

impl TestScenario {
    /// 创建简单场景：小代码库
    fn simple_codebase() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let base_path = temp_dir.path().to_path_buf();
        
        // 创建目录结构
        fs::create_dir_all(base_path.join("src")).unwrap();
        fs::create_dir_all(base_path.join("tests")).unwrap();
        fs::create_dir_all(base_path.join("docs")).unwrap();
        
        // 创建源代码文件
        let rust_code = r#"
fn main() {
    println!("Hello, world!");
    let error = "Something went wrong";
    eprintln!("Error: {}", error);
}

fn handle_error(err: &str) {
    eprintln!("Processing error: {}", err);
}
"#;
        
        for i in 0..10 {
            fs::write(base_path.join(format!("src/module_{}.rs", i)), rust_code).unwrap();
        }
        
        // 创建测试文件
        let test_code = r#"
#[test]
fn test_error_handling() {
    let result = parse_error("invalid data");
    assert!(result.is_err());
}

#[test]
fn test_warning_message() {
    let msg = format_warning("timeout");
    assert!(msg.contains("warning"));
}
"#;
        
        for i in 0..5 {
            fs::write(base_path.join(format!("tests/test_{}.rs", i)), test_code).unwrap();
        }
        
        // 创建文档文件
        let doc_content = "# Error Handling\nThis document describes error patterns.\nWarning: Always check errors!";
        for i in 0..3 {
            fs::write(base_path.join(format!("docs/doc_{}.md", i)), doc_content).unwrap();
        }
        
        TestScenario {
            _temp_dir: temp_dir,
            directory: base_path,
        }
    }
    
    /// 创建中等场景：中型项目
    fn medium_project() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let base_path = temp_dir.path().to_path_buf();
        
        // 创建多个模块
        for module_idx in 0..5 {
            let module_dir = base_path.join(format!("module_{}", module_idx));
            fs::create_dir_all(&module_dir.join("src")).unwrap();
            fs::create_dir_all(&module_dir.join("tests")).unwrap();
            
            let rust_code = r#"
pub fn process() {
    match validate_input() {
        Ok(data) => println!("Processing: {:?}", data),
        Err(e) => eprintln!("Processing error: {}", e),
    }
}

fn validate_input() -> Result<String, String> {
    Err("Validation failed: check constraints".to_string())
}

pub fn handle_exception(msg: &str) {
    eprintln!("Exception: {}", msg);
}
"#;
            
            for i in 0..20 {
                fs::write(
                    module_dir.join(format!("src/lib_{}.rs", i)),
                    rust_code,
                ).unwrap();
            }
            
            for i in 0..10 {
                fs::write(
                    module_dir.join(format!("tests/integration_{}.rs", i)),
                    rust_code,
                ).unwrap();
            }
        }
        
        TestScenario {
            _temp_dir: temp_dir,
            directory: base_path,
        }
    }
    
    /// 创建复杂场景：大型项目
    fn large_project() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let base_path = temp_dir.path().to_path_buf();
        
        // 创建大型项目结构
        for top_level in 0..10 {
            let top_dir = base_path.join(format!("subsystem_{}", top_level));
            
            for module_idx in 0..5 {
                let module_dir = top_dir.join(format!("module_{}", module_idx));
                fs::create_dir_all(&module_dir.join("src")).unwrap();
                fs::create_dir_all(&module_dir.join("tests")).unwrap();
                fs::create_dir_all(&module_dir.join("bench")).unwrap();
                
                let code = r#"
// Complex error handling workflow
pub fn process_request(req: Request) -> Result<Response, ProcessError> {
    match validate_request(&req) {
        Ok(valid_req) => {
            match execute_operation(&valid_req) {
                Ok(result) => Ok(create_response(result)),
                Err(exec_err) => {
                    eprintln!("Execution error: {:?}", exec_err);
                    Err(exec_err)
                }
            }
        }
        Err(val_err) => {
            eprintln!("Validation error: {}", val_err);
            Err(val_err)
        }
    }
}

fn validate_request(req: &Request) -> Result<Request, ProcessError> {
    if req.data.is_empty() {
        Err(ProcessError::InvalidInput)
    } else {
        Ok(req.clone())
    }
}

fn execute_operation(_req: &Request) -> Result<String, ProcessError> {
    Err(ProcessError::ExecutionFailed)
}

fn create_response(result: String) -> Response {
    Response { data: result }
}
"#;
                
                for i in 0..30 {
                    fs::write(module_dir.join(format!("src/handler_{}.rs", i)), code).unwrap();
                }
                
                for i in 0..15 {
                    fs::write(module_dir.join(format!("tests/test_{}.rs", i)), code).unwrap();
                }
            }
        }
        
        TestScenario {
            _temp_dir: temp_dir,
            directory: base_path,
        }
    }
}

/// 基准：简单场景端到端 - 搜索"error"
fn bench_e2e_simple_literal_search(c: &mut Criterion) {
    let scenario = TestScenario::simple_codebase();
    
    let config = RuntimeConfig {
        directory: scenario.directory.clone(),
        keyword: "error".to_string(),
        threads: 4,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: false,
    };
    
    c.bench_function("e2e_simple_literal_error", |b| {
        b.iter(|| {
            let _ = run(black_box(&config));
        })
    });
}

/// 基准：简单场景正则搜索 - 搜索"error|warning"
fn bench_e2e_simple_regex_search(c: &mut Criterion) {
    let scenario = TestScenario::simple_codebase();
    
    let config = RuntimeConfig {
        directory: scenario.directory.clone(),
        keyword: "error|warning".to_string(),
        threads: 4,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: true,
    };
    
    c.bench_function("e2e_simple_regex_pattern", |b| {
        b.iter(|| {
            let _ = run(black_box(&config));
        })
    });
}

/// 基准：中等场景端到端 - 多并发级别对比
fn bench_e2e_medium_project_comparative(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_medium_comparative");
    group.sample_size(10);
    
    let scenario = TestScenario::medium_project();
    
    for thread_count in &[1, 2, 4, 8] {
        let config = RuntimeConfig {
            directory: scenario.directory.clone(),
            keyword: "error|exception".to_string(),
            threads: *thread_count,
            verbose: false,
            include_ext: vec!["rs".to_string()],
            exclude_ext: vec![],
            use_regex: true,
        };
        
        group.bench_with_input(
            BenchmarkId::new("medium_threads", thread_count),
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

/// 基准：大型场景端到端 - 完整搜索流程
fn bench_e2e_large_project_full_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_large_full");
    group.sample_size(10);
    
    let scenario = TestScenario::large_project();
    
    // 场景1：简单字面量搜索
    let config1 = RuntimeConfig {
        directory: scenario.directory.clone(),
        keyword: "error".to_string(),
        threads: 8,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: false,
    };
    
    group.bench_function("large_simple_literal", |b| {
        b.iter(|| {
            let _ = run(black_box(&config1));
        })
    });
    
    // 场景2：复杂正则搜索
    let config2 = RuntimeConfig {
        directory: scenario.directory.clone(),
        keyword: "(?:error|warning|exception)\\s*:\\s*[A-Za-z]".to_string(),
        threads: 8,
        verbose: false,
        include_ext: vec![],
        exclude_ext: vec![],
        use_regex: true,
    };
    
    group.bench_function("large_complex_regex", |b| {
        b.iter(|| {
            let _ = run(black_box(&config2));
        })
    });
    
    // 场景3：使用文件扩展过滤
    let config3 = RuntimeConfig {
        directory: scenario.directory.clone(),
        keyword: "ProcessError|ExecutionFailed".to_string(),
        threads: 8,
        verbose: false,
        include_ext: vec!["rs".to_string()],
        exclude_ext: vec!["md".to_string()],
        use_regex: true,
    };
    
    group.bench_function("large_with_filters", |b| {
        b.iter(|| {
            let _ = run(black_box(&config3));
        })
    });
    
    group.finish();
}

/// 基准：场景对比 - 相同搜索在不同项目规模
fn bench_e2e_scale_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_scale_comparison");
    group.sample_size(10);
    
    let scenarios = vec![
        ("simple", TestScenario::simple_codebase()),
        ("medium", TestScenario::medium_project()),
        ("large", TestScenario::large_project()),
    ];
    
    for (name, scenario) in scenarios {
        let config = RuntimeConfig {
            directory: scenario.directory.clone(),
            keyword: "error".to_string(),
            threads: 4,
            verbose: false,
            include_ext: vec![],
            exclude_ext: vec![],
            use_regex: false,
        };
        
        group.bench_with_input(
            BenchmarkId::new("scale", name),
            &name,
            |b, _| {
                b.iter(|| {
                    let _ = run(black_box(&config));
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        bench_e2e_simple_literal_search,
        bench_e2e_simple_regex_search,
        bench_e2e_medium_project_comparative,
        bench_e2e_large_project_full_search,
        bench_e2e_scale_comparison
);

criterion_main!(benches);
