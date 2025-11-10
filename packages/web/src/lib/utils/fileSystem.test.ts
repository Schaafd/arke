import { describe, it, expect, beforeAll } from 'vitest';
import { isFileSystemAccessSupported } from './fileSystem';

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
});
