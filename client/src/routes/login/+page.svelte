<script lang="ts">
  import { axios } from "$lib";
  import {onMount} from "svelte";

  onMount(() => {
    console.log(document.cookie);
  });
</script>
