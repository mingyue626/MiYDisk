<script>
  import { onMount } from "svelte";
  import TreeNode from "./Treemap.svelte";
  import { humanSize } from "./format.js";

  const { invoke } = window.__TAURI__.core;
  const { listen } = window.__TAURI__.event;

  let path = "";
  let status = "等待开始";
  let tree = null;

  onMount(() => {
    listen("scan-progress", (event) => {
      const p = event.payload;
      if (p.EnteredDirectory) {
        status = `扫描中：${p.EnteredDirectory.path}`;
      } else if (p.Error) {
        console.warn("跳过：", p.Error.path, p.Error.message);
      }
    });

    listen("scan-complete", (event) => {
      tree = event.payload;
      status = `完成，共 ${humanSize(tree.total_size)}`;
    });

    listen("scan-error", (event) => {
      status = `扫描失败：${event.payload}`;
    });
  });

  function startScan() {
    const p = path.trim();
    if (!p) {
      status = "请先输入要扫描的路径";
      return;
    }
    status = "开始扫描…";
    tree = null;
    invoke("start_scan", { path: p }).catch((err) => {
      status = `调用失败：${err}`;
    });
  }
</script>

<main>
  <h1>MiYDisk</h1>
  <div class="controls">
    <input type="text" placeholder="要扫描的目录路径，如 C:\Users\me" bind:value={path} />
    <button on:click={startScan}>开始扫描</button>
  </div>
  <p class="status">{status}</p>

  {#if tree}
    <div class="tree-container">
      <TreeNode node={tree} depth={0} parentTotal={tree.total_size} expandedInitially={true} />
    </div>
  {/if}
</main>

<style>
  main {
    font-family: -apple-system, "Segoe UI", sans-serif;
    margin: 1.5rem;
    color: #222;
    display: flex;
    flex-direction: column;
    height: calc(100vh - 3rem);
  }
  .controls {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }
  .controls input {
    flex: 1;
    padding: 0.4rem 0.6rem;
  }
  .status {
    color: #666;
    font-size: 0.85rem;
    margin: 0.25rem 0;
  }
  .tree-container {
    flex: 1;
    overflow: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
</style>