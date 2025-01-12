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
            <p>Всего модов: {pack.mods_count}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(pack.mods_path)}>моды</button>
        </div>
        <div class="file-category">
            <h3>Ресурспаки</h3>
            <p>Всего ресурспаков: {pack.resource_packs_count}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(pack.resource_packs_path)}>РЕСУРСПАКИ</button>
        </div>
        <div class="file-category">
            <h3>Миры</h3>
            <p>Всего миров: {pack.worlds_count}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(pack.worlds_path)}>МИРЫ</button>
        </div>
        <div class="file-category">
            <h3>Шейдеры</h3>
            <p>Всего шейдеров: {pack.shaders_count}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(pack.shaders_path)}>ШЕЙДЕР</button>
        </div>
    </div>
</div>

<style>
.files-section {
    width: 100%;
    flex-grow: 1;
    
    /* Auto layout */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
}

.files-section h2 {
    color: #FFFFFF;
    font-size: 32px;
    font-weight: 500;
    margin: 0;
}

.file-counters {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    align-items: stretch;
    gap: 16px;
}

.file-category {
    flex: 1;
    background-color: var(--dark);
    border: 2px solid var(--gray-light);
    border-radius: 16px;

    /* Auto layout */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 32px;
    gap: 16px;
}

.file-category h3 {
    display: flex;
    align-items: center;
    text-align: center;
    margin: 0;
    
    color: #fff;
    font-size: 24px;
    font-weight: 500;
}

.file-category p {
    display: flex;
    align-items: center;
    text-align: center;
    margin: 0;
    
    color: #D9D9D9;
    font-size: 16px;
}

.fill-frame {
    flex-grow: 1;
}

.file-category button {
    all: unset;
    cursor: pointer;

    border-radius: 8px;
    
    display: flex;
    align-items: center;
    text-align: center;
    text-transform: uppercase;
    padding: 4px;
    
    color: #14CFA3;
    font-size: 20px;

    transition: background-color 0.3s;
}

.file-category button:hover {
    background-color: #14CFA320;
}
</style>