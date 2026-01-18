use agentropic_runtime::prelude::*;

#[tokio::test]
async fn full_runtime_lifecycle() {
    // Create runtime
    let config = RuntimeConfig::new().with_max_workers(2);
    let runtime = Runtime::with_config(config);

    // Start runtime
    runtime.start().await.unwrap();
    assert!(runtime.is_running().await);

    // Spawn agents
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();

    runtime.spawn(agent1, "agent1").await.unwrap();
    runtime.spawn(agent2, "agent2").await.unwrap();

    assert_eq!(runtime.agent_count().await, 2);

    // Get handle
    let handle = runtime.handle();
    assert_eq!(handle.agent_count().await, 2);

    // Shutdown
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn supervised_runtime() {
    let runtime = Runtime::new();
    let mut supervisor = Supervisor::new("main");

    let agent_id = AgentId::new();
    let policy = RestartPolicy::new(RestartStrategy::OnFailure).with_max_retries(3);

    supervisor.supervise(agent_id, policy);
    runtime.spawn(agent_id, "supervised_agent").await.unwrap();

    assert!(supervisor.get_policy(&agent_id).is_some());
    assert!(runtime.has_agent(&agent_id).await);
}
