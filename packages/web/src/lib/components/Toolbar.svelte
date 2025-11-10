<script lang="ts">
  export let onOpenDirectory: () => void;
  export let onOpenFile: () => void;
  export let onSaveFile: () => void;
  export let onNewFile: () => void;
  export let isDirty = false;
  export let currentFileName: string | null = null;
</script>

<div class="toolbar">
  <div class="toolbar-section">
    <button class="toolbar-button" on:click={onOpenDirectory} title="Open Vault">
      📁 Open Vault
    </button>
    <button class="toolbar-button" on:click={onOpenFile} title="Open File"> 📄 Open File </button>
    <button class="toolbar-button" on:click={onNewFile} title="New File"> ➕ New </button>
    <button class="toolbar-button" on:click={onSaveFile} title="Save" disabled={!isDirty}>
      💾 Save
    </button>
  </div>

  <div class="toolbar-section center">
    {#if currentFileName}
      <span class="file-name">
        {currentFileName}
        {#if isDirty}
          <span class="dirty-indicator">●</span>
        {/if}
      </span>
    {:else}
      <span class="placeholder">Arke Editor</span>
    {/if}
  </div>

  <div class="toolbar-section">
    <span class="app-version">v0.1.0</span>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: #fff;
    border-bottom: 1px solid #e0e0e0;
    height: 48px;
  }

  .toolbar-section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toolbar-section.center {
    flex: 1;
    justify-content: center;
  }

  .toolbar-button {
    padding: 6px 12px;
    border: 1px solid #ddd;
    background: white;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.15s;
  }

  .toolbar-button:hover:not(:disabled) {
    background: #f5f5f5;
    border-color: #4a90e2;
  }

  .toolbar-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .file-name {
    font-weight: 500;
    font-size: 0.875rem;
    color: #333;
  }

  .dirty-indicator {
    color: #ff9800;
    margin-left: 4px;
  }

  .placeholder {
    color: #999;
    font-size: 0.875rem;
  }

  .app-version {
    font-size: 0.75rem;
    color: #999;
  }
</style>
