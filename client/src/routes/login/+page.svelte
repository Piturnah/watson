<script lang="ts">
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { goto } from "$app/navigation";
  const urlParams = new URLSearchParams(window.location.search);
  const redirect = urlParams.get("to");

  let error = "";
  function login() {
    error = "";
    axios
      .post("/login", { req_email: emailAddress, req_password: password })
      .then((res) => {
        console.log(res);
        localStorage.setItem("session", res.data);
        axios.defaults.headers["Authorization"] = res.data;
        goto(redirect === null ? "/" : redirect);
      })
      .catch((e) => {
        console.warn(e);
        error = e.response.data;
      });
  }

  let loggingIn = false;

  let emailAddress = "";
  let password = "";
</script>

<div class="absolute top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2">
  {#if loggingIn}
    <Box>
      <div class="flex flex-col gap-3 items-center">
        <input
          bind:value={emailAddress}
          placeholder="Email address"
          class="p-2 bg-midnight text-white"
        />
        <input
          bind:value={password}
          placeholder="Password"
          class="p-2 bg-midnight text-white"
          type="password"
        />
      </div>
      {#if error !== ""}
        <p class="text-red">{error}</p>
      {/if}
    </Box>
    <div class="flex justify-between w-full mt-4">
      <button class="btn-small btn-white py-2 px-4" on:click={() => (loggingIn = false)}>
        Back
      </button>
      <button class="btn-small btn-green py-2 px-4" on:click={login}>Login</button>
    </div>
  {:else}
    <Box>
      <button class="btn btn-green" on:click={() => (loggingIn = true)}>Login</button>
      <a href={`/register${!!redirect ? `?to=${redirect}` : ""}`} class="btn btn-white">Sign up</a>
    </Box>
  {/if}
</div>
