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
    <img class="pack-icon" src="/src/lib/images/no-texture.png" alt="pack" />
    </button>
  {/each}
</div>

<style>
.pack-tabs {
  width: 80px;
  flex-grow: 1;
  overflow-y: scroll;
  overflow-x: hidden;

  /* Auto layout */
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.pack-tabs::-webkit-scrollbar {
  width: 4px;
}

.pack-tabs::-webkit-scrollbar-thumb {
  background-color: var(--gray);
  border-radius: 2px;
}

.pack-tabs::-webkit-scrollbar-thumb:hover {
  background-color: var(--gray-light);
}

.pack-tab {
  all: unset;
  cursor: pointer;

  border-radius: 8px;
  padding: 8px;

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