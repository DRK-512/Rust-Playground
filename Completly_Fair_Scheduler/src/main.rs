use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

mod rbtree; // <-- this is where the RBTree<T> from earlier lives

#[derive(Debug, Clone)]
struct CFSProcess {
    pid: u32,
    priority: i32,
    burst_time: u32,
    arrival_time: u32,
    completion_time: u32,
    turnaround_time: u32,
    virt_runtime: u64,
}

impl CFSProcess {
    fn new(pid: u32, priority: i32, burst_time: u32, arrival_time: u32) -> Self {
        Self {
            pid,
            priority,
            burst_time,
            arrival_time,
            completion_time: 0,
            turnaround_time: 0,
            virt_runtime: 0,
        }
    }
}

// Linux nice -> weight table (simplified)
fn nice_to_weight(nice: i32) -> u64 {
    const WEIGHTS: [u64; 40] = [
        88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916, 9548, 7620, 6100,
        4904, 3906, 3121, 2501, 1991, 1586, 1277, 1024, 820, 655, 526, 423, 335, 272, 215, 172,
        137, 110, 87, 70, 56, 45, 36, 29, 23, 18, 15,
    ];
    WEIGHTS[(nice + 20) as usize]
}

fn main() -> io::Result<()> {
    let file = File::open("config/processes.txt")?;
    let reader = BufReader::new(file);
    let mut incoming: Vec<CFSProcess> = Vec::new();

    // Load process list
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue;
        } // skip header
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            eprintln!("Invalid line format: {line}");
            continue;
        }
        let pid: u32 = parts[0].trim().parse().expect("Invalid PID");
        let priority: i32 = parts[1].trim().parse().expect("Invalid priority");
        if !(-20..=19).contains(&priority) {
            eprintln!("Invalid priority: {priority}");
            process::exit(1);
        }
        let burst_time: u32 = parts[2].trim().parse().expect("Invalid burst time");
        let arrival_time: u32 = parts[3].trim().parse().expect("Invalid arrival time");
        incoming.push(CFSProcess::new(pid, priority, burst_time, arrival_time));
    }

    incoming.sort_by_key(|p| p.arrival_time); // sort by arrival for simulation

    // CFS scheduler
    let mut time: u32 = 0;
    let mut index = 0;
    let mut tree = rbtree::RBTree::<(u64, u32)>::new(); // key = (vruntime, pid)
    let mut proc_map: HashMap<u32, CFSProcess> = HashMap::new();

    let slice = 4; // fixed time slice for demo

    while index < incoming.len() || !tree.is_empty() {
        // Add newly arrived processes
        while index < incoming.len() && incoming[index].arrival_time <= time {
            let p = incoming[index].clone();
            proc_map.insert(p.pid, p.clone());
            tree.insert((p.virt_runtime, p.pid));
            index += 1;
        }

        if tree.is_empty() {
            time += 1; // idle
            continue;
        }

        // Pick process with smallest vruntime
        let &(vrun, pid) = tree.inorder().first().unwrap();
        tree.remove(&(vrun, pid));
        let proc = proc_map.get_mut(&pid).unwrap();

        let weight = nice_to_weight(proc.priority);
        let run_time = slice.min(proc.burst_time);
        proc.burst_time -= run_time;
        time += run_time;
        proc.virt_runtime += (run_time as u64 * 1024) / weight;

        if proc.burst_time == 0 {
            proc.completion_time = time;
            proc.turnaround_time = time - proc.arrival_time;
        } else {
            tree.insert((proc.virt_runtime, proc.pid));
        }
    }

    println!("******* CFS Scheduling Results *******");
    println!("PID  Priority  Arrival  Completion  Turnaround");
    for p in proc_map.values() {
        println!(
            "{:<4}{:<9}{:<8}{:<12}{:<11}",
            p.pid, p.priority, p.arrival_time, p.completion_time, p.turnaround_time
        );
    }

    Ok(())
}
