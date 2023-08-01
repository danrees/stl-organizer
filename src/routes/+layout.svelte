<script>
  // Your selected Skeleton theme:
  import "@skeletonlabs/skeleton/themes/theme-gold-nouveau.css";
  // This contains the bulk of Skeletons required styles:
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.postcss";
  import {
    AppBar,
    AppShell,
    Drawer,
    Toast,
    drawerStore,
  } from "@skeletonlabs/skeleton";
  import Navigation from "$lib/components/navigation/Navigation.svelte";

  function drawerOpen() {
    drawerStore.open({});
  }
</script>

<Drawer>
  <Navigation />
</Drawer>
<Toast />
<AppShell slotSidebarLeft="bg-surface-500/5 w-0 lg:w-64">
  <svelte:fragment slot="header">
    <AppBar>
      <svelte:fragment slot="lead">
        <div class="flex items-center">
          <button class="lg:hidden btn btn-sm mr-4" on:click={drawerOpen}>
            <span>
              <svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
                <rect width="100" height="20" />
                <rect y="30" width="100" height="20" />
                <rect y="60" width="100" height="20" />
              </svg>
            </span>
          </button>
          <strong class="text-xl uppercase"
            ><a href="/">STL Organizer</a></strong
          >
        </div>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <svelte:fragment slot="sidebarLeft">
    <Navigation />
  </svelte:fragment>
  <div class="p-5">
    <slot />
  </div>
</AppShell>
