<script>
  import { humanSize } from "./format.js";

  export let groups; // DuplicateGroup[]: { size, hash, paths }
  export let onClose;
  export let onReveal; // (path) => void
  export let onTrash; // (path) => void

  $: totalWasted = groups.reduce((s, g) => s + g.size * (g.paths.length - 1), 0);
</script>

<div class="overlay" role="button" tabindex="-1" on:click={onClose} on:keydown={(e) => e.key === "Escape" && onClose()}>
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="dialog" role="dialog" on:click|stopPropagation>
    <div class="header">
      <h3>重复文件</h3>
      <button class="close" on:click={onClose}>✕</button>
    </div>

    {#if groups.length === 0}
      <p class="empty">没有发现重复文件。</p>
    {:else}
      <p class="summary">
        共 {groups.length} 组重复，删除多余副本可节省约 <strong>{humanSize(totalWasted)}</strong>
      </p>
      <div class="groups">
        {#each groups as group (group.hash)}
          <div class="group">
            <div class="group-head">{humanSize(group.size)} × {group.paths.length}</div>
            {#each group.paths as path (path)}
              <div class="path-row">
                <span class="path" title={path}>{path}</span>
                <button on:click={() => onReveal(path)}>定位</button>
                <button class="danger" on:click={() => onTrash(path)}>移到回收站</button>
              </div>
            {/each}
          </div>
        {/each}
      </div>
    {/if}
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
    width: 560px;
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  h3 {
    margin: 0;
    font-size: 1rem;
  }
  .close {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.9rem;
    color: #888;
  }
  .empty {
    color: #666;
    font-size: 0.85rem;
  }
  .summary {
    font-size: 0.85rem;
    color: #333;
    margin: 0.5rem 0 0.75rem;
  }
  .groups {
    overflow-y: auto;
    flex: 1;
  }
  .group {
    border: 1px solid #eee;
    border-radius: 6px;
    padding: 0.5rem 0.75rem;
    margin-bottom: 0.6rem;
  }
  .group-head {
    font-size: 0.8rem;
    font-weight: 600;
    color: #444;
    margin-bottom: 0.35rem;
  }
  .path-row {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.2rem 0;
    font-size: 0.78rem;
  }
  .path {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: monospace;
    color: #333;
  }
  .path-row button {
    flex-shrink: 0;
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    border: 1px solid #ccc;
    background: #f5f5f5;
    cursor: pointer;
    font-size: 0.75rem;
  }
  .path-row button.danger {
    color: #dc2626;
    border-color: #f5b5b5;
  }
</style>