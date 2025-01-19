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
    export let userName: string;

    async function saveNickname() {
        try {
        await invoke("set_nickname", { nickname: userName });
        console.log("Никнейм успешно сохранен:", userName);
        } catch (error) {
        console.error("Ошибка при сохранении ника:", error);
        }
    }
  
    function pluralize(value: number, one: string, few: string, many: string): string {
        const mod10 = value % 10;
        const mod100 = value % 100;
        
        if (mod10 === 1 && mod100 !== 11) return one;
        if ((mod10 >= 2 && mod10 <= 4) && (mod100 < 10 || mod100 >= 20)) return few;
        return many;
    }

    function formatLastPlayed(lastPlayedString: string): string {
        const lastPlayed: number = +lastPlayedString;
        if (lastPlayed === 0) {
            return "Никогда";
        }

        const now = Date.now();
        const elapsed = now - lastPlayed * 1000;
        const seconds = Math.floor(elapsed / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);
        const days = Math.floor(hours / 24);
        const months = Math.floor(days / 30);
        const years = Math.floor(days / 365);

        if (seconds < 60) return "Только что";
        if (minutes < 60) return `${minutes} ${pluralize(minutes, "минута", "минуты", "минут")} назад`;
        if (hours < 24) return `${hours} ${pluralize(hours, "час", "часа", "часов")} назад`;
        if (days < 30) return `${days} ${pluralize(days, "день", "дня", "дней")} назад`;
        if (months < 12) return `${months} ${pluralize(months, "месяц", "месяца", "месяцев")} назад`;
        if (years < 10) return `${years} ${pluralize(years, "год", "года", "лет")} назад`;

        const date = new Date(lastPlayed * 1000);
        return date.toLocaleDateString("ru-RU");
    }
  
    function formatPlayTime(playTimeString: string): string {
        const playTime: number = +playTimeString;
        if (playTime < 60) {
            return `${playTime}s`;
        }
        const minutes = playTime / 60;
        if (minutes < 60) {
            return `${minutes.toFixed(1)}m`;
        }
        const hours = minutes / 60;
        return `${hours.toFixed(1)}h`;
    }
  
    async function launchPack() {
    try {
        await saveNickname();
        const result = await invoke('launch_pack', { packName: pack.name, userName: userName, minecraftVersion: pack.minecraft, fabricVersion: pack.fabric });
        console.log("Успешный запуск:", result);
    } catch (error) {
        console.error("Ошибка запуска:", error);
    }
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
                    <span class="value">{pack.minecraft}</span>
                </div>
            </div>
            <div class="fabric-info">
            <img src={fabricIconImage} alt="Fabric" />
            <div class="fabric-version">
                <span class="info">Fabric</span>
                <span class="value">{pack.fabric}</span>
            </div>
            </div>
            <div class="play-stats">
            <img src={clockSvg} alt="Play Time" />
            <div class="play-time-info">
                <span class="info">Время игры</span>
                <span class="value">{formatPlayTime(pack.play_time)}</span>
            </div>
            </div>
            <div class="last-played">
            <img src={revClockSvg} alt="Last Played" />
            <div class="last-played-info">
                <span class="info">Последняя игра</span>
                <span class="value">{formatLastPlayed(pack.last_played)}</span>
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