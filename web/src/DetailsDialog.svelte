<script>
  import { humanSize } from "./format.js";

  export let node;
  export let onClose;

  function typeLabel(t) {
    if (t === "Directory") return "文件夹";
    if (t === "Symlink") return "符号链接";
    return "文件";
  }
</script>

<div
  class="overlay"
  role="button"
  tabindex="-1"
  on:click={onClose}
  on:keydown={(e) => e.key === "Escape" && onClose()}
>
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="dialog" role="dialog" on:click|stopPropagation>
    <h3>详细信息</h3>
    <dl>
      <dt>名称</dt>
      <dd>{node.name}</dd>
      <dt>路径</dt>
      <dd class="path">{node.path}</dd>
      <dt>类型</dt>
      <dd>{typeLabel(node.node_type)}</dd>
      <dt>大小</dt>
      <dd>{humanSize(node.total_size)}</dd>
      {#if node.error}
        <dt>扫描错误</dt>
        <dd class="error">{node.error}</dd>
      {/if}
    </dl>
    <div class="actions">
      <button on:click={onClose}>关闭</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.35);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }
  .dialog {
    background: white;
    border-radius: 8px;
    padding: 1.25rem 1.5rem;
    width: 380px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  }
  h3 {
    margin: 0 0 0.75rem;
    font-size: 1rem;
  }
  dl {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.35rem 0.75rem;
    font-size: 0.82rem;
  }
  dt {
    color: #888;
  }
  dd {
    margin: 0;
    word-break: break-all;
  }
  .path {
    font-family: monospace;
    font-size: 0.78rem;
  }
  .error {
    color: #dc2626;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
  }
  .actions button {
    padding: 0.4rem 0.9rem;
    border-radius: 4px;
    border: 1px solid #ccc;
    background: #f5f5f5;
    cursor: pointer;
    font-size: 0.85rem;
  }
</style>