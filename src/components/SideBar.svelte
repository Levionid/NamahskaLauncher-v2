<script lang="ts">
  import logoImage from '$lib/images/logo.png';
  import settingsSvg from '$lib/images/settings.svg';
  import PackTabs from '../components/PackTabs.svelte';

  export let packs: any[];
  export let selectedPackIndex: number;
  export let handlePackChange: (index: number) => void;
</script>

<div class="layout-sidebar">
  <img class="logo" src={logoImage} alt="Лого" />
  <div class="horizontal-line"></div>
  <PackTabs
    packs={packs}
    selectedPackIndex={selectedPackIndex}
    handlePackChange={handlePackChange}
  />
  <div class="horizontal-line"></div>
  <div class="settings-button">
    <img src={settingsSvg} alt="Settings" />
  </div>
</div>

<style>
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
  height: 1px;
  background-color: var(--gray);
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
</style>