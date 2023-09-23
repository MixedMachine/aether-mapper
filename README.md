# Aether Mapper

*Mixed Machine* <br />
*mixedmachine.dev@gmail.com*

## Summary

This project aims to create a real-time network topology mapper capable of intelligently discovering and identifying devices, services, and network attributes. Combining the power of low-level network programming in C and high-level asynchronous event handling in Rust, the tool offers a versatile approach to network mapping and monitoring.

## Key Features

- [ ] Real-Time Device Discovery: Rapidly identifies connected devices on the network.
- [ ] Port Scanning: Detects open ports and attempts to identify running services.
- [ ] Dynamic Topology Updates: Updates network topology in real-time to reflect changes.
- [ ] IPC for Real-Time Data: Uses POSIX Message Queues for fast, real-time inter-process communication between C and Rust components.
- [ ] Event-Driven Architecture: Utilizes asynchronous programming in Rust to handle real-time events effectively.

## Technologies Used

- C for device discovery and port scanning
- Rust for the main application logic and event-handling
- POSIX Message Queues for IPC
- [Future] REST API for external data consumption
