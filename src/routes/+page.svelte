<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte';

  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';

  let progress = 0;
  let currentPack = 'Инициализация...';

  function goToMainPage() {
    history.pushState(null, '', location.href);
    window.onpopstate = () => {
      history.pushState(null, '', location.href);
    };
    goto('/main');
  }

  onMount(async () => {
    listen('progress', (event: { payload: string }) => {
      const [packName, percentString] = event.payload.split(' (');
      currentPack = packName;
      progress = parseInt(percentString);
    });

    const unlisten: UnlistenFn = await listen('progress', (event: { payload: { progress: number, packName: string } }) => {
      progress = event.payload.progress;
      currentPack = event.payload.packName;
    });

    invoke('modpacks_load').then(() => {
      unlisten();
      window.location.href = '/main'; // Замените '/' на маршрут основного экрана, если он другой.
    });
  });
</script>

<main>
  <TopBar main_menu={false}/>

  <div class="loading-screen">
    <h1>Загрузка модпаков</h1>
    <progress class="progress-bar" value={progress} max="100"></progress>
    <p>{currentPack}</p>
  </div>  
</main>

<style>
main {
  width: 100vw;
  height: 100vh;

  display: flex;
  flex-direction: column;
}

.loading-screen {
  flex-grow: 1;
  background-color: var(--black-2);

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.progress-bar {
    width: 80%;
    height: 20px;
    background-color: var(--gray);
    border-radius: 10px;
    overflow: hidden;
    margin: 20px 0;
}

.progress {
    height: 100%;
    background-color: var(--green);
    transition: width 0.3s ease;
}
</style>
