use agentropic_runtime::prelude::*;
use std::time::Duration;

#[test]
fn create_supervisor() {
    let supervisor = Supervisor::new("main");
    assert_eq!(supervisor.name(), "main");
    assert_eq!(supervisor.supervised_count(), 0);
}

#[test]
fn supervise_agent() {
    let mut supervisor = Supervisor::new("main");
    let agent_id = AgentId::new();
    let policy = RestartPolicy::new(RestartStrategy::Always);

    supervisor.supervise(agent_id, policy);
    assert_eq!(supervisor.supervised_count(), 1);
    assert!(supervisor.get_policy(&agent_id).is_some());
}

#[test]
fn restart_policy() {
    let policy = RestartPolicy::new(RestartStrategy::OnFailure)
        .with_max_retries(3)
        .with_backoff_seconds(5);

    assert_eq!(policy.strategy(), RestartStrategy::OnFailure);
    assert_eq!(policy.max_retries(), Some(3));
    assert_eq!(policy.backoff_seconds(), 5);
}

#[test]
fn exponential_backoff() {
    let mut backoff = ExponentialBackoff::new(Duration::from_secs(1), Duration::from_secs(60));

    let delay1 = backoff.next_delay();
    let delay2 = backoff.next_delay();

    assert_eq!(delay1, Duration::from_secs(1));
    assert_eq!(delay2, Duration::from_secs(2));
    assert_eq!(backoff.retries(), 2);
}

#[test]
fn health_check() {
    let mut health = HealthCheck::new();

    assert_eq!(health.status(), HealthStatus::Unknown);

    health.record_healthy();
    assert!(health.is_healthy());
    assert_eq!(health.failures(), 0);

    health.record_unhealthy();
    assert!(!health.is_healthy());
    assert_eq!(health.failures(), 1);
}

#[test]
fn circuit_breaker() {
    let mut breaker = CircuitBreaker::new(3, Duration::from_secs(10));

    assert_eq!(breaker.state(), CircuitState::Closed);
    assert!(breaker.is_allowed());

    breaker.record_failure();
    breaker.record_failure();
    breaker.record_failure();

    assert_eq!(breaker.state(), CircuitState::Open);
    assert!(!breaker.is_allowed());

    breaker.record_success();
    assert_eq!(breaker.state(), CircuitState::Closed);
}
