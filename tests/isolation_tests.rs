use agentropic_runtime::prelude::*;

#[test]
fn create_isolation_config() {
    let config = IsolationConfig::new()
        .with_cpu_quota(50.0)
        .with_memory_limit(512 * 1024 * 1024);

    assert_eq!(config.cpu_quota, 50.0);
    assert_eq!(config.memory_limit, 512 * 1024 * 1024);
}

#[test]
fn resource_limits() {
    let limits = ResourceLimits::new().with_max_cpu(75.0).with_max_threads(5);

    assert_eq!(limits.max_cpu, 75.0);
    assert_eq!(limits.max_threads, 5);
}

#[test]
fn sandbox_creation() {
    let agent_id = AgentId::new();
    let config = IsolationConfig::new();
    let limits = ResourceLimits::new();

    let sandbox = Sandbox::new(agent_id, config, limits);
    assert_eq!(sandbox.agent_id(), &agent_id);
    assert!(sandbox.is_enabled());
}

#[test]
fn resource_monitor() {
    let monitor = ResourceMonitor::new();
    assert_eq!(monitor.current_usage().cpu_usage, 0.0);
    assert_eq!(monitor.current_usage().memory_usage, 0);
}

#[test]
fn namespace() {
    let agent_id = AgentId::new();
    let namespace = Namespace::new(agent_id, "test_namespace");

    assert_eq!(namespace.agent_id(), &agent_id);
    assert_eq!(namespace.name(), "test_namespace");
}
