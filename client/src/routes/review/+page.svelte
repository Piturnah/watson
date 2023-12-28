<script>
  import TexBox from "$lib/TexBox.svelte";
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { get } from "svelte/store";
  import { selected_topic_ids } from "$lib/stores";
  import { onMount } from "svelte";

  let showSoln = false;

  let body = "";
  let soln = "";
  function getProblem() {
    axios
      .post("/problems/request", { topic_ids: get(selected_topic_ids) })
      .then(({ data }) => {
        body = data.problem.body;
        soln = data.solution;
      })
      .catch((e) => console.warn(e));
  }
  onMount(getProblem);
</script>

<div class="grid grid-cols-1 p-12 w-[959px] m-auto gap-6">
  <Box>
    <TexBox content={body} />
  </Box>
  {#if showSoln}
    <Box>
      <TexBox content={soln} />
    </Box>
    <div class="flex flex-row-reverse w-full gap-4">
      <button class="btn btn-green"> Correct </button>
      <button class="btn btn-red"> Incorrect </button>
    </div>
  {:else}
    <div class="flex flex-row-reverse justify-between w-full">
      <button on:click={() => (showSoln = !showSoln)} class="btn btn-grey"> Show solution </button>
      <a href="/" class="btn btn-grey"> Back </a>
    </div>
  {/if}
</div>
