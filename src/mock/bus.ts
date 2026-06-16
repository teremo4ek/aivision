/**
 * In-memory event bus that mimics @tauri-apps/api/event.
 *
 * The copied Handy UI talks to a Tauri backend through `listen`/`emit`. Since
 * aivision has no backend, this bus lets the mock command implementations
 * (src/mock/core.ts) drive the exact same UI flows the real backend would —
 * e.g. model download progress events and the recording overlay's mic levels.
 */
export type MockEvent<T> = { event: string; id: number; payload: T };
type Handler<T = unknown> = (event: MockEvent<T>) => void;

const listeners = new Map<string, Set<Handler>>();
let nextId = 0;

export function listen<T = unknown>(
  event: string,
  handler: Handler<T>,
): Promise<() => void> {
  let set = listeners.get(event);
  if (!set) {
    set = new Set();
    listeners.set(event, set);
  }
  set.add(handler as Handler);
  return Promise.resolve(() => {
    const s = listeners.get(event);
    if (s) s.delete(handler as Handler);
  });
}

export function once<T = unknown>(
  event: string,
  handler: Handler<T>,
): Promise<() => void> {
  const wrapped: Handler<T> = (e) => {
    handler(e);
    const s = listeners.get(event);
    if (s) s.delete(wrapped as Handler);
  };
  return listen(event, wrapped);
}

export function emit<T = unknown>(event: string, payload?: T): Promise<void> {
  const set = listeners.get(event);
  if (set) {
    const e: MockEvent<T> = { event, id: nextId++, payload: payload as T };
    for (const h of [...set]) {
      try {
        h(e);
      } catch (err) {
        // eslint-disable-next-line no-console
        console.error(`[mock-bus] handler error for "${event}":`, err);
      }
    }
  }
  return Promise.resolve();
}
