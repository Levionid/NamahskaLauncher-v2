<script lang="ts">
  import closeSvg from '$lib/images/close.svg';
  import maximizeSvg from '$lib/images/maximize.svg';
  import minimizeSvg from '$lib/images/minimize.svg';
  import noTexture from '$lib/images/no-texture.png';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  const appWindow = getCurrentWindow();
  
  function minimize() {
    appWindow.minimize();
  }
  
  function toggleMaximize() {
    appWindow.toggleMaximize();
  }
  
  function close() {
    appWindow.close();
  }
</script>

<header data-tauri-drag-region>
  <div class="task-status">
    <img src={noTexture} alt="Task Status" />
    <span>Нет запущенных задач</span>
  </div>
  <div class="user-info">
    <img class="user-avatar" src={noTexture} alt="Аватар" />
    <span class="user-name">Levionid222</span>
  </div>
  <div class="window-buttons">
    <button class="window-minimize" on:click={minimize} title="Свернуть">
    <img src={minimizeSvg} alt="Minimize" />
    </button>
    <button class="window-maximize" on:click={toggleMaximize} title="Развернуть">
    <img src={maximizeSvg} alt="Maximize" />
    </button>
    <button class="window-close" on:click={close} title="Закрыть">
    <img src={closeSvg} alt="Close" />
    </button>
  </div>
</header>

<style>
header {
  width: 100%;
  height: 40px;
  background-color: var(--black);

  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-direction: row;
  gap: 32px;

  color: #d9d9d9;
  font-size: 24px;
}

.task-status {
  /* Auto layout */
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  padding: 0px;
  gap: 4px;

  width: 308px;
  height: 32px;
}

.task-status img {
  width: 32px;
  height: 32px;
}

.user-info {
  /* Auto layout */
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  padding: 0px;
  gap: 4px;

  width: 175px;
  height: 40px;
}

.user-info .user-avatar {
  width: 32px;
  height: 32px;
}

.window-buttons {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.window-buttons button {
  all: unset;
  cursor: pointer;
  width: 48px;
  height: 40px;

  display: flex;
  align-items: center;
  justify-content: center;

  transition: background-color 0.3s;
}

.window-close:hover {
  background-color: var(--red);
}

.window-maximize:hover, .window-minimize:hover {
  background-color: var(--gray);
}

.window-buttons img {
  height: 20px;
}
</style>