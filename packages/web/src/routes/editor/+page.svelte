<script lang="ts">
  import { onMount } from 'svelte';
  import Editor from '$lib/components/Editor.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import FileTree from '$lib/components/FileTree.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import { editorStore } from '$lib/stores/editor';
  import { vaultStore, type VaultFile } from '$lib/stores/vault';
  import {
    openFile,
    openDirectory,
    saveFile,
    saveFileAs,
    listMarkdownFiles,
    isFileSystemAccessSupported,
  } from '$lib/utils/fileSystem';

  let showPreview = true;
  let currentFileHandle: FileSystemFileHandle | null = null;
  let apiSupported = false;

  onMount(() => {
    apiSupported = isFileSystemAccessSupported();
  });

  async function handleOpenDirectory() {
    try {
      vaultStore.setLoading(true);
      const dirHandle = await openDirectory();

      if (dirHandle) {
        const files = await listMarkdownFiles(dirHandle);
        vaultStore.setDirectory(
          dirHandle,
          files.map((f) => ({ path: f.path, handle: f.handle }))
        );
      }
    } catch (err) {
      console.error('Failed to open directory:', err);
      alert('Failed to open directory: ' + (err as Error).message);
    } finally {
      vaultStore.setLoading(false);
    }
  }

  async function handleOpenFile() {
    try {
      const fileData = await openFile();

      if (fileData) {
        currentFileHandle = fileData.handle;
        editorStore.setContent(fileData.content);
        editorStore.setFilePath(fileData.name);
        editorStore.markSaved();
      }
    } catch (err) {
      console.error('Failed to open file:', err);
      alert('Failed to open file: ' + (err as Error).message);
    }
  }

  async function handleSaveFile() {
    try {
      const content = $editorStore.content;

      if (currentFileHandle) {
        await saveFile(currentFileHandle, content);
        editorStore.markSaved();
      } else {
        const fileData = await saveFileAs(content);
        if (fileData) {
          currentFileHandle = fileData.handle;
          editorStore.setFilePath(fileData.name);
          editorStore.markSaved();
        }
      }
    } catch (err) {
      console.error('Failed to save file:', err);
      alert('Failed to save file: ' + (err as Error).message);
    }
  }

  function handleNewFile() {
    currentFileHandle = null;
    editorStore.reset();
  }

  async function handleFileSelect(file: VaultFile) {
    try {
      const fileObj = await file.handle.getFile();
      const content = await fileObj.text();

      currentFileHandle = file.handle;
      vaultStore.setCurrentFile(file);
      editorStore.setContent(content);
      editorStore.setFilePath(file.path);
      editorStore.markSaved();
    } catch (err) {
      console.error('Failed to read file:', err);
      alert('Failed to read file: ' + (err as Error).message);
    }
  }

  function handleContentChange(newContent: string) {
    editorStore.setContent(newContent);
  }

  // Auto-save on Cmd/Ctrl+S
  function handleKeydown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key === 's') {
      event.preventDefault();
      if ($editorStore.isDirty) {
        handleSaveFile();
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="editor-page">
  {#if !apiSupported}
    <div class="unsupported-banner">
      <p>⚠️ File System Access API is not supported in this browser.</p>
      <p>Please use a Chromium-based browser (Chrome, Edge, Brave) for the best experience.</p>
    </div>
  {/if}

  <Toolbar
    onOpenDirectory={handleOpenDirectory}
    onOpenFile={handleOpenFile}
    onSaveFile={handleSaveFile}
    onNewFile={handleNewFile}
    isDirty={$editorStore.isDirty}
    currentFileName={$editorStore.filePath}
  />

  <div class="editor-layout">
    <aside class="sidebar">
      <FileTree onFileSelect={handleFileSelect} />
    </aside>

    <main class="main-content">
      <div class="editor-container">
        <Editor content={$editorStore.content} onChange={handleContentChange} />
      </div>

      {#if showPreview}
        <div class="preview-container">
          <Preview content={$editorStore.content} />
        </div>
      {/if}
    </main>
  </div>

  <div class="status-bar">
    <span class="status-item">
      {$editorStore.content.length} characters
    </span>
    <span class="status-item">
      {$editorStore.content.split('\n').length} lines
    </span>
    <button class="status-button" on:click={() => (showPreview = !showPreview)}>
      {showPreview ? '👁️ Hide Preview' : '👁️ Show Preview'}
    </button>
  </div>
</div>

<style>
  .editor-page {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .unsupported-banner {
    background: #fff3cd;
    color: #856404;
    padding: 12px 16px;
    text-align: center;
    border-bottom: 1px solid #ffeaa7;
  }

  .unsupported-banner p {
    margin: 4px 0;
  }

  .editor-layout {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 250px;
    min-width: 200px;
    max-width: 400px;
    overflow-y: auto;
  }

  .main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .editor-container {
    flex: 1;
    overflow: hidden;
    border-right: 1px solid #e0e0e0;
  }

  .preview-container {
    flex: 1;
    overflow: hidden;
  }

  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 16px;
    background: #f5f5f5;
    border-top: 1px solid #e0e0e0;
    height: 28px;
    font-size: 0.75rem;
  }

  .status-item {
    color: #666;
    margin-right: 16px;
  }

  .status-button {
    padding: 2px 8px;
    border: none;
    background: transparent;
    cursor: pointer;
    font-size: 0.75rem;
    color: #666;
  }

  .status-button:hover {
    color: #333;
  }
</style>
