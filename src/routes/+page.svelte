<script lang="ts">
  import closeSvg from './assets/images/close.svg';
  import maximizeSvg from './assets/images/maximize.svg';
  import minimizeSvg from './assets/images/minimize.svg';
  import settingsSvg from './assets/images/settings.svg';
  import logoImage from './assets/images/logo.png';
  import catalogImage from './assets/images/catalog.png';
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const catalog1Image = catalogImage;
  const catalog2Image = catalogImage;
  const catalog3Image = catalogImage;

  const appWindow = getCurrentWindow();

  function setupWindowControls() {
    const minimizeButton = document.getElementById('minimize');
    const maximizeButton = document.getElementById('maximize');
    const closeButton = document.getElementById('close');

    if (minimizeButton) {
      minimizeButton.addEventListener('click', () => appWindow.minimize());
    }

    if (maximizeButton) {
      maximizeButton.addEventListener('click', () => appWindow.toggleMaximize());
    }

    if (closeButton) {
      closeButton.addEventListener('click', () => appWindow.close());
    }
  }

  onMount(setupWindowControls);
</script>

<main>
  <div class="layout-sidebar">
    <img class="logo" src={logoImage} alt="Лого" />
    <div class="horizontal-line"></div>
    <div class="sidebar-frame">
      <button class="catalog-image" id="catalog1">
        <img src={catalog1Image} alt="Catalog 1" />
      </button>
      <button class="catalog-image" id="catalog2">
        <img src={catalog2Image} alt="Catalog 2" />
      </button>
      <button class="catalog-image" id="catalog3">
        <img src={catalog3Image} alt="Catalog 3" />
      </button>
    </div>
    <div class="horizontal-line"></div>
    <div class="settings-button">
      <img src={settingsSvg} alt="Settings" />
    </div>
  </div>
  <div class="layout-mainbar">
    <header data-tauri-drag-region>
      <div class="window-buttons">
        <button class="window-minimize" id="minimize" title="Свернуть">
          <img src={minimizeSvg} alt="Minimize" />
        </button>
        <button class="window-maximize" id="maximize" title="Развернуть">
          <img src={maximizeSvg} alt="Maximize" />
        </button>
        <button class="window-close" id="close" title="Закрыть">
          <img src={closeSvg} alt="Close" />
        </button>
      </div>
    </header>
    <div class="content-body"></div>
  </div>
</main>

<style>
:root {
  --black: #101010;
  --black-2: #121212;
  --gray: #4A4A4A;
  --red: #e81123;

  background-color: var(--black);

  user-select: none;
}

main {
  width: 100vw;
  height: 100vh;
  
  display: flex;
  flex-direction: row;
}

.layout-sidebar {
  width: 64px;
  background-color: var(--black);

  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 16px 8px;
  gap: 16px;
}

.logo {
  width: 52px;
  height: 52px;
}

.horizontal-line {
  width: 100%;
  height: 4px;
  border-radius: 2px;
  background-color: var(--gray);
}

.sidebar-frame {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.catalog-image {
  all: unset;
  cursor: pointer;

  width: 52px;
  height: 52px;
  padding: 8px;

  border-radius: 8px;

  transition: background-color 0.3s;
}

.catalog-image:hover {
  background-color: var(--gray);
}

.catalog-image img {
  width: 52px;
  height: 52px;
  border-radius: 26px;
}

.settings-button {
  width: 100%;
  border-radius: 8px;

  display: flex;
  align-items: center;
  justify-content: center;

  transition: background-color 0.3s;
}

.settings-button:hover {
  background-color: var(--gray);
}

.settings-button img {
  height: 32px;
  margin: 4px;
}

.layout-mainbar {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.content-body {
  border-style: solid;
  border-color: var(--gray);
  border-radius: 32px 0px 0px 0px;
  border-width: 2px;

  border-right: none;
  border-bottom: none;
}

header {
  width: 100%;
  height: 40px;
  background-color: var(--black);

  display: flex;
  align-items: center;
  justify-content: flex-end;

  flex-direction: row;
  gap: 32px;
}

.window-buttons {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.content-body {
  flex-grow: 1;
  background-color: var(--black-2);
}

#close, #maximize, #minimize {
  all: unset;
  cursor: pointer;
  width: 48px;
  height: 40px;

  display: flex;
  align-items: center;
  justify-content: center;

  transition: background-color 0.3s;
}

#close:hover {
  background-color: var(--red);
}

#maximize:hover, #minimize:hover {
  background-color: var(--gray);
}

.window-buttons img {
  height: 20px;
}
</style>