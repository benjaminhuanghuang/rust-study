# Ch3 Asynchronous Primitives

## What are asynchronous primitives?

Established parameters that are used to synchronize the running of asynchronous threads

Asynchronous primitives allows us to accomplish
sophisticated goals.

## What is a mutex?

A programming mechanism that allows for exclusive access to a resource

Only one task at a time can have control

To manipulate an item protected by a Mutex, you must request access

## What is a semaphore?

An asynchronous primitive that limits access to a resource

## What is notify?

Acts as both a transmitter and a receiver
Create thread-safe clones of your notify instance
Pass the clone to all functions that need them

## What is a barrier?

Waits for all wanted threads to arrive before allowing any single thread to make progress

An asynchronous primitive that ensures a certain number of tasks reach a rendezvous point before
they can proceed with their execution

## What is RwLock?
