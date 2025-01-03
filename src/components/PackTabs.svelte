<script lang="ts">
    export let packs: any[];
    export let selectedPackIndex: number;
    export let handlePackChange: (index: number) => void;
</script>

<div class="pack-tabs">
  {#each packs as pack, index}
    <button 
      class="pack-tab" 
      class:open={selectedPackIndex === index}
      on:click={() => handlePackChange(index)}
    >
      <img class="pack-icon" src={pack.icon_url} alt={pack.name} />
    </button>
  {/each}
</div>

<style>
.pack-tabs {
  width: 80px;
  height: 100%;

  /* Auto layout */
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.pack-tab {
  all: unset;
  cursor: pointer;

  width: 64px;
  height: 64px;
  border-radius: 8px;

  /* Auto layout */
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;

  transition: background-color 0.3s;
}

.pack-tab:hover {
  background-color: var(--gray);
}

.pack-tab.open {
  background-color: var(--gray);
}

.pack-tab.open:hover {
  background-color: var(--gray-light);
}

.pack-icon {
  width: 52px;
  height: 52px;

  border-radius: 26px;
}
</style>