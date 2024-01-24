<script lang="ts">
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { goto } from "$app/navigation";

  const urlParams = new URLSearchParams(window.location.search);
  const redirect = urlParams.get("to");

  let accessToken = "";
  let emailAddress = "";
  let password = "";
  let error = "";
  function register() {
    error = "";
    axios
      .post("/register", {
        req_token: accessToken,
        req_email: emailAddress,
        req_password: password,
      })
      .then(() => {
        axios
          .post("/login", { req_email: emailAddress, req_password: password })
          .then((res) => {
            localStorage.setItem("session", res.data);
            axios.defaults.headers["Authorization"] = res.data;
            goto(redirect === null ? "/" : redirect);
          })
          .catch((e) => (error = e.response.data));
      })
      .catch((e) => {
        console.warn(e);
        error = e.response.data;
      });
  }
</script>

<div class="absolute top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2">
  <Box>
    <form class="flex flex-col gap-2">
      <label for="access_token">Access token: </label>
      <input id="access_token" class="p-2 bg-midnight text-white" bind:value={accessToken} />
      <hr class="text-white my-5" />
      <input
        class="p-2 bg-midnight text-white"
        placeholder="Email address"
        bind:value={emailAddress}
      />
      <input
        class="p-2 bg-midnight text-white"
        placeholder="Password"
        type="password"
        bind:value={password}
      />
      <hr class="text-white my-5" />
      {#if error !== ""}
        <p class="text-red">{error}</p>
      {/if}
      <button class="btn-small btn-white" on:click={register}>Register</button>
    </form>
  </Box>
</div>
