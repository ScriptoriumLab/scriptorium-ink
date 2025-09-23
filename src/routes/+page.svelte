<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";

  // 定义与 Rust 同步的候选词更新类型
  type CandidateUpdate = {
    candidates: string[];
  };

  let candidates: string[] = [];
  let pageIndex = 0;
  const pageSize = 4;
  let selectedIndex: number = 0;
  let chan: Channel<CandidateUpdate> | null = null;

  function handleKeyDown(e: KeyboardEvent) {
    const currentPageCount = Math.min(
      pageSize, 
      candidates.length - pageIndex * pageSize
    );

    switch(e.key) {
      case "ArrowLeft":
        if (pageIndex * pageSize + selectedIndex > 0) {
          if (selectedIndex > 0) {
            --selectedIndex;
          } else {
            --pageIndex;
            selectedIndex = pageSize - 1;
          }
        }
        break;
      case "ArrowRight":
        if (pageIndex * pageSize + selectedIndex < candidates.length - 1) {
          if (selectedIndex < currentPageCount - 1) {
            ++selectedIndex;
          } else {
            ++pageIndex;
            selectedIndex = 0;
          }
        }
        break;
      case " ":
        selectCandidate(pageIndex * pageSize + selectedIndex);
        break;
      // 数字键直接选择
      case "1": case "2": case "3": case "4": case "5":
      case "6": case "7": case "8": case "9":
        const index = parseInt(e.key) - 1;
        if (index < currentPageCount) selectCandidate(pageIndex * pageSize + index);
        break;
    }
  }

  // 选择候选词后通知 Rust 后端
  function selectCandidate(index: number) {
    invoke("select_candidate", { index });
  }

  onMount(async () => {
    window.addEventListener("keydown", handleKeyDown);

    // 创建并注册 Channel，只需调用一次
    chan = new Channel<CandidateUpdate>();
    chan.onmessage = (msg) => {
      candidates = msg.candidates;
      pageIndex = 0;
      selectedIndex = 0;
    };
    await invoke("register_candidate_channel", { channel: chan });

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  });

  // Svelte 专用的组件卸载钩子，确保关闭 Channel
  onDestroy(() => {
    if (chan) chan.close();
  });
</script>

{#if candidates.length > 0}
<div class="candidate-bar">
  <div class="candidate-list">
    {#each candidates.slice(pageIndex * pageSize, (pageIndex + 1) * pageSize) as candidate, i}
      <div 
        class="candidate-item"
        class:selected={selectedIndex === i}
        on:click={() => selectCandidate(pageIndex * pageSize + i)}
      >
        <!-- 序号样式改为蓝色背景白色文字 -->
        <span class="index">{i + 1}</span>
        <span class="text">{candidate}</span>
      </div>
    {/each}
  </div>
  
  <!-- 修改分页控件为下拉箭头 -->
  <div class="pagination">
    <!-- <button class="dropdown-btn" on:click={() => pageIndex++}> -->
    <button class="dropdown-btn">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M6 9L12 15L18 9" stroke="#666" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>
</div>
{/if}

<style>
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