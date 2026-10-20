// PerfReport (Rust) | IS2083 Lab 3
// Migrated from the Windows-only C++ utility using sysinfo.

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// Scoring logic provided in the assignment.
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");
    let mut sys = System::new_all();
    sys.refresh_all();

    // Processors
    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    // Memory
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB) : {:.2}", total as f64 / GB);
    println!("Memory free (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%) : {}", mem_load);

    // Uptime
    println!("Uptime (minutes)  : {}", System::uptime() / 60);

    // Disk usage
    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct: f64 = 0.0;
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        let used_pct = if dtotal > 0 {
            100.0 * (dtotal - dfree) as f64 / dtotal as f64
        } else {
            0.0
        };
        println!("Disk total (GB)   : {:.2}", dtotal as f64 / GB);
        println!("Disk free (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used (%)     : {:.2}", used_pct);
        disk_used_pct = used_pct;
    } else {
        println!("Disk information  : unavailable");
    }

    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score  : {} / 100 ({})", score, rating(score));
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn healthy_machine_is_good() {
        let findings = Findings {
            mem_load: 40,
            disk_used_pct: 35.0,
        };
        assert_eq!(compute_score(&findings), 100);
        assert_eq!(rating(compute_score(&findings)), "GOOD");
    }

    #[test]
    fn moderate_usage_reduces_score() {
        let findings = Findings {
            mem_load: 75,
            disk_used_pct: 80.0,
        };
        assert_eq!(compute_score(&findings), 80);
        assert_eq!(rating(compute_score(&findings)), "FAIR");
    }

    #[test]
    fn high_usage_is_penalized() {
        let findings = Findings {
            mem_load: 95,
            disk_used_pct: 95.0,
        };
        assert_eq!(compute_score(&findings), 50);
        assert_eq!(rating(compute_score(&findings)), "POOR");
    }
}
