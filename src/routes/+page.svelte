<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte';

  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';

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
    const unlisten: UnlistenFn = await listen('progress', (event: { payload: { progress: number; packName: string } }) => {
      progress = event.payload.progress; // Обновляем прогресс
      currentPack = event.payload.packName; // Обновляем текущий пакет
      // Меняем сообщение каждые 25% прогресса
      messageIndex = Math.min(Math.floor(progress / 25), loadingMessages.length - 1);
    });

    // Вызываем функцию загрузки модпаков
    invoke('modpacks_load').then(() => {
      // После завершения снимаем слушатель и переходим на главную страницу
      unlisten();
      goto('/main'); // Убедитесь, что маршрут правильный
    });
  });
</script>

<main>
  <TopBar main_menu={false} />

  <div class="loading-screen">
    <h1 class="loading-title">Загрузка модпаков</h1>
    <div class="progress-bar">
      <div class="progress" style="width: {progress}%;"></div>
    </div>
    <p class="current-pack">{currentPack}</p>
    <div class="additional-info">
      <p>{loadingMessages[messageIndex]}</p>
      <p>Не выключайте приложение до завершения загрузки.</p>
    </div>
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

.loading-screen {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 20px;
  position: relative;
}

.loading-title {
  font-size: 2.5rem;
  margin-bottom: 20px;
  color: #fff;
  animation: fadeIn 1.5s ease-in-out;
}

.current-pack {
  margin-top: 15px;
  font-size: 1.2rem;
  color: var(--gray-light);
  animation: pulse 2s infinite;
}

.progress-bar {
  width: 60%;
  height: 15px;
  background: var(--gray);
  border-radius: 7.5px;
  overflow: hidden;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  margin-bottom: 20px;
}

.progress {
  height: 100%;
  background: var(--green);
  width: 0%;
  border-radius: 7.5px;
  transition: width 0.3s ease-in-out;
}

.additional-info {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 1rem;
  color: var(--gray-light);
  line-height: 1.5;
  max-width: 80%;
  text-align: center;
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
</style>