<script lang="ts">
  import TopBar from '../components/TopBar.svelte';
  import SideBar from '../components/SideBar.svelte';
  import ContentBody from '../components/ContentBody.svelte';

  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';

  const appWindow = getCurrentWindow();

  let packs: any;
  let selectedPack: any;
  let selectedPackIndex = 0;

  const loadPacks = async () => {
    packs = await invoke('get_packs') as any[];
    selectedPack = packs[selectedPackIndex];
    console.log(packs);
  };

  function handlePackChange(index: number) {
    selectedPackIndex = index;
    selectedPack = packs[index];
  }

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

  onMount(() => {
    setupWindowControls(); 
    loadPacks();
  });
</script>

<main>
  <SideBar 
    packs={packs} 
    selectedPackIndex={selectedPackIndex} 
    handlePackChange={handlePackChange} 
  />
  <div class="layout-mainbar">
    <TopBar />
    {#if selectedPack}
      <ContentBody pack={selectedPack} />
    {/if}
  </div>
</main>

<style>
main {
  width: 100vw;
  height: 100vh;
  
  display: flex;
  flex-direction: row;
}

.layout-mainbar {
  flex-grow: 1;

  display: flex;
  flex-direction: column;
}
</style>