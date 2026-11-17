//! Read-only readiness probe. No `turn/start` or model-generation operation is implemented until D5 isolation is proven.
use serde::Serialize;
use serde_json::{Value, json};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::timeout,
};

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
    #[error("Codex could not be started or stopped: {0}")]
    Io(#[from] std::io::Error),
    #[error("Codex did not respond within the readiness deadline.")]
    Timeout,
    #[error("Codex returned an unsupported or unsuccessful protocol response.")]
    Protocol,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Readiness {
    pub chatgpt_signed_in: bool,
    pub ambient_instruction_sources: usize,
    pub generation_enabled: bool,
    pub reason: &'static str,
}

struct Session {
    child: Child,
    input: ChildStdin,
    lines: Lines<BufReader<ChildStdout>>,
    sequence: u32,
}
impl Session {
    async fn send(&mut self, message: Value) -> Result<(), ProbeError> {
        let mut encoded = serde_json::to_vec(&message).map_err(|_| ProbeError::Protocol)?;
        encoded.push(b'\n');
        self.input.write_all(&encoded).await?;
        self.input.flush().await?;
        Ok(())
    }
    async fn request(&mut self, method: &str, params: Value) -> Result<Value, ProbeError> {
        self.sequence += 1;
        let id = self.sequence;
        self.send(json!({"id":id,"method":method,"params":params}))
            .await?;
        while let Some(line) = self.lines.next_line().await? {
            if line.len() > 4_000_000 {
                return Err(ProbeError::Protocol);
            }
            let message: Value = serde_json::from_str(&line).map_err(|_| ProbeError::Protocol)?;
            if message.get("id") == Some(&json!(id)) && message.get("method").is_none() {
                return message.get("result").cloned().ok_or(ProbeError::Protocol);
            }
            if message.get("id").is_some() && message.get("method").is_some() {
                self.send(json!({"id":message["id"],"error":{"code":-32601,"message":"This probe provides no tools or approvals."}})).await?;
            }
        }
        Err(ProbeError::Protocol)
    }
    async fn inspect(&mut self, cwd: &Path, mut config: Value) -> Result<Readiness, ProbeError> {
        self.request("initialize", json!({"clientInfo":{"name":"agentic_wiki_readiness","version":"0.1.0"},"capabilities":{"experimentalApi":true}})).await?;
        self.send(json!({"method":"initialized"})).await?;
        let account = self
            .request("account/read", json!({"refreshToken":false}))
            .await?;
        let signed_in = account.pointer("/account/type").and_then(Value::as_str) == Some("chatgpt");
        if !signed_in {
            return Ok(Readiness {
                chatgpt_signed_in: false,
                ambient_instruction_sources: 0,
                generation_enabled: false,
                reason: "Sign in to Codex with ChatGPT. There is no API-key fallback.",
            });
        }
        let effective = self
            .request("config/read", json!({"includeLayers":false}))
            .await?;
        if let Some(servers) = effective
            .pointer("/config/mcp_servers")
            .and_then(Value::as_object)
        {
            let disabled: serde_json::Map<String, Value> = servers
                .keys()
                .map(|name| (name.clone(), json!({"enabled":false})))
                .collect();
            config["mcp_servers"] = Value::Object(disabled);
        }
        let thread = self.request("thread/start", json!({"cwd":cwd,"ephemeral":true,"approvalPolicy":"never","sandbox":"read-only","environments":[],"runtimeWorkspaceRoots":[],"selectedCapabilityRoots":[],"dynamicTools":[],"baseInstructions":"Use only explicitly supplied application context. Do not use tools.","developerInstructions":"This is a read-only readiness check. No model turn is authorized by this probe.","config":config})).await?;
        let sources = thread
            .get("instructionSources")
            .and_then(Value::as_array)
            .ok_or(ProbeError::Protocol)?
            .len();
        Ok(Readiness {
            chatgpt_signed_in: true,
            ambient_instruction_sources: sources,
            generation_enabled: false,
            reason: if sources > 0 {
                "Ambient instruction files are inherited. D5 context isolation is not satisfied; no model turn was started."
            } else {
                "No ambient instruction files reported. Request/tool isolation still requires verification before generation can be enabled."
            },
        })
    }
}

pub async fn inspect(
    binary: &Path,
    cwd: &Path,
    deadline: Duration,
) -> Result<Readiness, ProbeError> {
    let mut config = json!({
        "project_doc_max_bytes":0,"project_doc_fallback_filenames":[],"web_search":"disabled",
        "plugins":{"_default":{"enabled":false}},"apps":{"_default":{"enabled":false}},
        "skills":{"include_instructions":false,"bundled":{"enabled":false}},"agents":{"enabled":false},
        "analytics":{"enabled":false},"include_apps_instructions":false,
        "features":{}
    });
    for name in [
        "shell_tool",
        "view_image",
        "apps",
        "connectors",
        "plugins",
        "code_mode",
        "code_mode_only",
        "code_mode_host",
        "js_repl",
        "memories",
        "memory_tool",
        "hooks",
        "codex_hooks",
        "plugin_hooks",
        "multi_agent",
        "multi_agent_v2",
        "collab",
        "browser_use",
        "computer_use",
        "image_generation",
        "imagegenext",
        "apply_patch_freeform",
        "tool_suggest",
        "tool_search",
        "search_tool",
        "skill_search",
        "skill_mcp_dependency_install",
        "shell_snapshot",
        "goals",
        "worktrees",
        "remote_plugin",
    ] {
        config["features"][name] = false.into();
    }
    config["features"]["skip_host_skill_discovery"] = true.into();
    let mut command = Command::new(binary);
    command
        .arg("app-server")
        .arg("--stdio")
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    // TOML inline tables are not JSON; encode each leaf as a dotted config assignment.
    fn overrides(command: &mut Command, prefix: &str, value: &Value) {
        if let Some(object) = value.as_object() {
            for (key, child) in object {
                overrides(
                    command,
                    &if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    },
                    child,
                );
            }
        } else {
            command.arg("-c").arg(format!("{prefix}={value}"));
        }
    }
    overrides(&mut command, "", &config);
    let mut child = command.spawn()?;
    let input = child.stdin.take().ok_or(ProbeError::Protocol)?;
    let output = child.stdout.take().ok_or(ProbeError::Protocol)?;
    let mut session = Session {
        child,
        input,
        lines: BufReader::new(output).lines(),
        sequence: 0,
    };
    let result = timeout(deadline, session.inspect(cwd, config)).await;
    session.child.start_kill()?;
    let _ = session.child.wait().await;
    result.map_err(|_| ProbeError::Timeout)?
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn missing_binary_is_a_visible_failure() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            inspect(
                &dir.path().join("missing-codex"),
                dir.path(),
                Duration::from_secs(1)
            )
            .await,
            Err(ProbeError::Io(_))
        ));
    }
    #[cfg(unix)]
    #[tokio::test]
    async fn unresponsive_child_times_out_and_is_reaped() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("unresponsive");
        std::fs::write(&script, "#!/bin/sh\nexec sleep 30\n").unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert!(matches!(
            inspect(&script, dir.path(), Duration::from_millis(100)).await,
            Err(ProbeError::Timeout)
        ));
    }
}
