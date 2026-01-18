use agentropic_runtime::prelude::*;

fn main() {
    println!("=== Isolated Agents Example ===\n");

    // Create isolation configuration
    let isolation_config = IsolationConfig::new()
        .with_cpu_quota(50.0)
        .with_memory_limit(512 * 1024 * 1024) // 512MB
        .with_network_isolation(true);

    println!("Isolation Config:");
    println!("  Enabled: {}", isolation_config.enabled);
    println!("  CPU Quota: {}%", isolation_config.cpu_quota);
    println!(
        "  Memory Limit: {} MB",
        isolation_config.memory_limit / 1024 / 1024
    );
    println!(
        "  Network Isolation: {}",
        isolation_config.network_isolation
    );

    // Create resource limits
    let limits = ResourceLimits::new()
        .with_max_cpu(75.0)
        .with_max_memory(256 * 1024 * 1024) // 256MB
        .with_max_threads(5)
        .with_max_file_descriptors(100);

    println!("\nResource Limits:");
    println!("  Max CPU: {}%", limits.max_cpu);
    println!("  Max Memory: {} MB", limits.max_memory / 1024 / 1024);
    println!("  Max Threads: {}", limits.max_threads);
    println!("  Max File Descriptors: {}", limits.max_file_descriptors);

    // Create sandbox
    let agent_id = AgentId::new();
    let sandbox = Sandbox::new(agent_id, isolation_config, limits);

    println!("\n✓ Sandbox created for agent: {}", sandbox.agent_id());
    println!("  Sandbox enabled: {}", sandbox.is_enabled());

    // Create resource monitor
    let mut monitor = ResourceMonitor::new();

    println!("\n--- Resource Monitoring ---");
    println!("Initial usage:");
    println!("  CPU: {}%", monitor.current_usage().cpu_usage);
    println!("  Memory: {} bytes", monitor.current_usage().memory_usage);

    // Simulate resource usage update
    let usage = ResourceUsage {
        cpu_usage: 45.5,
        memory_usage: 128 * 1024 * 1024,
        thread_count: 3,
    };

    monitor.update(usage);

    println!("\nUpdated usage:");
    println!("  CPU: {}%", monitor.current_usage().cpu_usage);
    println!(
        "  Memory: {} MB",
        monitor.current_usage().memory_usage / 1024 / 1024
    );
    println!("  Threads: {}", monitor.current_usage().thread_count);
    println!("  Uptime: {:?}", monitor.uptime());

    // Create namespace
    let namespace = Namespace::new(agent_id, "trading_namespace");
    println!("\n✓ Namespace created: {}", namespace.name());
}
