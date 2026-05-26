use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use content::build_content_database_and_bundle;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

struct WatchConfig {
    content_root: PathBuf,
    output_dir: PathBuf,
    sentinel_file: PathBuf,
    debounce: Duration,
}

fn parse_args() -> WatchConfig {
    let mut args = std::env::args().skip(1);

    let mut content_root = PathBuf::from("contents");
    let mut output_dir = PathBuf::from("target/content-build");
    let mut sentinel_file = PathBuf::from("apps/frontend/src/content_reload_sentinel.rs");
    let mut debounce_ms = 600_u64;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--content-root" => {
                if let Some(value) = args.next() {
                    content_root = PathBuf::from(value);
                }
            }
            "--output-dir" => {
                if let Some(value) = args.next() {
                    output_dir = PathBuf::from(value);
                }
            }
            "--sentinel-file" => {
                if let Some(value) = args.next() {
                    sentinel_file = PathBuf::from(value);
                }
            }
            "--debounce-ms" => {
                if let Some(value) = args.next() {
                    debounce_ms = value.parse().unwrap_or(600);
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: content-watch [--content-root <path>] [--output-dir <path>] [--sentinel-file <path>] [--debounce-ms <ms>]\n\nDefaults:\n  --content-root contents\n  --output-dir target/content-build\n  --sentinel-file apps/frontend/src/content_reload_sentinel.rs\n  --debounce-ms 600"
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }

    WatchConfig {
        content_root,
        output_dir,
        sentinel_file,
        debounce: Duration::from_millis(debounce_ms),
    }
}

fn run_command(mut command: Command, name: &str) -> Result<(), String> {
    let status = command
        .status()
        .map_err(|error| format!("Failed to run {name}: {error}"))?;

    if status.success() {
        return Ok(());
    }

    Err(format!("{name} failed with status {status}"))
}

fn run_shiki_pipeline() -> Result<(), String> {
    run_command(
        {
            let mut command = Command::new("pnpm");
            command.args(["run", "langs:codeblock"]);
            command
        },
        "pnpm run langs:codeblock",
    )?;

    run_command(
        {
            let mut command = Command::new("pnpm");
            command.args(["run", "build:shiki"]);
            command
        },
        "pnpm run build:shiki",
    )
}

fn run_signature_generation(output_dir: &Path) -> Result<(), String> {
    let output = output_dir
        .join("public")
        .join("minia")
        .join("signature.png");

    run_command(
        {
            let mut command = Command::new("cargo");
            command.args([
                "run",
                "--package",
                "content",
                "--bin",
                "generate-signature",
                "--",
                "--output",
            ]);
            command.arg(output.as_os_str());
            command
        },
        "cargo run --package content --bin generate-signature",
    )
}

fn write_reload_sentinel(path: &Path) -> Result<u64, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create sentinel directory: {error}"))?;
    }

    let now_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("Failed to read system clock: {error}"))?
        .as_secs();

    let content = format!("pub const CONTENT_RELOAD_UNIX: u64 = {now_unix};\n");
    fs::write(path, content).map_err(|error| format!("Failed to write sentinel file: {error}"))?;

    Ok(now_unix)
}

fn run_rebuild(config: &WatchConfig) -> Result<(), String> {
    println!("[content-watch] Running shiki generation...");
    run_shiki_pipeline()?;

    println!("[content-watch] Rebuilding content database...");
    let (database, bundle) =
        build_content_database_and_bundle(&config.content_root, &config.output_dir)
            .map_err(|error| format!("Content build failed: {error}"))?;

    println!(
        "[content-watch] Content build success: {} entries -> {}",
        database.entries.len(),
        bundle.root_dir.display()
    );

    println!("[content-watch] Regenerating signature...");
    run_signature_generation(&config.output_dir)?;

    let timestamp = write_reload_sentinel(&config.sentinel_file)?;
    println!(
        "[content-watch] Updated reload sentinel at {} ({timestamp})",
        config.sentinel_file.display()
    );

    Ok(())
}

fn event_paths(event: &Event) -> Vec<String> {
    event
        .paths
        .iter()
        .map(|path| path.display().to_string())
        .collect()
}

fn is_generated_path(path: &str, config: &WatchConfig) -> bool {
    let output_dir = config.output_dir.display().to_string();
    let sentinel = config.sentinel_file.display().to_string();

    path.starts_with(&output_dir)
        || path == sentinel
        || path.ends_with("apps/frontend/shiki.entry.js")
        || path.ends_with("apps/frontend/public/js/shiki.bundle.js")
}

fn keep_relevant_paths(paths: Vec<String>, config: &WatchConfig) -> Vec<String> {
    paths
        .into_iter()
        .filter(|path| !is_generated_path(path, config))
        .collect()
}

fn main() {
    let config = parse_args();

    println!(
        "[content-watch] Watching '{}' (debounce {}ms)",
        config.content_root.display(),
        config.debounce.as_millis()
    );

    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = RecommendedWatcher::new(
        move |result| {
            let _ = tx.send(result);
        },
        Config::default(),
    )
    .expect("Failed to initialize file watcher");

    watcher
        .watch(&config.content_root, RecursiveMode::Recursive)
        .expect("Failed to watch content root");
    watcher
        .watch(
            Path::new("scripts/list-codeblock-langs.mjs"),
            RecursiveMode::NonRecursive,
        )
        .expect("Failed to watch shiki language script");
    watcher
        .watch(Path::new("package.json"), RecursiveMode::NonRecursive)
        .expect("Failed to watch package.json");

    if let Err(error) = run_rebuild(&config) {
        eprintln!("[content-watch] Initial rebuild failed: {error}");
    }

    loop {
        let mut changed_paths = match rx.recv() {
            Ok(Ok(event)) => event_paths(&event),
            Ok(Err(error)) => {
                eprintln!("[content-watch] Watch error: {error}");
                continue;
            }
            Err(error) => {
                eprintln!("[content-watch] Watch channel closed: {error}");
                break;
            }
        };

        loop {
            match rx.recv_timeout(config.debounce) {
                Ok(Ok(event)) => changed_paths.extend(event_paths(&event)),
                Ok(Err(error)) => eprintln!("[content-watch] Watch error: {error}"),
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    eprintln!("[content-watch] Watch channel disconnected");
                    return;
                }
            }
        }

        changed_paths = keep_relevant_paths(changed_paths, &config);
        changed_paths.sort();
        changed_paths.dedup();

        if changed_paths.is_empty() {
            continue;
        }

        println!(
            "[content-watch] Detected {} file change(s): {}",
            changed_paths.len(),
            changed_paths.join(", ")
        );

        if let Err(error) = run_rebuild(&config) {
            eprintln!("[content-watch] Rebuild failed: {error}");
        }
    }
}
