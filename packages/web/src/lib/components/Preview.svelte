<script lang="ts">
  export let content: string;

  // Simple markdown to HTML conversion for preview
  // In Phase 1, we'll integrate the Rust parser via WASM
  function renderMarkdown(markdown: string): string {
    let html = markdown
      // Headers
      .replace(/^### (.*$)/gim, '<h3>$1</h3>')
      .replace(/^## (.*$)/gim, '<h2>$1</h2>')
      .replace(/^# (.*$)/gim, '<h1>$1</h1>')
      // Bold
      .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
      // Italic
      .replace(/\*(.+?)\*/g, '<em>$1</em>')
      // Code inline
      .replace(/`(.+?)`/g, '<code>$1</code>')
      // Links
      .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>')
      // Wikilinks
      .replace(/\[\[([^\]]+)\]\]/g, '<a href="#" class="wikilink">$1</a>')
      // Line breaks
      .replace(/\n/g, '<br>');

    return html;
  }

  $: html = renderMarkdown(content);
</script>

<div class="preview">
  <div class="preview-content">
    {@html html}
  </div>
</div>

<style>
  .preview {
    height: 100%;
    width: 100%;
    overflow-y: auto;
    background: white;
    padding: 16px 24px;
  }

  .preview-content {
    max-width: 800px;
    margin: 0 auto;
    line-height: 1.6;
  }

  .preview-content :global(h1) {
    font-size: 2em;
    margin: 0.67em 0;
    font-weight: bold;
  }

  .preview-content :global(h2) {
    font-size: 1.5em;
    margin: 0.75em 0;
    font-weight: bold;
  }

  .preview-content :global(h3) {
    font-size: 1.17em;
    margin: 0.83em 0;
    font-weight: bold;
  }

  .preview-content :global(code) {
    background: #f4f4f4;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 0.9em;
  }

  .preview-content :global(a) {
    color: #4a90e2;
    text-decoration: none;
  }

  .preview-content :global(a:hover) {
    text-decoration: underline;
  }

  .preview-content :global(.wikilink) {
    color: #7b68ee;
    font-weight: 500;
  }

  .preview-content :global(strong) {
    font-weight: bold;
  }

  .preview-content :global(em) {
    font-style: italic;
  }
</style>
