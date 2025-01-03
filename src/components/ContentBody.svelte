<script lang="ts">
  import noTexture from '$lib/images/no-texture.png';
  import PackDetails from './PackDetails.svelte';

  export let pack: any;
</script>

<div class="content-body">
    <div class="modpack-caption" style="background-image: url({noTexture})">
        <p>Namashka Craft</p>
    </div>
    <PackDetails pack={pack} />
</div>

<style>
.content-body {
  flex-grow: 1;
  background-color: var(--black-2);
  border: 1px solid var(--gray);
  border-top-left-radius: 32px;
  border-right: none;
  border-bottom: none;

  display: flex;
  flex-direction: column;
}

.modpack-caption {
    height: 37.04vh;

    display: flex;
    align-items: flex-end;
    padding: 16px;
}

.modpack-caption p {
    color: #fff;
    font-size: 64px;
    font-weight: 500;
    -webkit-text-stroke-width: 2px;
    -webkit-text-stroke-color: black;

    margin: 0;
}
</style>