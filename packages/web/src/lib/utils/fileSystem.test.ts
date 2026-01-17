import { describe, it, expect, beforeAll } from 'vitest';
import { isFileSystemAccessSupported, listMarkdownFiles } from './fileSystem';

type MockEntry = {
  kind: 'file' | 'directory';
  name: string;
  values?: () => AsyncIterableIterator<MockEntry>;
};

const createFile = (name: string): MockEntry => ({ kind: 'file', name });

const createDirectory = (name: string, entries: MockEntry[]): MockEntry => ({
  kind: 'directory',
  name,
  values: async function* values() {
    for (const entry of entries) {
      yield entry;
    }
  },
});

describe('fileSystem utilities', () => {
  beforeAll(() => {
    // Mock window object for tests
    if (typeof global.window === 'undefined') {
      (global as any).window = {};
    }
  });

  it('should check for File System Access API support', () => {
    const supported = isFileSystemAccessSupported();
    // In test environment, this will be false
    expect(typeof supported).toBe('boolean');
    expect(supported).toBe(false); // No File System Access API in test env
  });

  it('should list markdown files recursively and skip hidden directories', async () => {
    const dirHandle = createDirectory('root', [
      createFile('note.md'),
      createFile('image.png'),
      createDirectory('nested', [createFile('nested-note.md'), createFile('readme.txt')]),
      createDirectory('.git', [createFile('ignored.md')]),
      createDirectory('node_modules', [createFile('ignored.md')]),
    ]);

    const files = await listMarkdownFiles(dirHandle as unknown as FileSystemDirectoryHandle);

    expect(files.map((file) => file.path)).toEqual(['nested/nested-note.md', 'note.md']);
  });
});
