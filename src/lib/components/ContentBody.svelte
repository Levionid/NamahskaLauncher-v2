<script lang="ts">
  import PackDetails from './PackDetails.svelte';
  import { join } from '@tauri-apps/api/path';
  import { onMount } from 'svelte';

  export let pack: any;
  export let userName: string;

  let headerPath: string;

  async function getHeaderPath(packName: string): Promise<string> {
    return await join('modpacks', packName, 'header.png');
  }

  onMount(async () => {
    headerPath = await getHeaderPath(pack.name);
  });
</script>

<div
  class="content-body"
  style="background-image: url({headerPath})"
>
  <div
    class="modpack-caption"
    style="background-image: url({headerPath})"
  >
    <span>{pack.name}</span>
  </div>
  <PackDetails userName={userName} {pack} />
</div>

<style>
.content-body {
  flex-grow: 1;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  border: 1px solid var(--gray);
  border-top-left-radius: 32px;
  border-right: none;
  border-bottom: none;

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  overflow: hidden;
}

.modpack-caption {
    height: 37.04vh;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;

    display: flex;
    align-items: flex-end;
    padding: 12px 16px;
}

.modpack-caption span {
    color: #fff;
    font-size: 64px;
    font-weight: 500;
    text-shadow: 0px 0px 4px #000;

    margin: 0;
}
</style>