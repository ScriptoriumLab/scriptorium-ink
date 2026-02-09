import { writable } from 'svelte/store';

export interface RenderState {
    visible: boolean;
    candidates: string[];
    highlight_index: number;
    page_index: number;
    total_pages: number;
}

const initialState: RenderState = {
    visible: true,
    candidates: ["测试1", "测试2", "测试3"],
    highlight_index: 0,
    page_index: 0,
    total_pages: 1
};

export const uiState = writable<RenderState>(initialState);