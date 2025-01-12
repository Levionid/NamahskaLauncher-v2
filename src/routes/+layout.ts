// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

// import { invoke } from '@tauri-apps/api/tauri';
// 
// export const load = async () => {
//     const isLoading = await invoke('is_loading'); // Предполагается, что есть команда для проверки загрузки
//     if (isLoading) {
//         return { redirect: '/loading' };
//     }
// };
  