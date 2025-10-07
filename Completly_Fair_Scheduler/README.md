# Completly Fair Scheduler (CFS)
A permative scheduler (can be interrupted) utilzed by the linux kernel
TODO: 
- The red-black tree has been started but needs work and is needed for the CFS algorithm which has not been started

CFS is initially calculated based on the min_vruntime of the system, which represents the lowest vruntime among all currently runnable tasks.
When a new task is created and becomes runnable, its initial vruntime is set to the current min_vruntime of the system's run queue. This ensures that newly created tasks do not immediately gain a significant advantage or disadvantage in scheduling compared to existing tasks.

utilizes red-black tree
node is either red or black
root/leaves are black
if a node is red then its children are black
all paths from a node to its NIL descendants contain the same number of black nodes


utilizes virtual runtimes per process as opposed to total runtime of the cpu
all sorted with a red-black tree algorithm, and the lowest vruntime is selected next to run
CFS also uses weighted fair queuing (WFQ) which determines a processors share of the CPU
- higher weight = more cpu time
- by default all processes have the same wfq equalling cpu shares
Nice values are user-level priority adjustments 
- it is a range from -20 until 19
- Lower nice values give a process higher priority (higher weight), while higher nice values lower priority.

## How it works
Scheduling Event: When the scheduler needs to select a new process to run (e.g., due to a time slice expiring or a process blocking), it examines the red-black tree.
Process Selection: The process with the lowest vruntime is selected. This is the process that has received the least CPU time relative to its weight.
Runtime Update: The selected process is allowed to run for a short period. Its vruntime is then increased to reflect the actual CPU time consumed.
Tree Rebalancing: The red-black tree is rebalanced to maintain its sorted order.
Repeat: Steps 1-4 are repeated continuously, ensuring that processes are scheduled in a fair manner.
