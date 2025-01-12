<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte';

  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  let progress = 0;
  let currentPack = 'Инициализация...';

  onMount(() => {
    listen('progress', (event: { payload: string }) => {
      const [packName, percentString] = event.payload.split(' (');
      currentPack = packName;
      progress = parseInt(percentString);
    });
  });

  import { goto } from '$app/navigation';

  function goToMainPage() {
    history.pushState(null, '', location.href);
    window.onpopstate = () => {
      history.pushState(null, '', location.href);
    };
    goto('/main');
  }
</script>

<main>
  <TopBar main_menu={false}/>

  <div class="loading-screen">
    <h1>Загрузка модпаков</h1>
    <progress class="progress-bar" value={progress} max="100"></progress>
    <p>{currentPack}</p>
    <button on:click={goToMainPage}>Перейти на главную страницу</button>
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
