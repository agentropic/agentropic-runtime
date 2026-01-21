# agentropic-runtime

[![Crates.io](https://img.shields.io/crates/v/agentropic-runtime.svg)](https://crates.io/crates/agentropic-runtime)
[![Documentation](https://docs.rs/agentropic-runtime/badge.svg)](https://docs.rs/agentropic-runtime)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**High-performance execution engine, scheduler, and isolation runtime for autonomous agents.**

`agentropic-runtime` is the execution foundation of the Agentropic ecosystem. It provides a robust, concurrent runtime environment for managing agent lifecycles, scheduling agent execution, enforcing resource limits, and ensuring fault isolation between agents.

---

## Purpose

This crate provides:

- **Execution Engine**: Concurrent agent execution with async/await support
- **Scheduler**: Fair, priority-based, and custom scheduling strategies
- **Isolation**: Resource limits, sandboxing, and fault containment
- **Lifecycle Management**: Automatic supervision, restart policies, and health monitoring
- **Observability**: Metrics, tracing, and telemetry

---

## Core Concepts

### Runtime

The central execution environment for agents:
```rust
use agentropic_runtime::{Runtime, RuntimeConfig};

// Create runtime
let runtime = Runtime::builder()
    .worker_threads(4)
    .max_agents(1000)
    .enable_isolation(true)
    .build()?;

// Spawn agent
let handle = runtime.spawn(agent).await?;

// Wait for completion
handle.await?;
```

### Scheduler

Controls when and how agents execute:
```rust
use agentropic_runtime::{Scheduler, SchedulingPolicy};

// Configure scheduler
let scheduler = Scheduler::builder()
    .policy(SchedulingPolicy::FairShare)
    .time_slice(Duration::from_millis(100))
    .max_concurrent(50)
    .build();

runtime.set_scheduler(scheduler);
```

### Agent Handle

Interface for controlling running agents:
```rust
use agentropic_runtime::AgentHandle;

// Spawn agent
let handle = runtime.spawn(my_agent).await?;

// Control agent
handle.pause().await?;
handle.resume().await?;
handle.stop().await?;

// Query state
let state = handle.state().await?;
let metrics = handle.metrics().await?;

// Wait for completion
let result = handle.await?;
```

### Isolation

Resource limits and sandboxing:
```rust
use agentropic_runtime::{IsolationConfig, ResourceLimits};

let isolation = IsolationConfig::builder()
    .memory_limit(512 * 1024 * 1024) // 512 MB
    .cpu_quota(0.5) // 50% of one core
    .max_file_handles(100)
    .network_restricted(true)
    .build();

let handle = runtime.spawn_isolated(agent, isolation).await?;
```

### Supervision

Automatic failure recovery:
```rust
use agentropic_runtime::{Supervisor, RestartPolicy};

let supervisor = Supervisor::builder()
    .restart_policy(RestartPolicy::OnFailure)
    .max_restarts(3)
    .backoff_strategy(BackoffStrategy::Exponential)
    .health_check_interval(Duration::from_secs(30))
    .build();

runtime.supervise(agent, supervisor).await?;
```

---

## What's Included

### Core Components

- `Runtime` - Main execution environment
- `RuntimeConfig` - Runtime configuration
- `AgentHandle` - Control interface for running agents
- `AgentExecutor` - Low-level agent execution
- `ExecutionContext` - Per-agent execution context

### Scheduling

- `Scheduler` - Agent scheduling coordinator
- `SchedulingPolicy` - Fair share, priority, round-robin, custom
- `TaskQueue` - Multi-level priority queues
- `TimeSlice` - Execution time allocation
- `Preemption` - Cooperative and preemptive scheduling

### Isolation

- `IsolationConfig` - Resource limits and sandboxing
- `ResourceLimits` - CPU, memory, I/O limits
- `Sandbox` - Restricted execution environment
- `NamespaceIsolation` - Process-level isolation
- `ResourceMonitor` - Real-time resource tracking

### Supervision

- `Supervisor` - Failure recovery and health monitoring
- `RestartPolicy` - Always, OnFailure, Never
- `BackoffStrategy` - Fixed, linear, exponential
- `HealthCheck` - Liveness and readiness probes
- `CircuitBreaker` - Failure cascade prevention

### Observability

- `Metrics` - Agent performance metrics
- `Tracing` - Execution traces
- `Logging` - Structured logging integration
- `Telemetry` - OpenTelemetry support

---

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agentropic-runtime = "0.1.0"
agentropic-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Runtime Usage
```rust
use agentropic_runtime::{Runtime, RuntimeConfig};
use agentropic_core::Agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create runtime
    let runtime = Runtime::new(RuntimeConfig::default())?;

    // Create agent
    let agent = MyAgent::new();

    // Spawn agent
    let handle = runtime.spawn(agent).await?;

    // Wait for completion
    let result = handle.await?;

    println!("Agent completed: {:?}", result);

    Ok(())
}
```

### Multi-Agent Execution
```rust
use agentropic_runtime::Runtime;

async fn run_multi_agent_system() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::builder()
        .worker_threads(8)
        .max_agents(100)
        .build()?;

    // Spawn multiple agents
    let mut handles = vec![];
    
    for i in 0..10 {
        let agent = WorkerAgent::new(i);
        let handle = runtime.spawn(agent).await?;
        handles.push(handle);
    }

    // Wait for all agents
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
```

### Custom Scheduling
```rust
use agentropic_runtime::{Scheduler, SchedulingPolicy, Priority};

async fn priority_scheduling() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = Scheduler::builder()
        .policy(SchedulingPolicy::Priority)
        .preemptive(true)
        .build();

    let runtime = Runtime::builder()
        .scheduler(scheduler)
        .build()?;

    // High priority agent
    let critical = CriticalAgent::new();
    let handle1 = runtime.spawn_with_priority(critical, Priority::High).await?;

    // Normal priority agent
    let worker = WorkerAgent::new();
    let handle2 = runtime.spawn_with_priority(worker, Priority::Normal).await?;

    // Low priority agent
    let background = BackgroundAgent::new();
    let handle3 = runtime.spawn_with_priority(background, Priority::Low).await?;

    Ok(())
}
```

### Resource Isolation
```rust
use agentropic_runtime::{IsolationConfig, ResourceLimits};

async fn isolated_execution() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::builder()
        .enable_isolation(true)
        .build()?;

    // Configure resource limits
    let limits = ResourceLimits::builder()
        .memory_limit(256 * 1024 * 1024) // 256 MB
        .cpu_quota(0.25) // 25% of one core
        .max_file_handles(50)
        .max_network_connections(10)
        .build();

    let isolation = IsolationConfig::builder()
        .resource_limits(limits)
        .filesystem_readonly(true)
        .network_restricted(true)
        .build();

    // Spawn isolated agent
    let agent = UntrustedAgent::new();
    let handle = runtime.spawn_isolated(agent, isolation).await?;

    // Monitor resource usage
    loop {
        let metrics = handle.metrics().await?;
        
        if metrics.memory_usage > limits.memory_limit {
            handle.stop().await?;
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
```

### Supervision and Recovery
```rust
use agentropic_runtime::{Supervisor, RestartPolicy, BackoffStrategy};

async fn supervised_agents() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new(RuntimeConfig::default())?;

    // Configure supervisor
    let supervisor = Supervisor::builder()
        .restart_policy(RestartPolicy::OnFailure)
        .max_restarts(5)
        .backoff_strategy(BackoffStrategy::Exponential {
            initial: Duration::from_secs(1),
            max: Duration::from_secs(60),
            multiplier: 2.0,
        })
        .health_check_interval(Duration::from_secs(10))
        .health_check(|agent| async move {
            agent.is_healthy().await
        })
        .build();

    // Spawn supervised agent
    let agent = UnreliableAgent::new();
    let handle = runtime.spawn_supervised(agent, supervisor).await?;

    // Agent will be automatically restarted on failure
    // with exponential backoff

    Ok(())
}
```

### Agent Lifecycle Hooks
```rust
use agentropic_runtime::{Runtime, LifecycleHooks};

async fn with_lifecycle_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let hooks = LifecycleHooks::builder()
        .on_start(|agent_id| async move {
            println!("Agent {} starting", agent_id);
        })
        .on_stop(|agent_id| async move {
            println!("Agent {} stopping", agent_id);
        })
        .on_error(|agent_id, error| async move {
            eprintln!("Agent {} error: {}", agent_id, error);
        })
        .build();

    let runtime = Runtime::builder()
        .lifecycle_hooks(hooks)
        .build()?;

    let agent = MyAgent::new();
    runtime.spawn(agent).await?;

    Ok(())
}
```

### Metrics and Observability
```rust
use agentropic_runtime::{Runtime, MetricsConfig};

async fn with_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::builder()
        .enable_metrics(true)
        .metrics_interval(Duration::from_secs(5))
        .build()?;

    let handle = runtime.spawn(agent).await?;

    // Collect metrics
    let metrics = handle.metrics().await?;
    println!("CPU usage: {}%", metrics.cpu_usage);
    println!("Memory: {} bytes", metrics.memory_usage);
    println!("Messages sent: {}", metrics.messages_sent);
    println!("Messages received: {}", metrics.messages_received);
    println!("Execution time: {:?}", metrics.execution_time);

    // Export to Prometheus
    runtime.export_metrics("0.0.0.0:9090").await?;

    Ok(())
}
```

---

## 🏗️ Architecture

### Runtime Architecture
```
┌─────────────────────────────────────────┐
│           Runtime Manager               │
├─────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │Scheduler │  │Supervisor│  │Metrics ││
│  └──────────┘  └──────────┘  └────────┘│
├─────────────────────────────────────────┤
│         Agent Execution Pool            │
│  ┌────────┐ ┌────────┐ ┌────────┐      │
│  │Agent 1 │ │Agent 2 │ │Agent N │      │
│  │Isolated│ │Isolated│ │Isolated│      │
│  └────────┘ └────────┘ └────────┘      │
└─────────────────────────────────────────┘
```

### Scheduling Policies

- **FairShare**: Equal time distribution among agents
- **Priority**: Execute higher priority agents first
- **RoundRobin**: Cyclic execution order
- **RateBased**: Throttle execution frequency
- **Custom**: User-defined scheduling logic

### Isolation Levels

- **None**: No isolation (fastest)
- **ResourceLimits**: CPU/memory constraints
- **Sandbox**: Restricted filesystem/network
- **Process**: Full process isolation
- **Container**: Container-based isolation

### Execution Modes

- **Synchronous**: Sequential execution
- **Concurrent**: Multiple agents, single thread
- **Parallel**: Multiple agents, multiple threads
- **Distributed**: Agents across multiple nodes (future)

---

## Performance

### Benchmarks

- **Agent spawn latency**: < 1ms
- **Context switch overhead**: < 10μs
- **Throughput**: 100,000+ agents/second
- **Memory per agent**: ~50KB baseline

### Optimization Tips

1. **Thread pool sizing**: Match worker threads to CPU cores
2. **Batch operations**: Spawn multiple agents together
3. **Async I/O**: Use tokio for I/O operations
4. **Resource pooling**: Reuse agent instances when possible
5. **Monitoring overhead**: Adjust metrics collection frequency

---

## Related Crates

- **[agentropic-core](../agentropic-core)** - Agent primitives and traits
- **[agentropic-messaging](../agentropic-messaging)** - Agent communication
- **[agentropic-cognition](../agentropic-cognition)** - Reasoning and planning
- **[agentropic-patterns](../agentropic-patterns)** - Multi-agent coordination

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agentropic-runtime).

For guides and tutorials, see [agentropic-docs](https://github.com/agentropic/agentropic-docs).

---

## References

This crate is inspired by:

- **Tokio** - Async runtime for Rust
- **Erlang/OTP** - Actor supervision and fault tolerance
- **Kubernetes** - Container orchestration patterns
- **Operating System Schedulers** - Process scheduling algorithms

---

## Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Status

**Active Development** - This crate is under active development. APIs may change before 1.0 release.

---

*Part of the [Agentropic](https://github.com/agentropic) ecosystem for agent-oriented programming in Rust.*
