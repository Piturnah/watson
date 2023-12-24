import { redirect } from "@sveltejs/kit";

// Check if we're authenticated. If not, redirect to login page.
//
// TODO: Refresh token.
export function load({ route }: { route: { id: string } }) {
  if (localStorage.getItem("credential") === null && route.id !== "/login") {
    throw redirect(303, `/login?to=${route.id}`);
  }
}
