cat > src/main.rs << 'EOF'
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{self, Write};

const SYSTEM_PROMPT: &str = include_str!("../prompts/queet.md");

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").context("set ANTHROPIC_API_KEY first")?;
    let model = std::env::var("QUEET_MODEL").unwrap_or_else(|_| "claude-sonnet-5-5".into());
    let client = reqwest::Client::new();
    let mut history: Vec<Value> = Vec::new();

    println!("✨ Queet is ready. Type 'exit' to quit.\n");
    loop {
        print!("you › ");
        io::stdout().flush()?;
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            break;
        }

        history.push(json!({"role": "user", "content": input}));
        let resp: Value = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": model,
                "max_tokens": 1500,
                "system": SYSTEM_PROMPT,
                "messages": history
            }))
            .send()
            .await?
            .json()
            .await?;

        match resp["content"][0]["text"].as_str() {
            Some(text) => {
                println!("\nqueet › {text}\n");
                history.push(json!({"role": "assistant", "content": text}));
            }
            None => {
                eprintln!("\nAPI error: {resp}\n");
                history.pop();
            }
        }
    }
    Ok(())
}
EOF
