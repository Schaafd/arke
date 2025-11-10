<script lang="ts">
  import { vaultStore, type VaultFile } from '$lib/stores/vault';

  export let onFileSelect: (file: VaultFile) => void;

  $: files = $vaultStore.files;
  $: currentFile = $vaultStore.currentFile;

  function handleFileClick(file: VaultFile) {
    onFileSelect(file);
  }

  function getFileDisplayName(path: string): string {
    return path.split('/').pop() || path;
  }

  function getFileDirectory(path: string): string {
    const parts = path.split('/');
    return parts.length > 1 ? parts.slice(0, -1).join('/') : '';
  }

  // Group files by directory
  $: groupedFiles = files.reduce(
    (acc, file) => {
      const dir = getFileDirectory(file.path) || 'Root';
      if (!acc[dir]) {
        acc[dir] = [];
      }
      acc[dir].push(file);
      return acc;
    },
    {} as Record<string, VaultFile[]>
  );
</script>

<div class="file-tree">
  {#if files.length === 0}
    <div class="empty-state">
      <p>No files in vault</p>
      <p class="hint">Open a directory to get started</p>
    </div>
  {:else}
    {#each Object.entries(groupedFiles) as [directory, dirFiles]}
      <div class="directory-group">
        {#if directory !== 'Root'}
          <div class="directory-label">{directory}</div>
        {/if}
        {#each dirFiles as file}
          <button
            class="file-item"
            class:active={currentFile?.path === file.path}
            on:click={() => handleFileClick(file)}
          >
            <span class="file-icon">📄</span>
            <span class="file-name">{getFileDisplayName(file.path)}</span>
          </button>
        {/each}
      </div>
    {/each}
  {/if}
</div>

<style>
  .file-tree {
    height: 100%;
    overflow-y: auto;
    padding: 8px;
    background: #f9f9f9;
    border-right: 1px solid #e0e0e0;
  }

  .empty-state {
    padding: 24px 16px;
    text-align: center;
    color: #666;
  }

  .empty-state p {
    margin: 8px 0;
  }

  .hint {
    font-size: 0.875rem;
    color: #999;
  }

  .directory-group {
    margin-bottom: 16px;
  }

  .directory-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    padding: 4px 8px;
    margin-bottom: 4px;
  }

  .file-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.15s;
    text-align: left;
  }

  .file-item:hover {
    background: #e8e8e8;
  }

  .file-item.active {
    background: #4a90e2;
    color: white;
  }

  .file-icon {
    margin-right: 8px;
    font-size: 14px;
  }

  .file-name {
    flex: 1;
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
