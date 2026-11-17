//! Rust Trainer: a small local web server for practising Rust.
//!
//! - Serves the web page from `static/`.
//! - Lists the exercises found in `exercises/` (one folder per exercise).
//! - Compiles your code with the real `rustc`, runs it, or runs the
//!   exercise's tests against it, and sends the results back as JSON.
//!
//! Run it with `cargo run --release`, then open http://localhost:8090

use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};
use tiny_http::{Header, Method, Request, Response, Server};

const ADDRESS: &str = "127.0.0.1:8090"; // only reachable from this PC
const COMPILE_TIMEOUT: Duration = Duration::from_secs(30);
const RUN_TIMEOUT: Duration = Duration::from_secs(5);

/// What the page sends when you press Run or Check.
#[derive(Deserialize)]
struct RunRequest {
    exercise: String,
    code: String,
    mode: Mode,
}

#[derive(Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Mode {
    Run,  // compile as a program and run main()
    Test, // compile with the exercise's tests and run them
}

/// What the server answers.
#[derive(Serialize, Default)]
struct RunResult {
    compiled: bool,
    compiler_output: String, // errors and warnings from rustc
    output: String,          // what the program (or the test runner) printed
    tests: Vec<TestResult>,
    passed: bool, // Test mode: every test passed
    timed_out: bool,
}

#[derive(Serialize)]
struct TestResult {
    name: String,
    passed: bool,
}

/// One exercise, read from its folder.
#[derive(Serialize)]
struct Exercise {
    id: String,
    title: String,
    topic: String,
    hint: String,
    level: String,
    lesson: String,
    starter: String,
    solution: String,
}

#[derive(Deserialize)]
struct Meta {
    title: String,
    topic: String,
    hint: String,
    /// "Beginner" exercises are listed first; anything else (or nothing) is "Practice".
    #[serde(default)]
    level: String,
}

fn main() {
    let root = project_root();
    let rustc = find_rustc();
    let version = rustc_version(&rustc);
    let server = Server::http(ADDRESS).unwrap_or_else(|error| {
        eprintln!("Could not start the server on {ADDRESS}: {error}");
        eprintln!("Is Rust Trainer already running? Close the other one first.");
        std::process::exit(1);
    });

    println!("Rust Trainer is running at http://localhost:8090");
    println!("Compiler: {version}");
    println!("Press Enter to stop (or just close this window).");
    // Stop cleanly on Enter. Ctrl+C also works, but Windows then shows scary messages.
    // Ok(0) means there's no keyboard (stdin closed): keep running in that case.
    thread::spawn(|| {
        let mut line = String::new();
        if let Ok(1..) = std::io::stdin().read_line(&mut line) {
            println!("Rust Trainer stopped. Bye!");
            std::process::exit(0);
        }
    });
    if cfg!(windows) && std::env::var_os("RUST_TRAINER_NO_BROWSER").is_none() {
        let _ = Command::new("cmd")
            .args(["/C", "start", "", "http://localhost:8090"])
            .spawn(); // open the browser
    }

    for request in server.incoming_requests() {
        let root = root.clone();
        let rustc = rustc.clone();
        let version = version.clone();
        // One thread per request, so a slow compile doesn't block the page
        thread::spawn(move || handle(request, &root, &rustc, &version));
    }
}

fn handle(mut request: Request, root: &Path, rustc: &Path, version: &str) {
    let url = request.url().split('?').next().unwrap_or("/").to_string();
    let response = match (request.method(), url.as_str()) {
        (Method::Get, "/api/exercises") => json(&load_exercises(&root.join("exercises"))),
        (Method::Get, "/api/info") => json(&serde_json::json!({ "rustc": version })),
        (Method::Post, "/api/run") => {
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            match serde_json::from_str::<RunRequest>(&body) {
                Ok(run) => json(&run_code(&run, root, rustc)),
                Err(error) => text(400, &format!("bad request: {error}")),
            }
        }
        (Method::Get, path) => static_file(&root.join("static"), path),
        _ => text(405, "method not allowed"),
    };
    let _ = request.respond(response);
}

// ---------------------------------------------------------------------------
// Compiling and running code

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_code(run: &RunRequest, root: &Path, rustc: &Path) -> RunResult {
    // Each run gets its own folder, so two runs never mix their files
    let id = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("rust-trainer-{}-{id}", std::process::id()));
    if let Err(error) = fs::create_dir_all(&dir) {
        return RunResult {
            compiler_output: format!("could not create a temporary folder: {error}"),
            ..Default::default()
        };
    }
    let result = compile_and_run(run, root, rustc, &dir);
    let _ = fs::remove_dir_all(&dir);
    result
}

fn compile_and_run(run: &RunRequest, root: &Path, rustc: &Path, dir: &Path) -> RunResult {
    let mut result = RunResult::default();

    // Your code first (so error line numbers match the editor), the tests after it
    let mut source = run.code.clone();
    if run.mode == Mode::Test {
        let tests_path = exercise_dir(root, &run.exercise).map(|d| d.join("tests.rs"));
        match tests_path.and_then(|p| fs::read_to_string(p).ok()) {
            Some(tests) => source.push_str(&format!("\n\n#[cfg(test)]\nmod tests {{\n    use super::*;\n{tests}\n}}\n")),
            None => {
                result.compiler_output = format!("no tests found for exercise '{}'", run.exercise);
                return result;
            }
        }
    }
    if let Err(error) = fs::write(dir.join("main.rs"), source) {
        result.compiler_output = format!("could not write the code: {error}");
        return result;
    }

    let exe = dir.join(if cfg!(windows) { "main.exe" } else { "main" });
    let mut compile = Command::new(rustc);
    compile
        .current_dir(dir)
        .args(["--edition", "2021", "--color", "never", "-A", "dead_code", "-o"])
        .arg(&exe)
        .arg("main.rs");
    if run.mode == Mode::Test {
        compile.arg("--test");
    }
    let compiled = run_with_timeout(&mut compile, COMPILE_TIMEOUT);
    result.compiler_output = compiled.stderr;
    if compiled.timed_out {
        result.timed_out = true;
        result.compiler_output.push_str("\nThe compiler took too long and was stopped.");
        return result;
    }
    if !compiled.success {
        return result;
    }
    result.compiled = true;

    let mut program = Command::new(&exe);
    program.current_dir(dir);
    if run.mode == Mode::Test {
        program.args(["--test-threads", "1", "--color", "never"]);
    }
    let ran = run_with_timeout(&mut program, RUN_TIMEOUT);
    result.timed_out = ran.timed_out;
    result.output = ran.stdout;
    if !ran.stderr.is_empty() {
        if !result.output.is_empty() {
            result.output.push('\n');
        }
        result.output.push_str(&ran.stderr);
    }
    if ran.timed_out {
        result
            .output
            .push_str("\n⏱ Stopped after 5 seconds: is there an endless loop?");
    }

    if run.mode == Mode::Test {
        // Lines look like: "test tests::adds_numbers ... ok"
        for line in result.output.lines() {
            if let Some(rest) = line.strip_prefix("test ") {
                if let Some((name, outcome)) = rest.rsplit_once(" ... ") {
                    let name = name.trim_start_matches("tests::").replace('_', " ");
                    result.tests.push(TestResult {
                        name,
                        passed: outcome.trim() == "ok",
                    });
                }
            }
        }
        result.passed = ran.success && !result.tests.is_empty() && result.tests.iter().all(|t| t.passed);
    }
    result
}

struct Finished {
    success: bool,
    timed_out: bool,
    stdout: String,
    stderr: String,
}

/// Runs a command, but kills it if it takes longer than `limit`.
fn run_with_timeout(command: &mut Command, limit: Duration) -> Finished {
    let mut child = match command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Finished {
                success: false,
                timed_out: false,
                stdout: String::new(),
                stderr: format!("could not start {:?}: {error}", command.get_program()),
            }
        }
    };
    // Read both outputs in the background, so a chatty program can't block
    let mut out = child.stdout.take().expect("stdout is piped");
    let mut err = child.stderr.take().expect("stderr is piped");
    let out_reader = thread::spawn(move || {
        let mut text = String::new();
        let _ = out.read_to_string(&mut text);
        text
    });
    let err_reader = thread::spawn(move || {
        let mut text = String::new();
        let _ = err.read_to_string(&mut text);
        text
    });

    let started = Instant::now();
    let (success, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status.success(), false),
            Ok(None) if started.elapsed() > limit => {
                let _ = child.kill();
                let _ = child.wait();
                break (false, true);
            }
            Ok(None) => thread::sleep(Duration::from_millis(20)),
            Err(_) => break (false, false),
        }
    };
    Finished {
        success,
        timed_out,
        stdout: out_reader.join().unwrap_or_default(),
        stderr: err_reader.join().unwrap_or_default(),
    }
}

// ---------------------------------------------------------------------------
// Exercises

fn load_exercises(dir: &Path) -> Vec<Exercise> {
    let mut folders: Vec<PathBuf> = fs::read_dir(dir)
        .map(|entries| entries.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect())
        .unwrap_or_default();
    folders.sort(); // "01-...", "02-..." give the order
    let mut exercises: Vec<Exercise> = folders
        .iter()
        .filter_map(|folder| {
            let read = |name: &str| fs::read_to_string(folder.join(name)).ok();
            let meta: Meta = serde_json::from_str(&read("meta.json")?)
                .map_err(|error| eprintln!("{}: bad meta.json: {error}", folder.display()))
                .ok()?;
            Some(Exercise {
                id: folder.file_name()?.to_string_lossy().into_owned(),
                title: meta.title,
                topic: meta.topic,
                hint: meta.hint,
                level: if meta.level.is_empty() { "Practice".into() } else { meta.level },
                lesson: read("lesson.md").unwrap_or_default(),
                starter: read("starter.rs").unwrap_or_default(),
                solution: read("solution.rs").unwrap_or_default(),
            })
        })
        .collect();
    // Stable sort by group (Beginner, Practice, Intermediate, then any other level);
    // inside a group the folder order is kept.
    exercises.sort_by_key(|exercise| match exercise.level.as_str() {
        "Beginner" => 0,
        "Practice" => 1,
        "Intermediate" => 2,
        _ => 3,
    });
    exercises
}

/// The folder of an exercise, refusing names like "../x" that leave `exercises/`.
fn exercise_dir(root: &Path, id: &str) -> Option<PathBuf> {
    if id.is_empty() || id.contains(['/', '\\']) || id.contains("..") {
        return None;
    }
    let dir = root.join("exercises").join(id);
    dir.is_dir().then_some(dir)
}

// ---------------------------------------------------------------------------
// Small helpers

fn static_file(dir: &Path, url: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let name = if url == "/" { "index.html" } else { url.trim_start_matches('/') };
    if name.contains("..") || name.contains('\\') {
        return text(404, "not found");
    }
    match fs::read(dir.join(name)) {
        Ok(bytes) => {
            let kind = match Path::new(name).extension().and_then(|e| e.to_str()) {
                Some("html") => "text/html; charset=utf-8",
                Some("css") => "text/css; charset=utf-8",
                Some("js") => "text/javascript; charset=utf-8",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            Response::from_data(bytes).with_header(header("Content-Type", kind))
        }
        Err(_) => text(404, "not found"),
    }
}

fn json<T: Serialize>(value: &T) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_data(serde_json::to_vec(value).unwrap_or_default())
        .with_header(header("Content-Type", "application/json"))
}

fn text(status: u16, message: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_data(message.as_bytes().to_vec()).with_status_code(status)
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("valid header")
}

/// The folder with `static/` and `exercises/`: next to Cargo.toml.
fn project_root() -> PathBuf {
    let from_cargo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if from_cargo.join("static").is_dir() {
        return from_cargo;
    }
    std::env::current_dir().unwrap_or(from_cargo)
}

/// `rustc` from PATH, or from rustup's usual folder if PATH doesn't have it.
fn find_rustc() -> PathBuf {
    if Command::new("rustc").arg("--version").output().is_ok() {
        return PathBuf::from("rustc");
    }
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_default();
    home.join(".cargo").join("bin").join("rustc")
}

fn rustc_version(rustc: &Path) -> String {
    Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "rustc NOT FOUND: install Rust from https://rustup.rs".to_string())
}
