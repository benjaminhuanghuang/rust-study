Rust programs are free from

- Dangling pointer — Live references to data that has become invalid over the course of the program 

- Data races — The inability to determine how a program will behave from run to run because external factors change 

- Buffer overflow — An attempt to access the 12th element of an array with only 6 elements

- Iterator invalidation — An issue caused by something that is iterated over after being altered midway through 