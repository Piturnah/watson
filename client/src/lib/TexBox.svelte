<script lang="ts">
  export let content: string;

  import { displayTex } from "$lib";
</script>

<p class="bg-[#fff] font-serif rounded-md p-3 text-[#000]">
  {@html displayTex(content)}
</p>

<style>
  p {
    background-size: 24px 24px;
    background-image: linear-gradient(to right, #e2e2e2 1px, transparent 1px),
      linear-gradient(to bottom, #e2e2e2 1px, transparent 1px);
    background-position: 20px, 10px;
  }
</style>
