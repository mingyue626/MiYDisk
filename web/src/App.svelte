<script>
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import TreeNode from "./TreeNode.svelte";
  import ContextMenu from "./ContextMenu.svelte";
  import ConfirmDeleteDialog from "./ConfirmDeleteDialog.svelte";
  import DetailsDialog from "./DetailsDialog.svelte";
  import { humanSize } from "./format.js";
  import { shouldSkipConfirm, skipConfirmForToday } from "./permanentDeletePref.js";

  const { invoke } = window.__TAURI__.core;
  const { listen } = window.__TAURI__.event;

  let path = "";
  let status = "等待开始";
  let tree = null;

  let menu = null;
  let confirmDeleteNode = null;
  let infoNode = null;

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

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      path = selected;
    }
  }

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

  function handleNodeContextMenu(e) {
    const { node, x, y } = e.detail;
    menu = { node, x, y };
  }

  function closeMenu() {
    menu = null;
  }

  $: menuItems = menu
    ? [
        { label: "在资源管理器中打开", action: () => runAction("reveal", menu.node) },
        { label: "复制路径", action: () => runAction("copy", menu.node) },
        { label: "详细信息", action: () => runAction("info", menu.node) },
        { label: "移到回收站", action: () => runAction("trash", menu.node) },
        { label: "永久删除", danger: true, action: () => runAction("delete", menu.node) },
      ]
    : [];

  function runAction(action, node) {
    menu = null;
    if (action === "reveal") {
      invoke("reveal_in_explorer", { path: node.path }).catch((err) => {
        status = `打开失败：${err}`;
      });
    } else if (action === "copy") {
      navigator.clipboard.writeText(node.path).catch(() => {});
    } else if (action === "info") {
      infoNode = node;
    } else if (action === "trash") {
      invoke("move_to_trash", { path: node.path })
        .then(() => removeNodeFromTree(node))
        .catch((err) => {
          status = `移到回收站失败：${err}`;
        });
    } else if (action === "delete") {
      if (shouldSkipConfirm()) {
        performPermanentDelete(node);
      } else {
        confirmDeleteNode = node;
      }
    }
  }

  function performPermanentDelete(node) {
    invoke("delete_permanently", { path: node.path })
      .then(() => removeNodeFromTree(node))
      .catch((err) => {
        status = `删除失败：${err}`;
      });
  }

  function confirmPermanentDelete(dontAskAgain) {
    if (dontAskAgain) skipConfirmForToday();
    const node = confirmDeleteNode;
    confirmDeleteNode = null;
    performPermanentDelete(node);
  }

  function removeNodeFromTree(target) {
    if (!tree) return;
    tree = removeAndRecalc(tree, target.id);
  }

  function removeAndRecalc(node, targetId) {
    if (!node.children || node.children.length === 0) return node;
    const filteredChildren = node.children
      .filter((c) => c.id !== targetId)
      .map((c) => removeAndRecalc(c, targetId));
    const childrenSum = filteredChildren.reduce((s, c) => s + c.total_size, 0);
    return { ...node, children: filteredChildren, total_size: node.own_size + childrenSum };
  }
</script>

<svelte:window
  on:click={closeMenu}
  on:keydown={(e) => e.key === "Escape" && closeMenu()}
/>

<main>
  <h1>MiYDisk</h1>
  <div class="controls">
    <input type="text" placeholder="要扫描的目录路径，如 C:\Users\me" bind:value={path} />
    <button on:click={pickFolder}>浏览…</button>
    <button on:click={startScan}>开始扫描</button>
  </div>
  <p class="status">{status}</p>

  {#if tree}
    <div class="tree-container">
      <TreeNode
        node={tree}
        depth={0}
        parentTotal={tree.total_size}
        expandedInitially={true}
        on:nodecontextmenu={handleNodeContextMenu}
      />
    </div>
  {/if}
</main>

{#if menu}
  <ContextMenu x={menu.x} y={menu.y} items={menuItems} />
{/if}

{#if confirmDeleteNode}
  <ConfirmDeleteDialog
    node={confirmDeleteNode}
    onConfirm={confirmPermanentDelete}
    onCancel={() => (confirmDeleteNode = null)}
  />
{/if}

{#if infoNode}
  <DetailsDialog node={infoNode} onClose={() => (infoNode = null)} />
{/if}

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