const dropZone = document.getElementById("drop-zone");
const fileInput = document.getElementById("file-input");
const hint = document.getElementById("hint");
const statusList = document.getElementById("status-list");

const worker = new Worker("./worker.js", { type: "module" });

let workerReady = false;
let processing = false;
let seq = 0;

const queue = [];
const inFlight = new Map();

worker.addEventListener("message", (event) => {
  const msg = event.data;

  if (msg.type === "ready") {
    workerReady = true;
    setHint("拖拽 .ncm 文件到上方区域，或点击选择文件");
    pump();
    return;
  }

  const item = inFlight.get(msg.id);
  if (!item) return;
  inFlight.delete(msg.id);

  if (msg.ok) {
    const name = outputName(item.name, msg.format);
    download(name, msg.data);
    setRow(item.row, "success", `已下载 ${name}`);
    addRedownload(item.row, name, msg.data);
  } else {
    setRow(item.row, "error", msg.error);
  }

  processing = false;
  pump();
});

function pump() {
  if (!workerReady || processing) return;

  const item = queue.shift();
  if (!item) return;

  processing = true;
  inFlight.set(item.id, item);
  setRow(item.row, "working", "正在解压…");
  worker.postMessage({ id: item.id, bytes: item.bytes }, [item.bytes.buffer]);
}

async function handleFiles(files) {
  for (const file of Array.from(files)) {
    const row = addRow(file.name);

    if (!file.name.toLowerCase().endsWith(".ncm")) {
      setRow(row, "error", "不是 .ncm 文件，已跳过");
      continue;
    }

    const bytes = new Uint8Array(await file.arrayBuffer());
    queue.push({ id: seq++, row, name: file.name, bytes });
    pump();
  }
}

function addRow(name) {
  const li = document.createElement("li");
  li.className = "item";

  const nameSpan = document.createElement("span");
  nameSpan.className = "name";
  nameSpan.textContent = name;

  const statusSpan = document.createElement("span");
  statusSpan.className = "status";

  li.append(nameSpan, statusSpan);
  statusList.appendChild(li);

  return { li, status: statusSpan };
}

function setRow({ li, status }, state, text) {
  status.className = `status ${state}`;
  status.textContent = text;
  li.classList.add("done");
}

function setHint(text) {
  if (hint) hint.textContent = text;
}

function outputName(name, format) {
  return `${name.replace(/\.ncm$/i, "")}.${format}`;
}

function download(name, bytes) {
  const blob = new Blob([bytes], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = name;
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

function addRedownload({ li }, name, bytes) {
  const blob = new Blob([bytes], { type: "application/octet-stream" });
  const link = document.createElement("a");
  link.className = "redownload";
  link.href = URL.createObjectURL(blob);
  link.download = name;
  link.textContent = "重新下载";
  li.appendChild(link);
}

dropZone.addEventListener("click", () => fileInput.click());
dropZone.addEventListener("keydown", (event) => {
  if (event.key === "Enter" || event.key === " ") {
    event.preventDefault();
    fileInput.click();
  }
});
dropZone.addEventListener("dragover", (event) => {
  event.preventDefault();
  dropZone.classList.add("dragover");
});
dropZone.addEventListener("dragleave", () => {
  dropZone.classList.remove("dragover");
});
dropZone.addEventListener("drop", (event) => {
  event.preventDefault();
  dropZone.classList.remove("dragover");
  handleFiles(event.dataTransfer.files);
});

fileInput.addEventListener("change", () => {
  const files = Array.from(fileInput.files);
  fileInput.value = "";
  handleFiles(files);
});

window.addEventListener("dragover", (event) => event.preventDefault());
window.addEventListener("drop", (event) => event.preventDefault());
