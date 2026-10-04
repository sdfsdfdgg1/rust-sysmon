use std::thread;
use std::time::Duration;
use std::fs;

fn get_cpu_usage() -> f64 {
    let stat = fs::read_to_string("/proc/stat").unwrap_or_default();
    let line = stat.lines().next().unwrap_or("");
    let parts: Vec<u64> = line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    if parts.len() < 4 { return 0.0; }
    let total: u64 = parts.iter().sum();
    let idle = parts[3];
    ((total - idle) as f64 / total as f64) * 100.0
}

fn get_memory_usage() -> (u64, u64) {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0u64;
    let mut available = 0u64;
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
        }
    }
    (total - available, total)
}

fn main() {
    println!("sysmon v0.1.0 - System Monitor");
    loop {
        let cpu = get_cpu_usage();
        let (used_mem, total_mem) = get_memory_usage();
        println!("CPU: {:.1}% | Memory: {} / {} kB ({:.1}%)",
            cpu, used_mem, total_mem,
            (used_mem as f64 / total_mem as f64) * 100.0);
        thread::sleep(Duration::from_secs(5));
    }
}