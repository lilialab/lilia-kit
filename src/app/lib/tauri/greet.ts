import { invoke } from '@tauri-apps/api/core';

/** 调用 Rust `greet` 命令，返回问候文案 */
export async function greet(name: string): Promise<string> {
  return invoke<string>('greet', { name });
}
