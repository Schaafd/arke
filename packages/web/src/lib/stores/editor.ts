import { writable } from 'svelte/store';

export interface EditorState {
  content: string;
  filePath: string | null;
  isDirty: boolean;
  lastSaved: Date | null;
}

function createEditorStore() {
  const { subscribe, set, update } = writable<EditorState>({
    content: '',
    filePath: null,
    isDirty: false,
    lastSaved: null,
  });

  return {
    subscribe,
    setContent: (content: string) =>
      update((state) => ({
        ...state,
        content,
        isDirty: true,
      })),
    setFilePath: (filePath: string | null) =>
      update((state) => ({
        ...state,
        filePath,
      })),
    markSaved: () =>
      update((state) => ({
        ...state,
        isDirty: false,
        lastSaved: new Date(),
      })),
    reset: () =>
      set({
        content: '',
        filePath: null,
        isDirty: false,
        lastSaved: null,
      }),
  };
}

export const editorStore = createEditorStore();
