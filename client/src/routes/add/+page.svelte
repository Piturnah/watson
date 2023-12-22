<script lang="ts">
  import { onMount } from "svelte";
  import { axios, displayTex } from "$lib";
  let body =
    "Find and classify the stationary points of the following function: $$f(x, y) = 2x^3 -6xy + 3y^2$$";
  //let soln = "";
  let soln = `Find the stationary points
  $$
  \\begin{aligned}
  f_x = 6x^2 - 6y&\\therefore f_x = 0 \\implies& x^2 = y \\\\
  f_y = -6x + 6y&\\therefore f_y = 0 \\implies& x = y
  \\end{aligned}
  $$
  Hence there are two stationary points, at $(0, 0)$ and $(1, 1)$. The Hessian matrix is given by:
  $$
  H = \\begin{pmatrix}
  f_{xx} & f_{xy} \\\\ f_{yx} & f_{yy}
  \\end{pmatrix}
  = \\begin{pmatrix}
  12x & -6 \\\\ -6 & 6
  \\end{pmatrix}. \\\\
  $$
  At $(0, 0)$,
  $$
  \\text{det}(H) = \\begin{vmatrix}
  0 & -6 \\\\ -6 & 6
  \\end{vmatrix}
  = -36 < 0,
  $$
  so $(0, 0)$ is a saddle point. At $(1, 1)$,
  $$
  \\text{det}(H) = \\begin{vmatrix}
  12 & -6 \\\\ -6 & 6
  \\end{vmatrix}
  = 36 > 0.
  $$
  Further, det$(H_1) = 12 > 0$, so $(1,1)$ is a local minimum.`;
  let source = "";
  let author = "";

  let solnLink = "";

  let modules = [{ id: -1, title: "New module" }];
  let topics = [];
  let selectedModule = -2;
  let selectedTopic = -2;
  let newModule = "";
  let newTopic = "";

  let errors = [];
  let waiting = false;

  onMount(() => {
    axios
      .get("/modules")
      .then(({ data }) => {
        modules = [{ id: -1, title: "New module" }, ...data.modules];
        topics = data.topics;
      })
      .catch((e) => console.warn(e));
  });

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
      <select name="module" bind:value={selectedModule} class="p-2 bg-midnight text-white">
        <option value={-2} disabled selected>Module</option>
        {#each modules as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedModule === -1}
        <input
          bind:value={newModule}
          placeholder="Module name"
          class="p-2 bg-midnight text-white"
        />
      {/if}
      <select name="topic" bind:value={selectedTopic} class="p-2 bg-midnight text-white">
        <option value={-2} disabled selected>Topic</option>
        <option value={-1}>New topic</option>
        {#each topics.filter((topic) => topic.module_id === selectedModule) as { id, title } (id)}
          <option value={id}>{title}</option>
        {/each}
      </select>
      {#if selectedTopic === -1}
        <input bind:value={newTopic} placeholder="Topic name" class="p-2 bg-midnight text-white" />
      {/if}
      <input
        bind:value={author}
        placeholder="Problem author"
        class="border-black p-2 w-full bg-midnight text-white"
      />
      <input
        bind:value={source}
        placeholder="Problem source (e.g. 2MVA Lecture Notes)"
        class="p-2 w-full bg-midnight text-white"
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
      <p class="bg-[#fff] font-serif rounded-md p-3 tex">
        {@html displayTex(body)}
      </p>
      {#if soln !== ""}
        <p class="bg-[#fff] font-serif rounded-md p-3 mt-6 tex">
          {@html displayTex(soln)}
        </p>
      {/if}
    </div>
  </div>
  <div class="flex mt-5 gap-5 w-full">
    <button
      on:click={handleFormSubmit}
      class={`py-2 w-36 rounded-sm ${waiting ? "bg-gray-400 cursor-wait" : "bg-fern text-white"}`}
    >
      Submit
    </button>
    <a href="/" class="bg-coal text-white border border-dust py-2 w-36 rounded-sm text-center"
      >Done</a
    >
  </div>
</form>

<style>
.tex {
  background-size: 24px 24px;
  background-image:
    linear-gradient(to right, #e2e2e2 1px, transparent 1px),
    linear-gradient(to bottom, #e2e2e2 1px, transparent 1px);
  background-position: 20px, 10px;
}</style>
