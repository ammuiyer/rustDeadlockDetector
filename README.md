# rustDeadlockDetector

This repo contains the Rust implementation of Banker's Algorithm, an algorithms that simulates and predicts deadlocks. A deadlock is best descirbed as a form of circular dependency - Process A is relying on Process B which is relying on Process C, which is relying on Process C. With a map of the allocation of processes and their respective resources, it is possible to predict whether or not a deadlock may occur.  
