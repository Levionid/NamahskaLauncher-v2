<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    export let pack: any;
  
    async function openFolder(path: string) {
      await invoke('open_folder', { path });
    }
</script>

<div class="files-section">
<h2>Файлы</h2>
<div class="file-counters">
    <div class="file-category">
    <h3>Моды</h3>
    <p>Всего модов: {pack.modsCount}</p>
    <button on:click={() => openFolder(pack.modsPath)}>моды</button>
    </div>
    <div class="file-category">
    <h3>Ресурспаки</h3>
    <p>Всего ресурспаков: {pack.resourcePacksCount}</p>
    <button on:click={() => openFolder(pack.resourcePacksPath)}>РЕСУРСПАКИ</button>
    </div>
    <div class="file-category">
    <h3>Миры</h3>
    <p>Всего миров: {pack.worldsCount}</p>
    <button on:click={() => openFolder(pack.worldsPath)}>МИРЫ</button>
    </div>
    <div class="file-category">
    <h3>Шейдеры</h3>
    <p>Всего шейдеров: {pack.shadersCount}</p>
    <button on:click={() => openFolder(pack.shadersPath)}>ШЕЙДЕР</button>
    </div>
</div>
</div>

<style>
.files-section {
    /* Auto layout */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 0px;
    gap: 16px;

    width: 1760px;
    height: 377px;
}

.files-section h2 {
    width: 215px;
    height: 47px;

    font-family: 'Inter';
    font-style: normal;
    font-weight: 500;
    font-size: 64px;
    line-height: 77px;
    /* identical to box height */
    display: flex;
    align-items: center;
    text-align: center;

    color: #FFFFFF;
}

.file-counters {
    /* Auto layout */
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    padding: 0px;
    gap: 16px;

    width: 1760px;
    height: 314px;
}

.file-category {
    box-sizing: border-box;

    /* Auto layout */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 32px;
    gap: 16px;

    width: 428px;
    height: 314px;

    /* Dark */
    background: #101010;
    /* Gray gray */
    border: 2px solid #4A4A4A;
    border-radius: 16px;
}

.file-category h3 {
    width: 105px;
    height: 44px;

    font-family: 'Inter';
    font-style: normal;
    font-weight: 500;
    font-size: 36px;
    line-height: 44px;
    /* identical to box height */
    display: flex;
    align-items: center;
    text-align: center;

    /* White */
    color: #D9D9D9;
}

.file-category p {
    width: 219px;
    height: 29px;

    font-family: 'Inter';
    font-style: normal;
    font-weight: 400;
    font-size: 24px;
    line-height: 29px;
    /* identical to box height */
    display: flex;
    align-items: center;
    text-align: center;

    /* White */
    color: #D9D9D9;
}

.file-category button {
    width: 79px;
    height: 29px;

    font-family: 'Inter';
    font-style: normal;
    font-weight: 400;
    font-size: 24px;
    line-height: 29px;
    /* identical to box height */
    display: flex;
    align-items: center;
    text-align: center;
    text-transform: uppercase;

    /* Cyan green */
    color: #14CFA3;
}
</style>