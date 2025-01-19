<script lang="ts">
  import closeSvg from '$lib/images/close.svg';
  import maximizeSvg from '$lib/images/maximize.svg';
  import minimizeSvg from '$lib/images/minimize.svg';
  import tasksSvg from '$lib/images/tasks.svg';
  import personSvg from '$lib/images/person.svg';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';

  export let main_menu: boolean = true;

  const appWindow = getCurrentWindow();

  export let userName: string;
  let userNameRef: HTMLSpanElement;

  function minimize() {
    appWindow.minimize();
  }

  function toggleMaximize() {
    appWindow.toggleMaximize();
  }

  function close() {
    appWindow.close();
  }

  function stopEditing() {
    const trimmedText = userNameRef.innerText.trim();
    userNameRef.innerText = trimmedText || userName;
    userName = trimmedText || userName;
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (userNameRef.innerText.length >= 16 && event.key !== "Backspace" && event.key !== "Delete") {
      event.preventDefault();
    }
  }
</script>

<header data-tauri-drag-region>
  {#if main_menu}
    <div class="task-status">
      <img src={tasksSvg} alt="Task Status" />
      <span>Нет запущенных задач</span>
    </div>
    <div class="user-info">
      <img class="user-avatar" src={personSvg} alt="Аватар" />
      <span
        class="user-name"
        contenteditable="true"
        role="textbox"
        tabindex="0"
        bind:this={userNameRef}
        on:blur={stopEditing}
        on:keydown={handleKeyDown}
      >
        {userName}
      </span>
    </div>
  {/if}
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
  height: 40px;
  background-color: var(--black);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-direction: row;
  gap: 32px;
  color: #d9d9d9;
  font-size: 20px;
}

.task-status, .user-info {
  display: flex;
  align-items: center;
  gap: 4px;
}

.task-status img, .user-avatar {
  width: 32px;
  height: 32px;
}

.user-info {
  cursor: pointer;
  transition: opacity 0.2s;
}

.user-info:hover {
  opacity: 0.8;
}

.user-name {
  display: inline-block;
  min-width: 5px;
  background-color: transparent;
  border: 1px solid transparent;
  outline: none;
  text-align: center;
  white-space: nowrap;
  transition: border 0.2s, background-color 0.2s;
}

.window-buttons {
  display: flex;
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