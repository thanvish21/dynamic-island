<script lang="ts">
  import Island from '$lib/components/Island.svelte';
</script>

<Island />

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    overflow: hidden;
    font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
  }
  :global(html) {
    background: transparent;
  }
  :global(*) {
    box-sizing: border-box;
  }
</style>
