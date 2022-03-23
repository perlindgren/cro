# Cro

Concurrent Reactive Objects (CRO) in Rust.

## Principle

CROs as a model provides concurrency by construction.
Each object holds a private state accessible only through is provided interface.

Objects may communicate either synchronously (by calling methods) on other CROs,
or asynchronously using message passing.



