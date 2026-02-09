<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { uiState, type RenderState } from "$lib/stores/ui";

  onMount(async () => {
    const unlisten = await listen<RenderState>("render_update", (event) => {
      uiState.set(event.payload);
    });

    return () => {
      unlisten();
    };
  });

  function handleCandidateClick(index: number) {
    invoke("select_candidate", { index });
  }
</script>

{#if $uiState.visible && $uiState.candidates.length > 0}
  <div class="candidate-bar">
    <div class="candidate-list">
      {#each $uiState.candidates as candidate, i}
        <button
          class="candidate-item"
          class:selected={$uiState.highlight_index === i}
          on:click={() => handleCandidateClick(i)}
        >
          <span class="index" class:selected-index={$uiState.highlight_index === i}>
            {i + 1}
          </span>
          <span class="text">{candidate}</span>
        </button>
      {/each}
    </div>
    
    <div class="pagination">
        <button class="dropdown-btn">
             V
        </button>
    </div>
  </div>
{/if}

<style>
  button {
    all: unset;
  }

  /* 整体容器样式 */
  .candidate-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0px 0px;
    background: #36383e;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    font-family: -apple-system, BlinkMacSystemFont, "PingFang SC", "Microsoft YaHei", sans-serif;
  }

  /* 候选词列表横向排列 */
  .candidate-list {
    display: flex;
    flex-wrap: nowrap;
  }

  /* 候选词项样式 */
  .candidate-item {
    display: flex;
    align-items: center;
    padding: 4px 10px 4px 0px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  /* 序号样式 - 蓝色背景白色文字 */
  .index {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 24px;
    color: #737373;
    font-size: 8px;
    font-weight: 500;
    border-radius: 4px;
    padding: 0px 5px 0px 5px;
  }

  .index.selected-index {
    color: white;
  }

  /* 候选词文字样式 */
  .text {
    color: white;
    font-size: 16px;
    white-space: nowrap;
  }

  /* 选中状态样式 */
  .candidate-item.selected {
    background: #007aff; /* 蓝色背景 */
  }

  .candidate-item.selected .index {
    background: #007aff; /* 蓝色背景 */
    color: white;
  }
  
  /* 分页控件样式 */
  .pagination {
    display: flex;
    align-items: center;
    margin-left: 10px;
  }

  /* 下拉按钮样式 */
  .dropdown-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: background 0.2s;
  }
  
  .dropdown-btn:hover {
    background: rgba(0, 0, 0, 0.05);
  }
  
  .dropdown-btn svg {
    transition: transform 0.2s;
  }
  
  .dropdown-btn:hover svg {
    transform: translateY(2px);
  }
</style>