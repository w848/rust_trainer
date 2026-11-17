// Rust Trainer: the page. Talks to the local Rust server (src/main.rs),
// which compiles your code with the real rustc.
"use strict";

const $ = (id) => document.getElementById(id);
const code = $("code");
const gutter = $("gutter");
const terminal = $("terminal");

let exercises = [];
let current = 0;
let busy = false;
let loadedId = null; // the exercise whose code is in the editor (none at first)

// ---- Saved progress (this browser only): your code and which exercises are solved
const store = {
  get(key, fallback) {
    try {
      const value = localStorage.getItem(key);
      return value === null ? fallback : JSON.parse(value);
    } catch {
      return fallback;
    }
  },
  set(key, value) {
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch {
      /* storage unavailable: progress just isn't remembered */
    }
  },
};
const solved = new Set(store.get("solved", []));

// ---- Start
async function start() {
  try {
    const [list, info] = await Promise.all([
      fetch("/api/exercises").then((r) => r.json()),
      fetch("/api/info").then((r) => r.json()),
    ]);
    exercises = list;
    $("rustc-version").textContent = info.rustc;
  } catch {
    $("ex-title").textContent = "Can't reach the Rust Trainer server";
    $("ex-lesson").innerHTML = "<p>Start it with <code>cargo run --release</code> in the rust_trainer folder, then reload this page.</p>";
    return;
  }
  buildList();
  const saved = exercises.findIndex((e) => e.id === store.get("current", ""));
  open(saved >= 0 ? saved : 0);
  print("Welcome! Write your code, then press ✔ Check (Ctrl+Enter) to test it,\nor ▶ Run (Ctrl+Shift+Enter) to see what main() prints.\n", "dim");
}

function buildList() {
  const list = $("exercise-list");
  list.replaceChildren();
  exercises.forEach((ex, index) => {
    if (index === 0 || exercises[index - 1].level !== ex.level) {
      const group = exercises.filter((e) => e.level === ex.level);
      const done = group.filter((e) => solved.has(e.id)).length;
      const heading = document.createElement("li");
      heading.className = "group";
      heading.textContent = `${ex.level}  ·  ${done}/${group.length}`;
      list.append(heading);
    }
    const li = document.createElement("li");
    li.classList.toggle("current", index === current);
    li.classList.toggle("solved", solved.has(ex.id));
    li.innerHTML = `<span class="mark">${solved.has(ex.id) ? "✔" : index + 1}</span><span></span>`;
    li.lastChild.append(ex.title);
    const topic = document.createElement("span");
    topic.className = "topic-small";
    topic.textContent = ex.topic;
    li.lastChild.append(topic);
    li.addEventListener("click", () => open(index));
    list.append(li);
  });
  $("progress-text").textContent = `${solved.size} / ${exercises.length} solved`;
  $("progress-fill").style.width = `${exercises.length ? (solved.size / exercises.length) * 100 : 0}%`;
}

function open(index) {
  saveCode();
  current = index;
  const ex = exercises[index];
  store.set("current", ex.id);
  $("ex-topic").textContent = `${index + 1}. ${ex.topic}`;
  $("ex-title").textContent = ex.title;
  $("ex-lesson").innerHTML = markdown(ex.lesson);
  $("hint-box").hidden = true;
  $("solution-wrap").hidden = true;
  $("solution-btn").textContent = "👁 Solution";
  $("solution").textContent = ex.solution;
  code.value = store.get(`code:${ex.id}`, ex.starter);
  loadedId = ex.id;
  updateGutter();
  $("prev-btn").disabled = index === 0;
  $("next-btn").disabled = index === exercises.length - 1;
  buildList();
  document.querySelector(".lesson").scrollTop = 0;
}

function saveCode() {
  if (loadedId) store.set(`code:${loadedId}`, code.value);
}

// ---- Buttons
$("run-btn").addEventListener("click", () => runCode("run"));
$("check-btn").addEventListener("click", () => runCode("test"));
$("prev-btn").addEventListener("click", () => open(current - 1));
$("next-btn").addEventListener("click", () => open(current + 1));
$("clear-btn").addEventListener("click", () => terminal.replaceChildren());
$("hint-btn").addEventListener("click", () => {
  const box = $("hint-box");
  box.hidden = false;
  box.innerHTML = "💡 " + inline(escapeHtml(exercises[current].hint));
});
$("solution-btn").addEventListener("click", () => {
  const wrap = $("solution-wrap");
  if (wrap.hidden && !solved.has(exercises[current].id) &&
      !confirm("Try a bit more first? Looking at the solution is fine, but you learn the most by struggling a little.\n\nShow the solution anyway?")) {
    return;
  }
  wrap.hidden = !wrap.hidden;
  $("solution-btn").textContent = wrap.hidden ? "👁 Solution" : "🙈 Hide solution";
});
$("reset-btn").addEventListener("click", () => {
  if (!confirm("Replace your code with the starting code for this exercise?")) return;
  code.value = exercises[current].starter;
  saveCode();
  updateGutter();
});

// ---- Running code on the server
async function runCode(mode) {
  if (busy || !exercises.length) return;
  busy = true;
  $("run-btn").disabled = $("check-btn").disabled = true;
  saveCode();
  const ex = exercises[current];
  print(`\n$ ${mode === "test" ? "rustc --test main.rs && ./main   # checking " + ex.title : "rustc main.rs && ./main"}\n`, "cmd");
  print("Compiling…\n", "dim");
  const started = performance.now();
  try {
    const response = await fetch("/api/run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ exercise: ex.id, code: code.value, mode }),
    });
    if (!response.ok) throw new Error(await response.text());
    const result = await response.json();
    terminal.lastChild?.remove(); // the "Compiling…" line
    showResult(result, mode, ex, performance.now() - started);
  } catch (error) {
    print(`Could not reach the server: ${error.message}\nIs "cargo run" still running?\n`, "error");
  } finally {
    busy = false;
    $("run-btn").disabled = $("check-btn").disabled = false;
    code.focus();
  }
}

function showResult(result, mode, ex, ms) {
  if (result.compiler_output.trim()) printCompiler(result.compiler_output);

  if (!result.compiled) {
    banner("fail", result.timed_out ? "⏱ The compiler took too long." : "✘ It doesn't compile yet. Read the first error above: it says the line and often how to fix it.");
    return;
  }

  if (mode === "run") {
    print(result.output || "(the program printed nothing)\n", result.output ? "" : "dim");
    if (/not yet implemented/.test(result.output)) {
      print("\nℹ todo!() is still in your code: replace it with your own code.\n", "help");
    }
    print(`\n[finished in ${(ms / 1000).toFixed(1)} s]\n`, "dim");
    return;
  }

  // Test mode: one line per test, then the failure details
  for (const test of result.tests) {
    print(test.passed ? `  ✔ ${test.name}\n` : `  ✘ ${test.name}\n`, test.passed ? "ok" : "error");
  }
  const failures = result.output.split(/\n(?=---- )/).filter((part) => part.startsWith("---- "));
  for (const failure of failures) {
    print("\n" + failure.replace(/\nfailures:[\s\S]*$/, "").trim() + "\n", "error");
  }
  if (result.timed_out) print("\n⏱ The tests took more than 5 seconds: is there an endless loop?\n", "error");

  const passedCount = result.tests.filter((t) => t.passed).length;
  if (result.passed) {
    const first = !solved.has(ex.id);
    solved.add(ex.id);
    store.set("solved", [...solved]);
    buildList();
    banner("success", `🎉 All ${result.tests.length} tests pass!${first ? " Exercise solved." : ""} Compare with 👁 Solution, then go to Next →`);
  } else if (/not yet implemented/.test(result.output)) {
    banner("fail", `✘ ${passedCount} / ${result.tests.length} tests pass. There's still a todo!() to replace with your code.`);
  } else {
    banner("fail", `✘ ${passedCount} / ${result.tests.length} tests pass. Look at "left" (your result) and "right" (expected) above.`);
  }
}

// ---- Terminal output
function print(text, cls = "") {
  const span = document.createElement("span");
  if (cls) span.className = cls;
  span.textContent = text;
  terminal.append(span);
  terminal.scrollTop = terminal.scrollHeight;
}

function banner(kind, text) {
  const div = document.createElement("span");
  div.className = `banner ${kind}`;
  div.textContent = text;
  terminal.append(div);
  terminal.scrollTop = terminal.scrollHeight;
}

// Colors rustc's messages: errors red, warnings yellow, help/notes teal
function printCompiler(text) {
  let cls = "";
  for (const line of text.split("\n")) {
    if (/^error/.test(line)) cls = "error";
    else if (/^warning/.test(line)) cls = "warning";
    else if (/^\s*(=\s*)?(help|note)/.test(line)) cls = "help";
    else if (line.trim() === "") cls = "";
    print(line + "\n", cls);
  }
}

// ---- Editor comfort: line numbers, Tab, auto-indent, shortcuts
function updateGutter() {
  const lines = code.value.split("\n").length;
  gutter.textContent = Array.from({ length: lines }, (_, i) => i + 1).join("\n");
  gutter.scrollTop = code.scrollTop;
}
let saveTimer;
code.addEventListener("input", () => {
  updateGutter();
  clearTimeout(saveTimer);
  saveTimer = setTimeout(saveCode, 500);
});
code.addEventListener("scroll", () => (gutter.scrollTop = code.scrollTop));

code.addEventListener("keydown", (e) => {
  if (e.key === "Enter" && e.ctrlKey) {
    e.preventDefault();
    runCode(e.shiftKey ? "run" : "test");
  } else if (e.key === "Tab") {
    e.preventDefault();
    insert(e.shiftKey ? null : "    ");
  } else if (e.key === "Enter") {
    // keep the indentation of the line, one level more after "{"
    e.preventDefault();
    const before = code.value.slice(0, code.selectionStart);
    const line = before.slice(before.lastIndexOf("\n") + 1);
    const indent = line.match(/^\s*/)[0];
    insert("\n" + indent + (/[{(\[]\s*$/.test(line) ? "    " : ""));
  } else if (e.key === "}" ) {
    // typing "}" on an empty line moves it back one level
    const before = code.value.slice(0, code.selectionStart);
    const line = before.slice(before.lastIndexOf("\n") + 1);
    if (/^ {4,}$/.test(line)) {
      e.preventDefault();
      code.setRangeText("}", code.selectionStart - 4, code.selectionEnd, "end");
      code.dispatchEvent(new Event("input"));
    }
  }
});

// Inserts text at the cursor (keeps Ctrl+Z working); null = remove one indent
function insert(text) {
  if (text === null) {
    const start = code.value.lastIndexOf("\n", code.selectionStart - 1) + 1;
    if (code.value.slice(start, start + 4) === "    ") {
      code.setSelectionRange(start, start + 4);
      document.execCommand("insertText", false, "");
    }
  } else if (!document.execCommand("insertText", false, text)) {
    code.setRangeText(text, code.selectionStart, code.selectionEnd, "end");
  }
  code.dispatchEvent(new Event("input"));
}

document.addEventListener("keydown", (e) => {
  if (e.key === "Enter" && e.ctrlKey && document.activeElement !== code) {
    e.preventDefault();
    runCode(e.shiftKey ? "run" : "test");
  }
});
window.addEventListener("beforeunload", saveCode);

// ---- A tiny Markdown reader for the lessons
function escapeHtml(text) {
  return text.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);
}
function inline(text) {
  return text
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
}
function markdown(source) {
  const html = [];
  source = source.replace(/\r\n?/g, "\n"); // files saved on Windows use \r\n
  const blocks = source.split(/```(?:rust)?\n([\s\S]*?)```/);
  blocks.forEach((block, i) => {
    if (i % 2 === 1) {
      html.push(`<pre><code>${escapeHtml(block.replace(/\n$/, ""))}</code></pre>`);
      return;
    }
    for (const para of block.split(/\n\s*\n/)) {
      const lines = para.trim().split("\n");
      if (!lines[0]) continue;
      if (lines.every((l) => /^\s*[-*] /.test(l))) {
        html.push("<ul>" + lines.map((l) => `<li>${inline(escapeHtml(l.replace(/^\s*[-*] /, "")))}</li>`).join("") + "</ul>");
      } else if (/^#{1,3} /.test(lines[0])) {
        html.push(`<h3>${inline(escapeHtml(lines[0].replace(/^#+ /, "")))}</h3>`);
      } else {
        html.push(`<p>${inline(escapeHtml(lines.join(" ")))}</p>`);
      }
    }
  });
  return html.join("\n");
}

start();
