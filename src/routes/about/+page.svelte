<script context="module">
  export const prerender = true;
</script>

<script lang="ts">
  import { goto } from '$app/navigation';

function goBack() {
  goto('/'); // Возврат на главную страницу
}
</script>

<main>
  <h1>О нас</h1>
  <p>Добро пожаловать на страницу "О нас"!</p>
  <button on:click={goBack} class="button">Назад</button>
</main>

<style>
  :root {
    background-color: #121212;
  }
  
  .button{
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    padding: 24px 92px;

    width: 436.83px;
    height: 92px;

    background: #af4c4c;
    border-radius: 16px;

    flex: none;
    order: 0;
    align-self: stretch;
    flex-grow: 0;

    color: #fff;
    font-family: 'Inter';
    font-style: normal;
    font-weight: 500;
    font-size: 36px;
    line-height: 44px;
  }

</style>
