use std::fs::File; 
use std::io::{self, BufRead, BufReader}; 

struct Process {
    pid: u32,
    burst_time: u32,
    arrival_time: u32,
    completion_time: u32,
    turnaround_time: u32,
    waiting_time: u32,
}

// Our process thread
impl Process {
    fn new(pid: u32, burst_time: u32, arrival_time: u32) -> Self {
        Process {
            pid,
            burst_time, 
            arrival_time, 
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("config/processes.txt")?;
    let reader = BufReader::new(file); 
    let mut processes: Vec<Process> = Vec::new();

    // Read the file
    for(index, line) in reader.lines().enumerate() {
        // I have a header line in the processes.txt file, I will just skip that
        if index == 0 { continue; }

        let line = line?;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect(); // NOTE: Spaces are ignored, txt file is formated to prove this
        
        // check if there are only 4 elements (AKA format is correct in the file)
        if parts.len() != 4 {
            eprintln!("ERROR: Invalid line format on line: {0}", line);
            continue;
        }

        // Now that we know the elements are correct, we can create the process from those elements here
        // PID, Priority, Burst, Arrival
        let pid: u32 = parts[0].parse().expect("ERROR: Invalid PID");
        // FIFO does not need priority, so we completly ignore it
        // let priority: u32 = parts[1].parse().expect("ERROR: Invalid priority");
        let burst_time: u32 = parts[2].parse().expect("ERROR: Invalid burst time");
        let arrival_time: u32 = parts[3].parse().expect("ERROR: Invalid arrival time");
        processes.push(Process::new(pid, burst_time, arrival_time));
    }

    // Now we are starting the FIFO scheduler
    processes.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time)); // sort by arival time so we can just iterate the processes in order
    let mut current_time = 0;
    for process in processes.iter_mut() {
        // Now we set the current time to the arrival time of the first process since thats when this starts
        if current_time < process.arrival_time {
            current_time = process.arrival_time
        }
        process.completion_time = current_time + process.burst_time;
        process.turnaround_time = process.completion_time - process.arrival_time;
        process.waiting_time = process.turnaround_time - process.burst_time;

        // Now that we processes the process, we update the current time until the next process starts, and the scheduler continues
        current_time = process.completion_time;
    }

    // Now we print the results
    println!("******* FIFO Scheduling Results *******");
    println!("PID    Burst Time    Arrival Time    Completion Time    Turnaround Time    Wait Time");
              
    for process in &processes {
        println!(" {0}         {1}              {2}               {3}                {4}                {5}", 
                 process.pid,
                 process.burst_time,
                 process.arrival_time,
                 process.completion_time,
                 process.turnaround_time,
                 process.waiting_time
                );
    }

    // For fun I am also gonna throw some averages
    let n = processes.len() as f32; // This is a float since itll be used for averages
    let avg_turn = processes.iter().map(|p| p.turnaround_time as f32).sum::<f32>() / n;
    let avg_wait = processes.iter().map(|p| p.waiting_time as f32).sum::<f32>() / n;

    println!("The Average turnaround time was: {0}", avg_turn);
    println!("The Average wait time was: {0}", avg_wait);
    Ok(())
}
