<script lang="ts">
  import TopBar from '$lib/components/TopBar.svelte';
  import SideBar from '$lib/components/SideBar.svelte';
  import ContentBody from '$lib/components/ContentBody.svelte';

  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let packs: any;
  let selectedPack: any;
  let selectedPackIndex = 0;

  let userName: string = "Username";

  async function fetchNickname() {
    try {
      const nickname: string = await invoke("get_nickname");
      userName = nickname;
      console.log("Nickname получен:", userName);
    } catch (error) {
      console.error("Ошибка при получении ника:", error);
    }
  }

  const loadPacks = async () => {
    packs = await invoke('get_packs') as any[];
    selectedPack = packs[selectedPackIndex];
    console.log(packs);
  };

  function handlePackChange(index: number) {
    selectedPackIndex = index;
    selectedPack = packs[index];

  }

  fetchNickname();

  onMount(() => {
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
    <TopBar userName={userName} />
    {#if selectedPack}
      <ContentBody pack={selectedPack} userName={userName} />
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