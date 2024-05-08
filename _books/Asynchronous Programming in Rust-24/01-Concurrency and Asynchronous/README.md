# Concurrency and Asynchronous Programming: a Detailed Overview

Non-preemptive multitasking / cooperative multitasking: programmer has the responsibility to yield control to other tasks. A small mistake in a program’s code could halt or crash the entire system.

Preemptive multitasking: The OS is responsible for scheduling tasks and does this by switching contexts on the CPU.

Hyper-threading:  CPUs have several arithmetic logic units (ALUs) and additional logic units, when an operation only required some parts of the CPU, an instruction could be run on the ALU simultaneously.

Multicore processors: CPUs have multiple cores, each core has its own ALUs and logic units, and can run multiple threads simultaneously.

## Concurrency versus parallelism

Concurrency:  One is by progressing tasks concurrently, but not at the same time.
Parallelism: progress tasks at the exact same time in parallel.
