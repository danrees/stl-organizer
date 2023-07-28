<script>
// Your selected Skeleton theme:
import '@skeletonlabs/skeleton/themes/theme-gold-nouveau.css';
// This contains the bulk of Skeletons required styles:
import '@skeletonlabs/skeleton/styles/skeleton.css';
import "../app.postcss";
import {AppShell} from "@skeletonlabs/skeleton";

</script>
<AppShell slotSidebarLeft="bg-surface-500/5 w-56 p-4">
  <svelte:fragment slot="sidebarLeft">
    <nav class="list-nav">
      <ul>
      <li>Menu</li>
      </ul>
    </nav>
  </svelte:fragment>
<slot></slot>

</AppShell>
