<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { writable } from 'svelte/store';

    export let pack: any;

    const modCount = writable(0);
    const resourcePackCount = writable(0);
    const worldCount = writable(0);
    const shaderCount = writable(0);

    async function openFolder(path: string) {
        await invoke('open_folder', { path: `modpacks\\${path}` });
    }

    async function getFileCount(path: string): Promise<number> {
        const count: number = await invoke('get_file_count', { path: `modpacks\\${path}` });
        return count;
    }

    onMount(async () => {
        // Обновляем состояния для каждой категории
        modCount.set(await getFileCount(`${pack.name}\\mods`));
        resourcePackCount.set(await getFileCount(`${pack.name}\\resoursepacks`));
        worldCount.set(await getFileCount(`${pack.name}\\worlds`));
        shaderCount.set(await getFileCount(`${pack.name}\\shaderpacks`));
    });
</script>

<div class="files-section">
    <h2>Файлы</h2>
    <div class="file-counters">
        <div class="file-category">
            <h3>Моды</h3>
            <p>Всего модов: {$modCount}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(`${pack.name}\\mods`)}>моды</button>
        </div>
        <div class="file-category">
            <h3>Ресурспаки</h3>
            <p>Всего ресурспаков: {$resourcePackCount}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(`${pack.name}\\resoursepacks`)}>РЕСУРСПАКИ</button>
        </div>
        <div class="file-category">
            <h3>Миры</h3>
            <p>Всего миров: {$worldCount}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(`${pack.name}\\worlds`)}>МИРЫ</button>
        </div>
        <div class="file-category">
            <h3>Шейдеры</h3>
            <p>Всего шейдеров: {$shaderCount}</p>
            <div class="fill-frame"></div>
            <button on:click={() => openFolder(`${pack.name}\\shaderpacks`)}>ШЕЙДЕРЫ</button>
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