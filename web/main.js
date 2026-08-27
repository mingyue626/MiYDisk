const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const scanBtn = document.getElementById("scan-btn");
const pathInput = document.getElementById("path-input");
const statusEl = document.getElementById("status");
const listEl = document.getElementById("result-list");

function humanSize(bytes) {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = bytes;
  let i = 0;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return `${size.toFixed(2)} ${units[i]}`;
}

function renderTopLevel(node) {
  listEl.innerHTML = "";
  const sorted = [...node.children].sort((a, b) => b.total_size - a.total_size);
  for (const child of sorted) {
    const li = document.createElement("li");
    li.textContent = `${humanSize(child.total_size)}  —  ${child.name}`;
    listEl.appendChild(li);
  }
}

listen("scan-progress", (event) => {
  const p = event.payload;
  if (p.EnteredDirectory) {
    statusEl.textContent = `扫描中：${p.EnteredDirectory.path}`;
  } else if (p.Error) {
    console.warn("跳过：", p.Error.path, p.Error.message);
  }
});

listen("scan-complete", (event) => {
  const tree = event.payload;
  statusEl.textContent = `完成，共 ${humanSize(tree.total_size)}`;
  renderTopLevel(tree);
});

listen("scan-error", (event) => {
  statusEl.textContent = `扫描失败：${event.payload}`;
});

scanBtn.addEventListener("click", () => {
  const path = pathInput.value.trim();
  if (!path) {
    statusEl.textContent = "请先输入要扫描的路径";
    return;
  }
  statusEl.textContent = "开始扫描…";
  listEl.innerHTML = "";
  invoke("start_scan", { path });
});