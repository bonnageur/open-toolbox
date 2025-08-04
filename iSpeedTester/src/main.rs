use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct SpeedTestResult {
    download: f64,
    upload: f64,
    ping: f64,
    server: Server,
}

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    name: String,
    country: String,
}

fn run_speedtest_cli() -> Result<SpeedTestResult, Box<dyn std::error::Error>> {
    println!("🚀 Running iSpeedTester via CLI...");

    let output = Command::new("speedtest-cli")
        .arg("--json")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Speedtest failed: {}", stderr).into());
    }

    let json_str = String::from_utf8(output.stdout)?;
    let result: SpeedTestResult = serde_json::from_str(&json_str)?;

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match run_speedtest_cli() {
        Ok(result) => {
            println!("\n✅ iSpeedTester Results");
            println!("📥 Download Speed  : {:.2} Mbps", result.download / 1_000_000.0);
            println!("📤 Upload Speed    : {:.2} Mbps", result.upload / 1_000_000.0);
            println!("🏓 Ping            : {:.2} ms", result.ping);
            println!("🌐 Server Location : {} - {}", result.server.name, result.server.country);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
