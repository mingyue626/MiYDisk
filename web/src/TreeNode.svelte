<script>
  import { createEventDispatcher } from "svelte";
  import { humanSize } from "./format.js";

  export let node;
  export let depth = 0;
  export let parentTotal = node ? node.total_size : 0;
  export let expandedInitially = false;

  const dispatch = createEventDispatcher();
  let expanded = expandedInitially;

  $: hasChildren = node && node.children && node.children.length > 0;
  $: sortedChildren = hasChildren
    ? [...node.children].sort((a, b) => b.total_size - a.total_size)
    : [];
  $: percent = parentTotal > 0 ? Math.min(100, (node.total_size / parentTotal) * 100) : 0;

  function toggle() {
    if (hasChildren) expanded = !expanded;
  }

  function handleContextMenu(e) {
    e.preventDefault();
    dispatch("nodecontextmenu", { node, x: e.clientX, y: e.clientY });
  }

  function extensionHue(name) {
    const dot = name.lastIndexOf(".");
    const ext = dot > 0 ? name.slice(dot + 1).toLowerCase() : "";
    if (!ext) return 210;
    let hash = 0;
    for (let i = 0; i < ext.length; i++) {
      hash = (hash * 31 + ext.charCodeAt(i)) >>> 0;
    }
    return hash % 360;
  }

  $: barColor =
    node.node_type === "Directory"
      ? "hsl(220, 25%, 55%)"
      : `hsl(${extensionHue(node.name)}, 55%, 60%)`;
</script>

<div
  class="row"
  style="padding-left: {depth * 18}px"
  role="button"
  tabindex="0"
  on:contextmenu={handleContextMenu}
>
  <button
    class="toggle"
    class:invisible={!hasChildren}
    on:click={toggle}
    aria-label={expanded ? "折叠" : "展开"}
  >
    {#if hasChildren}{expanded ? "▾" : "▸"}{/if}
  </button>
  <span class="name" class:dir={node.node_type === "Directory"} title={node.path}>
    {node.name}
  </span>
  <span class="bar-track">
    <span class="bar-fill" style="width: {percent}%; background: {barColor};"></span>
  </span>
  <span class="size">{humanSize(node.total_size)}</span>
</div>

{#if expanded}
  {#each sortedChildren as child (child.id)}
    <svelte:self
      node={child}
      depth={depth + 1}
      parentTotal={node.total_size}
      on:nodecontextmenu
    />
  {/each}
{/if}

<style>
  .row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 6px;
    font-size: 0.85rem;
  }
  .row:hover {
    background: #eef2ff;
  }
  .toggle {
    width: 14px;
    flex-shrink: 0;
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    font-size: 0.7rem;
    color: #555;
  }
  .toggle.invisible {
    visibility: hidden;
  }
  .name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .name.dir {
    font-weight: 600;
  }
  .bar-track {
    width: 100px;
    height: 8px;
    background: #eee;
    border-radius: 4px;
    overflow: hidden;
    flex-shrink: 0;
  }
  .bar-fill {
    display: block;
    height: 100%;
  }
  .size {
    width: 80px;
    text-align: right;
    font-family: monospace;
    color: #444;
    flex-shrink: 0;
  }
</style>