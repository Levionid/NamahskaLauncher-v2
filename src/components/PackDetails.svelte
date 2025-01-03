<script lang="ts">
  import playSvg from '$lib/images/play.svg';
  import minecraftVersionImage from '$lib/images/minecraft-version.png';
  import folderSvg from '$lib/images/no-texture.png';
  import { invoke } from '@tauri-apps/api/core';
  
  export let pack: any;
  
  async function launchPack() {
    await invoke('launch_pack', { packName: pack.name });
  }
</script>

<div class="selected-pack">
  <button class="launch-button" on:click={launchPack}>
    <img src={playSvg} alt="Play" />
    <span>ЗАПУСТИТЬ</span>
  </button>
  <div class="pack-info">
    <div class="launch-section">
        <div class="minecraft-info">
        <img src={minecraftVersionImage} alt="Minecraft" />
        <div class="minecraft-version">
            <span class="game-name">Minecraft</span>
            <span class="version-number">{pack.minecraft_version}</span>
        </div>
        </div>
        <div class="fabric-info">
        <img src={minecraftVersionImage} alt="Fabric" />
        <div class="fabric-version">
            <span class="game-name">Fabric</span>
            <span class="version-number">{pack.fabric_version}</span>
        </div>
        </div>
        <div class="play-stats">
        <img src={minecraftVersionImage} alt="Play Time" />
        <div class="play-time-info">
            <span class="info-name">Время игры</span>
            <span class="info-value">{pack.play_time}</span>
        </div>
        </div>
        <div class="last-played">
        <img src={minecraftVersionImage} alt="Last Played" />
        <div class="last-played-info">
            <span class="info-name">Последняя игра</span>
            <span class="info-value">{pack.last_played}</span>
        </div>
        </div>
    </div>
  </div>
  <!--
  <button class="open-folder" on:click={() => invoke('open_folder', { path: pack.folder_path })}>
    <img src={folderSvg} alt="Open Folder" />
  </button>
  -->
</div>

<style>
.selected-pack {
    width: calc(100% - 64px);
    background-color: var(--black-2);

    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 24px 32px;
    gap: 24px;
}

.pack-info {
    width: 100%;

    display: flex;
    flex-direction: row;
    align-items: center;

    color: #fff;
    font-size: 16px;
}

.launch-section {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 12px;

    color: #fff;
    font-size: 16px;
}

.launch-button {
    all: unset;
    cursor: pointer;

    background-color: var(--green);
    border-radius: 8px;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    padding: 16px 64px;
    gap: 8px;

    color: #fff;
    font-size: 20px;

    transition: background-color 0.3s;
}

.launch-button:hover {
    background-color: var(--green-light);
}

.launch-button img {
    height: 100%;
}

.minecraft-info, .fabric-info, .play-stats, .last-played {
    /* Auto layout */
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 4px;
}

.minecraft-info img, .fabric-info img, .play-stats img, .last-played img {
    height: 32px;
}


.minecraft-version, .fabric-version, .play-time-info, .last-played-info {
    /* Auto layout */
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 2px;
}
</style>