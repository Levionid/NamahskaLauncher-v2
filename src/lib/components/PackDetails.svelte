<script lang="ts">
  import FilesSection from './FilesSection.svelte';
  import playSvg from '$lib/images/play.svg';
  import minecraftIconImage from '$lib/images/minecraft-icon.png';
  import fabricIconImage from '$lib/images/fabric-icon.png';
  import clockSvg from '$lib/images/clock.svg';
  import revClockSvg from '$lib/images/rev-clock.svg';
  import folderSvg from '$lib/images/folder.svg';
  import { invoke } from '@tauri-apps/api/core';
  
  export let pack: any;
  
  async function launchPack() {
    await invoke('launch_pack', { packName: pack.name });
  }
</script>

<div class="selected-pack">
    <div class="pack-info">
        <button class="launch-button" on:click={launchPack}>
          <img src={playSvg} alt="Play" />
          <span>ЗАПУСТИТЬ</span>
        </button>
        <div class="launch-section">
            <div class="minecraft-info">
                <img src={minecraftIconImage} alt="Minecraft" />
                <div class="minecraft-version">
                    <span class="info">Minecraft</span>
                    <span class="value">{pack.minecraft_version}</span>
                </div>
            </div>
            <div class="fabric-info">
            <img src={fabricIconImage} alt="Fabric" />
            <div class="fabric-version">
                <span class="info">Fabric</span>
                <span class="value">{pack.fabric_version}</span>
            </div>
            </div>
            <div class="play-stats">
            <img src={clockSvg} alt="Play Time" />
            <div class="play-time-info">
                <span class="info">Время игры</span>
                <span class="value">{pack.play_time}</span>
            </div>
            </div>
            <div class="last-played">
            <img src={revClockSvg} alt="Last Played" />
            <div class="last-played-info">
                <span class="info">Последняя игра</span>
                <span class="value">{pack.last_played}</span>
            </div>
        </div>
    </div>
    <div class="fill-frame"></div>
    <button class="open-folder" on:click={() => invoke('open_folder', { path: pack.folder_path })}>
      <img src={folderSvg} alt="Open Folder" />
    </button>
  </div>
  <FilesSection pack={pack} />
</div>

<style>
.selected-pack {
    flex-grow: 1;

    background-color: #121212dd;
    backdrop-filter: blur(60px);
    box-shadow: inset 0px 4px 4px rgba(255, 255, 255, 0.15);
    box-shadow: inset 0px 0px 68px rgba(255, 255, 255, 0.05);

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 12px 20px;
    gap: 24px;
}

.pack-info {
    width: 100%;

    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 10px;
}

.launch-section {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 12px;
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

.info {
    color: #fff;
    font-size: 16px;
    font-weight: 500;
}

.value {
    color: #d9d9d9;
    font-size: 16px;
    font-weight: 400;
}

.fill-frame {
    flex-grow: 1;
}

.open-folder {
    all: unset;
    cursor: pointer;

    width: 32px;
    height: 32px;
    border-radius: 24px;
    padding: 8px;

    transition: background-color 0.3s;
}

.open-folder:hover {
    background-color: #fff2;
}

.open-folder img {
    width: 32px;
    height: 32px;
}
</style>