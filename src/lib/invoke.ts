import { InvokeArgs, invoke as tauriInvoke } from "@tauri-apps/api/tauri";

type Primitive = string | number | boolean | null | undefined;

/**
 * Sends a message to the backend.
 * @example
 * ```typescript
 * import { invoke } from '@tauri-apps/api/primitives';
 * await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
 * ```
 *
 * @template T The type of the response.
 * @param cmd The command name.
 * @param args The optional arguments to pass to the command.
 * @param options The request options.
 * @return A promise resolving or rejecting to the backend response.
 *
 * @since 1.0.0
 */
export function invoke<T extends Primitive>(
  cmd: string,
  args?: InvokeArgs
): Promise<T>;

/**
 * Sends a message to the backend.
 * @example
 * ```typescript
 * import { invoke } from '@/lib/invoke';
 * type LoginParams = { user: string; password: string };
 * await invoke<LoginParams>('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
 * ```
 * @template T The type of the arguments.
 * @param cmd The command name.
 * @param args The optional arguments to pass to the command. (Now with type safety!)
 * @param options The request options.
 * @return A promise resolving or rejecting to the backend response.
 *
 * @since 1.0.0
 */
export function invoke<T>(
  cmd: string,
  args: { [K in keyof T]: T[K] }
): Promise<T>;

/**
 * Sends a message to the backend.
 * @example
 * ```typescript
 * import { invoke } from '@/lib/invoke';
 * type LoginParams = { user: string; password: string };
 * await invoke<LoginParams>('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
 * ```
 * @template T The type of the arguments.
 * @template E The type of the response.
 * @param cmd The command name.
 * @param args The optional arguments to pass to the command. (Now with type safety!)
 * @param options The request options.
 * @return A promise resolving or rejecting to the backend response.
 *
 * @since 1.0.0
 */
export function invoke<T, E>(
  cmd: string,
  args: { [K in keyof T]: T[K] }
): Promise<E>;

// Implementation
export function invoke<T>(
  cmd: string,
  args?: InvokeArgs | { [K in keyof T]: T[K] }
): Promise<T> {
  return tauriInvoke<T>(cmd, args);
}
