<script lang="ts">
  import Box from "$lib/Box.svelte";
  import { axios } from "$lib";

  window.onSignIn = ({ credential }: { credential: string }) => {
    loggingIn = true;
    axios
      .post("/login", { credential })
      .then((res) => console.log(res))
      .catch((e) => console.warn(e));
  };

  let loggingIn = false;
</script>

<Box>
  {#if !loggingIn}
    <div
      id="g_id_onload"
      data-client_id="831067350519-i2ce97b0d7ru82eh3e394dg6j332nmi8.apps.googleusercontent.com"
      data-context="signin"
      data-ux_mode="popup"
      data-callback="onSignIn"
      data-itp_support="true"
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
