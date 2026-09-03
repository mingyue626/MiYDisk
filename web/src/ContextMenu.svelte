<script>
  export let x;
  export let y;
  export let items;
</script>

<div class="menu" style="left: {x}px; top: {y}px;">
  {#each items as item}
    <button class="item" class:danger={item.danger} on:click={item.action}>
      {item.label}
    </button>
  {/each}
</div>

<style>
  .menu {
    position: fixed;
    background: white;
    border: 1px solid #ccc;
    border-radius: 6px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    min-width: 170px;
    padding: 4px 0;
  }
  .item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 6px 14px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.85rem;
    color: #222;
  }
  .item:hover {
    background: #f0f0f0;
  }
  .item.danger {
    color: #dc2626;
  }
  .item.danger:hover {
    background: #fef2f2;
  }
</style>