<script lang="ts">
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";
  import { goto } from "$app/navigation";
  const urlParams = new URLSearchParams(window.location.search);
  const redirect = urlParams.get("to");

  window.onSignIn = ({ credential }: { credential: string }) => {
    loggingIn = true;
    axios
      .post("/login", { credential })
      .then(() => {
        localStorage.setItem("credential", credential);
        goto(redirect === null ? "/" : redirect);
      })
      .catch((e) => console.warn(e));
  };

  let loggingIn = false;
</script>

<div class="absolute top-1/2 left-1/2 -translate-y-1/2 -translate-x-1/2">
  <Box>
    {#if !loggingIn}
      <div
        id="g_id_onload"
        data-client_id="831067350519-i2ce97b0d7ru82eh3e394dg6j332nmi8.apps.googleusercontent.com"
        data-context="signin"
        data-ux_mode="popup"
        data-callback="onSignIn"
        data-itp_support="true"
        data-auto_select="true"
      ></div>

      <div
        class="g_id_signin"
        data-type="standard"
        data-shape="rectangular"
        data-theme="outline"
        data-text="signin_with"
        data-size="large"
        data-logo_alignment="left"
      ></div>
    {:else}
      Signing you in...
    {/if}
  </Box>
</div>
