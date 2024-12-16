# tmp_agent_control_server
This is a super simple web service to delete/deploy agent and will be deprecated in very short time.

# How to build
cargo build -r

# How to delete agent
curl -X POST http://[agent host]:3030/agent -H "Content-Type: application/json" -d '{"action": "delete", "agent_name": "name of agent"}'

# How to deploy agent
curl -X POST http://[agent host]:3030/agent -H "Content-Type: application/json" -d '{"action": "deploy"}'

# Note
The deploy API will deploy all agents in the configuration and take a lot of time to finish it job. Therefore, don't trigger it in very short period.
