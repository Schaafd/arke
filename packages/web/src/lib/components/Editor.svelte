<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { EditorView } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { markdown } from '@codemirror/lang-markdown';
  import { basicSetup } from 'codemirror';

  export let content = '';
  export let onChange: ((value: string) => void) | undefined = undefined;

  let editorElement: HTMLDivElement;
  let view: EditorView | null = null;

  onMount(() => {
    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged && onChange) {
        const newContent = update.state.doc.toString();
        onChange(newContent);
      }
    });

    const startState = EditorState.create({
      doc: content,
      extensions: [
        basicSetup,
        markdown(),
        updateListener,
        EditorView.theme({
          '&': {
            height: '100%',
            fontSize: '14px',
          },
          '.cm-scroller': {
            fontFamily: 'Menlo, Monaco, "Courier New", monospace',
          },
          '.cm-content': {
            padding: '16px 8px',
          },
        }),
      ],
    });

    view = new EditorView({
      state: startState,
      parent: editorElement,
    });
  });

  onDestroy(() => {
    view?.destroy();
  });

  // Update editor content when prop changes
  $: if (view && content !== view.state.doc.toString()) {
    view.dispatch({
      changes: {
        from: 0,
        to: view.state.doc.length,
        insert: content,
      },
    });
  }
</script>

<div class="editor-container">
  <div bind:this={editorElement} class="editor"></div>
</div>

<style>
  .editor-container {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  .editor {
    height: 100%;
    width: 100%;
  }

  :global(.cm-editor) {
    height: 100%;
  }

  :global(.cm-scroller) {
    overflow: auto;
  }
</style>
