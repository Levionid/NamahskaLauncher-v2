<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte';

  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import loadingLogo from '$lib/images/loading.gif';

  let progress = 0;
  let currentPack = 'Инициализация...';
  const loadingMessages = [
    'Подготовка данных...',
    'Загрузка компонентов...',
    'Оптимизация параметров...',
    'Финальные шаги...'
  ];
  let messageIndex = 0;

  onMount(async () => {
    // Слушаем событие прогресса
    // const unlisten: UnlistenFn = await listen('progress', (event: { payload: { progress: number; packName: string } }) => {
    //   progress = event.payload.progress; // Обновляем прогресс
    //   currentPack = event.payload.packName; // Обновляем текущий пакет
    //   // Меняем сообщение каждые 25% прогресса
    //   messageIndex = Math.min(Math.floor(progress / 25), loadingMessages.length - 1);
    // });
// 
    // // Вызываем функцию загрузки модпаков
    // invoke('modpacks_load').then(() => {
    //   // После завершения снимаем слушатель и переходим на главную страницу
    //   unlisten();
    //   goto('/main');
    // });

    goto('/main');
  });
</script>

<main>
  <TopBar main_menu={false} userName="" />

  <div class="loading-info">
    <img src={loadingLogo} alt="Loading Logo" class="loading-logo" />
    <div class="progress-bar">
        <div class="progress" style="width: {progress}%;"></div>
    </div>
    <span class="current-pack">{currentPack}</span>
  </div>
  <div class="additional-info">
      <span>{loadingMessages[messageIndex]}</span>
      <span>Не выключайте приложение до завершения загрузки.</span>
  </div>
</main>

<style>
main {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--black);
}

.loading-info {
  width: 100vw;
  flex-grow: 1;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.loading-logo {
  width: 200px;
  height: 200px;
  margin-bottom: 32px;
  animation: fadeIn 2s ease-in-out;
}

.current-pack {
  font-size: 1.2rem;
  color: var(--gray-light);
  animation: pulse 2s infinite, fadeIn 2s ease-in-out;
}

.progress-bar {
  width: 60%;
  height: 15px;
  background: var(--gray);
  border-radius: 7.5px;
  overflow: hidden;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  margin-bottom: 20px;
  animation: fadeIn 2s ease-in-out;
}

.progress {
  height: 100%;
  background: linear-gradient(90deg, var(--green) 45%, #fff, var(--green) 75%);
  width: 0%;
  border-radius: 7.5px;
  background-size: 400%;
  animation: gradient 2s ease-in-out infinite;
  transition: width 0.3s ease-in-out;
}

.additional-info {
  width: 100%;

  display: flex;
  padding-bottom: 24px;
  gap: 12px;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  color: var(--gray-light);
  animation: fadeIn 2s ease-in-out;
}

@keyframes fadeIn {
  from {
      opacity: 0;
  }
  to {
      opacity: 1;
  }
}

@keyframes pulse {
  0%, 100% {
      opacity: 1;
  }
  50% {
      opacity: 0.6;
  }
}

@keyframes gradient {
  0% {
      background-position: 100%;
  }
  100% {
      background-position: 0%;
  }
}
</style>