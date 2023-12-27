<script lang="ts">
  import { onMount } from "svelte";
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";

  let modules: { id: number; title: string }[] = [];
  let topics: { id: number; module_id: number; title: string }[] = [];
  onMount(() => {
    axios
      .get("/modules")
      .then(({ data }) => {
        modules = data.modules;
        topics = data.topics;
      })
      .catch((e) => console.warn(e));
  });

  let moduleCheckboxes = {};
  let topicCheckboxes = {};
</script>

<div class="grid grid-cols-3 w-full justify-between items-center absolute top-1/2 -translate-y-1/2">
  <div class="bg-coal border border-dust p-6 ml-10">
    <h1 class="text-lg font-bold">Filter Topics</h1>
    <ul class="flex flex-col gap-2 mt-5">
      {#each modules as { id, title } (id)}
        <li>
          <div class="bg-midnight p-2 border border-dust flex justify-between">
            {title}
            <input
              type="checkbox"
              bind:checked={moduleCheckboxes[id]}
              on:click={() =>
                topics
                  .filter((topic) => topic.module_id === id)
                  .forEach((topic) => (topicCheckboxes[topic.id] = !moduleCheckboxes[id]))}
            />
          </div>
          <ul class="flex flex-col gap-1 p-2">
            {#each topics.filter(({ module_id }) => module_id === id) as topic (topic.id)}
              <li class="flex justify-between flex-shrink-0 whitespace-nowrap ml-5">
                <div class="flex-1">{topic.title}</div>
                <div
                  class="border-b-2 border-white w-full border-dotted flex-shrink mb-1.5 border-opacity-50 mr-0.5"
                ></div>
                <input
                  type="checkbox"
                  bind:checked={topicCheckboxes[topic.id]}
                  on:click={() =>
                    topicCheckboxes[topic.id] === true
                      ? (moduleCheckboxes[topic.module_id] = false)
                      : ""}
                />
              </li>
            {/each}
          </ul>
        </li>
      {/each}
    </ul>
  </div>
  <div class="m-auto">
    <Box>
      <a href="/review" class="btn btn-white">Review</a>
      <a href="/add" class="btn btn-white">Add problems</a>
    </Box>
  </div>
</div>
