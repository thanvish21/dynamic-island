import { writable } from 'svelte/store';

export type WidgetType = 'media' | 'system' | 'weather' | 'notifications' | 'timer' | 'clipboard';
export type IslandState = 'compact' | 'expanded' | 'wide';

export const activeWidget = writable<WidgetType>('system');
export const islandState = writable<IslandState>('compact');
export const isHovered = writable(false);
