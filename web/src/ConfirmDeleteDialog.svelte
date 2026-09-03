<script>
  export let node;
  export let onConfirm;
  export let onCancel;

  let dontAskAgain = false;
</script>

<div
  class="overlay"
  role="button"
  tabindex="-1"
  on:click={onCancel}
  on:keydown={(e) => e.key === "Escape" && onCancel()}
>
  <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="dialog" role="dialog" on:click|stopPropagation>
    <h3>永久删除</h3>
    <p>
      确定要永久删除 <strong>{node.name}</strong> 吗？此操作<strong>不可恢复</strong>，不会经过回收站。
    </p>
    <label class="checkbox-row">
      <input type="checkbox" bind:checked={dontAskAgain} />
      今日不再提醒
    </label>
    <div class="actions">
      <button on:click={onCancel}>取消</button>
      <button class="danger" on:click={() => onConfirm(dontAskAgain)}>永久删除</button>
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
    width: 340px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  }
  h3 {
    margin: 0 0 0.5rem;
    font-size: 1rem;
  }
  p {
    font-size: 0.85rem;
    color: #333;
    line-height: 1.5;
  }
  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.8rem;
    color: #555;
    margin: 0.75rem 0;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }
  .actions button {
    padding: 0.4rem 0.9rem;
    border-radius: 4px;
    border: 1px solid #ccc;
    background: #f5f5f5;
    cursor: pointer;
    font-size: 0.85rem;
  }
  .actions .danger {
    background: #dc2626;
    color: white;
    border-color: #dc2626;
  }
</style>