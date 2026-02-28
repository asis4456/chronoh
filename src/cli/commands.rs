use crate::agents::{CoderAgent, InitializerAgent};
use crate::error::Result;
use crate::state::StateEngine;
use crate::types::{EndReason, EventType, Phase};
use clap::{Parser, Subcommand};
use std::path::Path;
use tracing::{error, info};

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        template: Option<String>,
    },
    Continue,
    Dev,
    Review {
        #[arg(short, long, default_value = "all")]
        scope: String,
    },
    Compact,
    Status,
}

pub async fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Init { name, template } => {
            info!("Initializing project: {}", name);

            let project_path = Path::new(&name);
            std::fs::create_dir_all(project_path)?;

            let state_path = project_path.join(".pi/state");
            let state = StateEngine::new(&state_path).await?;

            let initializer = InitializerAgent::new(state).await?;
            initializer
                .initialize(&name, template.as_deref(), project_path)
                .await?;

            println!("✅ Project '{}' initialized successfully!", name);
            println!("📁 Location: ./{}", name);
            println!("🚀 Next step: cd {} && chrono dev", name);
        }

        Commands::Continue | Commands::Dev => {
            let current_dir = std::env::current_dir()?;
            let state_path = current_dir.join(".pi/state");

            if !state_path.exists() {
                error!("Not a ChronoH project. Run 'chrono init' first.");
                return Ok(());
            }

            let state = StateEngine::new(&state_path).await?;
            let phase = state.get_current_phase().await?;

            println!("📊 Current phase: {:?}", phase);

            match phase {
                Phase::InfrastructureReady => {
                    println!("🚀 Starting Coder session...");
                    let mut coder = CoderAgent::new(state, 50).await?;
                    coder.start_session(&current_dir).await?;

                    println!("✅ Coder session ready. Waiting for tasks...");
                    println!("   (This is a placeholder - pi-rs integration needed)");

                    for i in 0..3 {
                        coder.increment_turn().await?;
                        println!("   Turn {}/50 completed", i + 1);
                    }

                    coder
                        .end_session(&current_dir, EndReason::TaskCompleted)
                        .await?;
                    println!("✅ Session ended cleanly");
                }
                _ => {
                    println!("🎯 Project phase: {:?}", phase);
                    println!("   (Continuing development would happen here)");
                }
            }
        }

        Commands::Review { scope } => {
            println!("🔍 Running review (scope: {})...", scope);
            println!("   (Review functionality to be implemented)");
        }

        Commands::Compact => {
            println!("📦 Triggering context compaction...");
            println!("   (Compaction functionality to be implemented)");
        }

        Commands::Status => {
            let current_dir = std::env::current_dir()?;
            let state_path = current_dir.join(".pi/state");

            if !state_path.exists() {
                println!("❌ Not a ChronoH project");
                println!("   Run 'chrono init --name <project>' to create one");
                return Ok(());
            }

            let state = StateEngine::new(&state_path).await?;
            let phase = state.get_current_phase().await?;
            let events = state.get_all_events().await?;

            println!("📊 Project Status");
            println!("═══════════════════════════════════════");
            println!("Current phase: {:?}", phase);
            println!("Total events: {}", events.len());

            if let Some(last_session) = state.get_last_session().await? {
                println!("Last activity: {}", last_session.timestamp);
                if let EventType::SessionStart { role } = &last_session.event_type {
                    println!("Last role: {:?}", role);
                } else if let EventType::SessionEnd { reason: _, turns_used: _ } = &last_session.event_type {
                    println!("Session ended");
                }
            }
        }
    }

    Ok(())
}
