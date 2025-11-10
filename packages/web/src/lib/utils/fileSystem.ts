/**
 * File System Access API utilities
 * Provides methods to open, save, and manage files using the browser's File System Access API
 */

export interface FileHandle {
  handle: FileSystemFileHandle;
  name: string;
  content: string;
}

/**
 * Check if File System Access API is supported
 */
export function isFileSystemAccessSupported(): boolean {
  return 'showOpenFilePicker' in window && 'showSaveFilePicker' in window;
}

/**
 * Open a file picker and read the selected file
 */
export async function openFile(): Promise<FileHandle | null> {
  if (!isFileSystemAccessSupported()) {
    throw new Error('File System Access API is not supported in this browser');
  }

  try {
    const [fileHandle] = await window.showOpenFilePicker({
      types: [
        {
          description: 'Markdown Files',
          accept: {
            'text/markdown': ['.md', '.markdown'],
          },
        },
      ],
      multiple: false,
    });

    const file = await fileHandle.getFile();
    const content = await file.text();

    return {
      handle: fileHandle,
      name: file.name,
      content,
    };
  } catch (err) {
    // User cancelled the picker
    if ((err as Error).name === 'AbortError') {
      return null;
    }
    throw err;
  }
}

/**
 * Save content to an existing file handle
 */
export async function saveFile(fileHandle: FileSystemFileHandle, content: string): Promise<void> {
  const writable = await fileHandle.createWritable();
  await writable.write(content);
  await writable.close();
}

/**
 * Save content to a new file (shows save picker)
 */
export async function saveFileAs(content: string): Promise<FileHandle | null> {
  if (!isFileSystemAccessSupported()) {
    throw new Error('File System Access API is not supported in this browser');
  }

  try {
    const fileHandle = await window.showSaveFilePicker({
      types: [
        {
          description: 'Markdown Files',
          accept: {
            'text/markdown': ['.md'],
          },
        },
      ],
      suggestedName: 'untitled.md',
    });

    await saveFile(fileHandle, content);

    return {
      handle: fileHandle,
      name: fileHandle.name,
      content,
    };
  } catch (err) {
    // User cancelled the picker
    if ((err as Error).name === 'AbortError') {
      return null;
    }
    throw err;
  }
}

/**
 * Open a directory picker and get access to the directory
 */
export async function openDirectory(): Promise<FileSystemDirectoryHandle | null> {
  if (!('showDirectoryPicker' in window)) {
    throw new Error('Directory picker is not supported in this browser');
  }

  try {
    const dirHandle = await window.showDirectoryPicker();
    return dirHandle;
  } catch (err) {
    // User cancelled the picker
    if ((err as Error).name === 'AbortError') {
      return null;
    }
    throw err;
  }
}

/**
 * List all markdown files in a directory recursively
 */
export async function listMarkdownFiles(
  dirHandle: FileSystemDirectoryHandle,
  path = ''
): Promise<Array<{ path: string; handle: FileSystemFileHandle }>> {
  const files: Array<{ path: string; handle: FileSystemFileHandle }> = [];

  for await (const entry of dirHandle.values()) {
    const entryPath = path ? `${path}/${entry.name}` : entry.name;

    if (entry.kind === 'file' && entry.name.endsWith('.md')) {
      files.push({
        path: entryPath,
        handle: entry as FileSystemFileHandle,
      });
    } else if (entry.kind === 'directory') {
      // Skip hidden directories and node_modules
      if (!entry.name.startsWith('.') && entry.name !== 'node_modules') {
        const subFiles = await listMarkdownFiles(entry as FileSystemDirectoryHandle, entryPath);
        files.push(...subFiles);
      }
    }
  }

  return files.sort((a, b) => a.path.localeCompare(b.path));
}
