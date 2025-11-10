import { writable } from 'svelte/store';

export interface VaultFile {
  path: string;
  handle: FileSystemFileHandle;
}

export interface VaultState {
  directoryHandle: FileSystemDirectoryHandle | null;
  files: VaultFile[];
  currentFile: VaultFile | null;
  isLoading: boolean;
}

function createVaultStore() {
  const { subscribe, set, update } = writable<VaultState>({
    directoryHandle: null,
    files: [],
    currentFile: null,
    isLoading: false,
  });

  return {
    subscribe,
    setDirectory: (handle: FileSystemDirectoryHandle, files: VaultFile[]) =>
      update((state) => ({
        ...state,
        directoryHandle: handle,
        files,
      })),
    setCurrentFile: (file: VaultFile | null) =>
      update((state) => ({
        ...state,
        currentFile: file,
      })),
    setLoading: (isLoading: boolean) =>
      update((state) => ({
        ...state,
        isLoading,
      })),
    reset: () =>
      set({
        directoryHandle: null,
        files: [],
        currentFile: null,
        isLoading: false,
      }),
  };
}

export const vaultStore = createVaultStore();
