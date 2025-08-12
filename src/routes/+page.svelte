<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  // 候选词数据（示例）
  let candidates: string[] = ["候选词1", "候选词2", "候选词3", "候选词4"];
  let pageIndex = 0; // 当前页码
  const pageSize = 4; // 每页显示数

  // 从 Rust 后端获取候选词（IPC通信）
  async function fetchCandidates(input: string) {
    candidates = await invoke("get_candidates", { input });
  }

  // 处理键盘事件（翻页/选择）
  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "PageDown") pageIndex++;
    else if (e.key === "PageUp") pageIndex--;
    else if (e.key >= "1" && e.key <= "9") {
      const index = parseInt(e.key) - 1;
      if (index < candidates.length) selectCandidate(index);
    }
  }

  let selectedIndex = 0;
  // 选择候选词后通知 Rust 后端
  function selectCandidate(index: number) {
    selectedIndex = index;
    console.log("选择候选词:", candidates[index]);
    invoke("select_candidate", { index });
  }

  // 初始化键盘监听
  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  });
</script>

<div class="candidate-bar">
  <div class="candidate-list">
    {#each candidates.slice(pageIndex * pageSize, (pageIndex + 1) * pageSize) as candidate, i}
      <div 
        class="candidate-item"
        class:selected={selectedIndex === i}
        on:click={() => selectCandidate(i)}
      >
        <!-- 序号样式改为蓝色背景白色文字 -->
        <span class="index">{i + 1}</span>
        <span class="text">{candidate}</span>
      </div>
    {/each}
  </div>
  
  <!-- 修改分页控件为下拉箭头 -->
  <div class="pagination">
    <button class="dropdown-btn" on:click={() => pageIndex++}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M6 9L12 15L18 9" stroke="#666" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>
</div>

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