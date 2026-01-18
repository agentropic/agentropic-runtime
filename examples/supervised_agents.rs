use agentropic_runtime::prelude::*;
use std::time::Duration;

fn main() {
    println!("=== Supervised Agents Example ===\n");

    // Create supervisor
    let mut supervisor = Supervisor::new("main_supervisor");

    println!("Supervisor created: {}", supervisor.name());

    // Create agents with different restart policies
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    let agent3 = AgentId::new();

    // Agent 1: Always restart
    let policy1 = RestartPolicy::new(RestartStrategy::Always).with_max_retries(5);

    // Agent 2: Restart on failure with backoff
    let policy2 = RestartPolicy::new(RestartStrategy::OnFailure)
        .with_max_retries(3)
        .with_backoff_seconds(5);

    // Agent 3: Never restart
    let policy3 = RestartPolicy::new(RestartStrategy::Never);

    supervisor.supervise(agent1, policy1);
    supervisor.supervise(agent2, policy2);
    supervisor.supervise(agent3, policy3);

    println!("\n✓ Supervising {} agents", supervisor.supervised_count());

    // Health checks
    println!("\n--- Health Checks ---");

    if let Some(health) = supervisor.get_health_check_mut(&agent1) {
        health.record_healthy();
        println!(
            "Agent 1 - Status: {:?}, Failures: {}",
            health.status(),
            health.failures()
        );
    }

    if let Some(health) = supervisor.get_health_check_mut(&agent2) {
        health.record_unhealthy();
        health.record_unhealthy();
        println!(
            "Agent 2 - Status: {:?}, Failures: {}",
            health.status(),
            health.failures()
        );
    }

    // Circuit breaker
    println!("\n--- Circuit Breaker ---");

    let mut breaker = CircuitBreaker::new(3, Duration::from_secs(60));
    println!("Initial state: {:?}", breaker.state());

    // Simulate failures
    for i in 1..=4 {
        breaker.record_failure();
        println!(
            "After failure {}: State={:?}, Failures={}, Allowed={}",
            i,
            breaker.state(),
            breaker.failure_count(),
            breaker.is_allowed()
        );
    }

    // Recovery
    breaker.record_success();
    println!("After success: State={:?}", breaker.state());

    // Exponential backoff
    println!("\n--- Exponential Backoff ---");

    let mut backoff = ExponentialBackoff::new(Duration::from_secs(1), Duration::from_secs(60));

    for i in 1..=5 {
        let delay = backoff.next_delay();
        println!("Retry {}: Delay={:?}", i, delay);
    }

    println!("\nTotal retries: {}", backoff.retries());

    backoff.reset();
    println!("After reset - Retries: {}", backoff.retries());
}
