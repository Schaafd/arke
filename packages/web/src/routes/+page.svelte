<script lang="ts">
  let version = '0.1.0';
</script>

<div class="container">
  <h1>Welcome to Arke</h1>
  <p>A cross-platform, markdown-first personal knowledge management system</p>
  <p class="version">Version: {version}</p>

  <div class="cta">
    <a href="/editor" class="button-primary">Open Editor</a>
  </div>

  <div class="features">
    <h2>Features</h2>
    <ul>
      <li>Fast, local-first markdown editing</li>
      <li>Wikilinks and backlinks</li>
      <li>Full-text search</li>
      <li>Tags and daily notes</li>
      <li>Plugin system</li>
      <li>Cross-platform (Web, macOS, iOS)</li>
    </ul>
  </div>

  <div class="status">
    <h3>Development Status</h3>
    <p>Phase 0: Editor Prototype ✨</p>
    <ul>
      <li>✅ CodeMirror 6 integration</li>
      <li>✅ File System Access API</li>
      <li>✅ Split view editor + preview</li>
      <li>✅ File tree navigation</li>
    </ul>
  </div>
</div>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    font-family:
      system-ui,
      -apple-system,
      sans-serif;
  }

  h1 {
    color: #333;
    margin-bottom: 0.5rem;
  }

  .version {
    color: #666;
    font-size: 0.9rem;
  }

  .features {
    margin-top: 2rem;
  }

  .features ul {
    list-style: none;
    padding: 0;
  }

  .features li {
    padding: 0.5rem 0;
    padding-left: 1.5rem;
    position: relative;
  }

  .features li::before {
    content: '✓';
    position: absolute;
    left: 0;
    color: #4caf50;
  }

  .status {
    margin-top: 2rem;
    padding: 1rem;
    background: #f5f5f5;
    border-radius: 4px;
  }

  .status h3 {
    margin-top: 0;
  }

  .status ul {
    list-style: none;
    padding: 0;
    margin-top: 1rem;
  }

  .status li {
    padding: 0.25rem 0;
  }

  .cta {
    margin: 2rem 0;
  }

  .button-primary {
    display: inline-block;
    padding: 12px 32px;
    background: #4a90e2;
    color: white;
    text-decoration: none;
    border-radius: 6px;
    font-weight: 500;
    transition: background 0.2s;
  }

  .button-primary:hover {
    background: #357abd;
    text-decoration: none;
  }
</style>
