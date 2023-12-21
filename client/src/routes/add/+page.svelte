<script lang="ts">
  import { axios, displayTex } from "$lib";
  let body =
    "Find and classify the stationary points of the following function: $$f(x, y) = 2x^3 -6xy + 3y^2$$";
  let soln = "";
  let source = "";
  let author = "";

  let solnLink = "";

  let modules = [{ id: -1, title: "New module" }];
  let topics = [{ id: -1, title: "New topic" }];
  let selectedModule = -2;
  let selectedTopic = -2;
  let newModule = "";
  let newTopic = "";

  let errors = [];
  let waiting = false;

  function handleFormSubmit() {
    if (waiting) return;

    errors = [];
    if (selectedModule < -1) errors = ["Please select a module", ...errors];
    if (selectedTopic < -1) errors = ["Please select a topic", ...errors];
    if (body.replaceAll(/\s+/g, "") === "") errors = ["Please enter a problem", ...errors];
    if (errors.length > 0) return;

    waiting = true;
    axios
      .post("/problems/create", {
        source,
        author,
        body,
        soln: soln.replaceAll(/\s+/g, "") === "" ? null : soln,
        solnlink: solnLink,
        module: selectedModule === -1 ? newModule : selectedModule,
        topic: selectedTopic === -1 ? newTopic : selectedTopic,
      })
      .then((res) => {
        body = "";
        soln = "";
        if (selectedModule === -1) selectedModule = -2;
        if (selectedTopic === -1) selectedTopic = -2;
        waiting = false;
      })
      .catch((error) => {
        console.warn(error);
        waiting = false;
      });
  }
</script>

<form class="flex flex-col items-center p-12">
  <div class="grid grid-cols-2 w-full gap-8">
    <div class="flex flex-col p-5 gap-4 border border-dust bg-coal">
      <select name="module" bind:value={selectedModule} class="p-1 bg-midnight text-white">
        <option value={-2} disabled selected>Module</option>
        {#each modules as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedModule === -1}
        <input
          bind:value={newModule}
          placeholder="Module name"
          class="p-1 bg-midnight text-white"
        />
      {/if}
      <select name="topic" bind:value={selectedTopic} class="p-1 bg-midnight text-white">
        <option value={-2} disabled selected>Topic</option>
        {#each topics as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedTopic === -1}
        <input bind:value={newTopic} placeholder="Topic name" class="p-1 bg-midnight text-white" />
      {/if}
      <input
        bind:value={author}
        placeholder="Problem author"
        class="border-black p-1 w-full bg-midnight text-white"
      />
      <input
        bind:value={source}
        placeholder="Problem source (e.g. 2MVA Lecture Notes)"
        class="p-1 w-full bg-midnight text-white"
      />
      <div>
        <label class="block text-white">Problem statement</label>
        <textarea bind:value={body} class="h-36 w-full p-1 font-mono bg-midnight text-white" />
      </div>
      <div>
        <label class="block text-white">Solution</label>
        <textarea bind:value={soln} class="h-56 w-full p-1 font-mono bg-midnight text-white" />
      </div>
      {#if soln === ""}
        <div>
          <label class="block text-white"
            >Where can the solution be found? (e.g. URL or textbook)</label
          >
          <input bind:value={solnLink} class="p-1 w-full bg-midnight text-white" />
        </div>
      {/if}
      <div>
        {#each errors as error (error)}
          <div class="block text-red-700">{error}</div>
        {/each}
      </div>
    </div>
    <div class="bg-gray-400 p-6 border border-dust bg-coal">
      <p class="bg-[#fff] font-serif rounded-md p-3">
        {@html displayTex(body)}
      </p>
      {#if soln !== ""}
        <p class="bg-[#fff] font-serif rounded-md p-3 mt-6">
          {@html displayTex(soln)}
        </p>
      {/if}
    </div>
  </div>
  <button
    on:click={handleFormSubmit}
    class={`mt-5 py-2 px-8 rounded-sm ${
      waiting ? "bg-gray-400 cursor-wait" : "bg-fern text-white"
    }`}
  >
    Submit
  </button>
</form>
