<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface StatisticsPeriod {
    period_type: string;
    period_start: string;
    total_stash_count: number;
    total_item_count: number;
    total_bytes: number;
  }

  let name = $state("What!");
  let greetMsg = $state("");
  let statistics = $state<StatisticsPeriod[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function loadStatistics() {
    loading = true;
    error = null;
    try {
      const result = await invoke<StatisticsPeriod[]>(
        "get_statistics_per_periods"
      );
      statistics = result;
    } catch (e) {
      error = e as string;
      console.error("Failed to load statistics:", e);
    } finally {
      loading = false;
    }
  }

  function formatBytes(bytes: number): string {
    const units = ["B", "KB", "MB", "GB", "TB"];
    let size = bytes;
    let unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return `${size.toFixed(2)} ${units[unitIndex]}`;
  }

  onMount(() => {
    loadStatistics();
  });
</script>

<main class="container">
  <h1>Statistics Per Periods</h1>

  <div class="controls">
    <button onclick={loadStatistics} disabled={loading}>
      {loading ? "Loading..." : "Refresh Data"}
    </button>
  </div>

  {#if error}
    <div class="error">
      <p>Error: {error}</p>
    </div>
  {/if}

  {#if loading}
    <div class="loading">
      <p>Loading statistics...</p>
    </div>
  {:else if statistics.length > 0}
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Period Type</th>
            <th>Period Start</th>
            <th>Total Stash Count</th>
            <th>Total Item Count</th>
            <th>Total Bytes</th>
          </tr>
        </thead>
        <tbody>
          {#each statistics as stat}
            <tr>
              <td>{stat.period_type}</td>
              <td>{stat.period_start}</td>
              <td>{stat.total_stash_count.toLocaleString()}</td>
              <td>{stat.total_item_count.toLocaleString()}</td>
              <td>{formatBytes(stat.total_bytes)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="empty">
      <p>No statistics data found.</p>
    </div>
  {/if}

  <div class="greeting-section">
    <p>Hello {greetMsg}</p>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  h1 {
    text-align: center;
    margin-bottom: 2rem;
    color: #333;
  }

  .controls {
    text-align: center;
    margin-bottom: 1rem;
  }

  button {
    background-color: #007acc;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  button:hover:not(:disabled) {
    background-color: #005999;
  }

  button:disabled {
    background-color: #ccc;
    cursor: not-allowed;
  }

  .error {
    background-color: #fee;
    border: 1px solid #fcc;
    border-radius: 4px;
    padding: 1rem;
    margin: 1rem 0;
    color: #c00;
  }

  .loading,
  .empty {
    text-align: center;
    padding: 2rem;
    color: #666;
  }

  .table-container {
    overflow-x: auto;
    margin: 1rem 0;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background-color: white;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  th,
  td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }

  th {
    background-color: #f8f9fa;
    font-weight: 600;
    color: #333;
  }

  tr:nth-child(even) {
    background-color: #f8f9fa;
  }

  tr:hover {
    background-color: #e3f2fd;
  }

  td:nth-child(3),
  td:nth-child(4),
  td:nth-child(5) {
    text-align: right;
  }

  th:nth-child(3),
  th:nth-child(4),
  th:nth-child(5) {
    text-align: right;
  }

  .greeting-section {
    margin-top: 3rem;
    text-align: center;
    padding: 1rem;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    h1 {
      color: #f6f6f6;
    }

    table {
      background-color: #3a3a3a;
    }

    th {
      background-color: #4a4a4a;
      color: #f6f6f6;
    }

    tr:nth-child(even) {
      background-color: #4a4a4a;
    }

    tr:hover {
      background-color: #5a5a5a;
    }

    .greeting-section {
      background-color: #3a3a3a;
    }

    .error {
      background-color: #4a2a2a;
      border-color: #6a3a3a;
      color: #ff6666;
    }
  }
</style>
