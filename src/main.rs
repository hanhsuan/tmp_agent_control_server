use std::process::Command;
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestPayload {
    action: String,
    agent_name: Option<String>,
}

#[derive(Serialize)]
struct ResponsePayload {
    status: String,
}

fn is_valid_agent_name(s: &str) -> bool {
    // Check if all characters in the string are either alphabetic, numeric, or a dash
    s.chars().all(|c| c.is_alphanumeric() || c == '-')
}

async fn handle_request(payload: RequestPayload) -> Result<impl warp::Reply, warp::Rejection> {
    match payload.action.as_str() {
        "delete" => {
            if let Some(agent_name) = payload.agent_name {
                if is_valid_agent_name(&agent_name) {
                    let result = delete_agent(&agent_name).await;
                    Ok(warp::reply::json(&ResponsePayload { status: result }))
                } else {
                    Ok(warp::reply::json(&ResponsePayload { status: "invalid agent name".to_string() }))
                }
            } else {
                Ok(warp::reply::json(&ResponsePayload { status: "agent_name is required for delete action".to_string() }))
            }
        }
        "deploy" => {
            let result = deploy_agent().await;
            Ok(warp::reply::json(&ResponsePayload { status: result }))
        }
        _ => Ok(warp::reply::json(&ResponsePayload { status: "Invalid action".to_string() })),
    }
}

async fn delete_agent(agent_name: &str) -> String {
    let output = Command::new("juju")
        .args(&["remove-application", "--force", "--no-wait", "--no-prompt", agent_name])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

async fn deploy_agent() -> String {
    let output = Command::new("juju")
        .args(&["deploy", "/home/k8s/testflinger-agent-charm-configs/ce-cert/ce-tf-agents-new-tel-l4.yaml"])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[tokio::main]
async fn main() {
    // Define the warp filter
    let api = warp::post()
        .and(warp::path("agent"))
        .and(warp::body::json())
        .and_then(handle_request);

    // Start the server
    println!("Starting server on http://0.0.0.0:3030/agent");
    warp::serve(api).run(([0, 0, 0, 0], 3030)).await;
}

