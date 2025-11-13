// ============================================================================
// ⚙️  Forge IDE - Main Entry Point
// Description:
//   The executable entry for the Forge IDE generator and runtime modules.
//   This CLI allows you to trigger module generation, rendering, or
//   runtime operations for schema, command, provider, and router modules.
// ============================================================================

mod schema;
mod command;
mod provider;
mod router;

use std::env;
use std::process::exit;

fn print_help() {
    println!(
        r#"
🔥 Forge IDE CLI

USAGE:
    forge-ide <command>

COMMANDS:
    build       Rebuild all Forge IDE modules from templates
    render      Render all modules into crates/forge-ide/src/
    list        List available Forge IDE modules
    help        Show this message

EXAMPLES:
    forge-ide build
    forge-ide render
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        exit(0);
    }

    match args[1].as_str() {
        // -----------------------------------------------------------------
        // 🏗️ Build: regenerate module YAMLs and render Rust files
        // -----------------------------------------------------------------
        "build" => {
            println!("⚙️  [Forge IDE] Building all modules...");
            let status = std::process::Command::new("./scripts/gen_forge_modules.zsh")
                .status()
                .expect("failed to execute generator script");
            if !status.success() {
                eprintln!("❌ Generation failed.");
                exit(1);
            }
            println!("✅ All modules rebuilt successfully!");
        }

        // -----------------------------------------------------------------
        // 🎨 Render: optional placeholder for later manual rendering logic
        // -----------------------------------------------------------------
        "render" => {
            println!("🎨 Rendering Forge IDE templates...");
            println!("(future extension: call forge-template APIs directly)");
        }

        // -----------------------------------------------------------------
        // 📜 List modules
        // -----------------------------------------------------------------
        "list" => {
            println!("📜 Available modules:");
            println!(" - schema");
            println!(" - command");
            println!(" - provider");
            println!(" - router");
        }

        // -----------------------------------------------------------------
        // ❓ Help / Unknown
        // -----------------------------------------------------------------
        "help" => print_help(),
        _ => {
            eprintln!("❌ Unknown command: {}", args[1]);
            print_help();
            exit(1);
        }
    }
}
