<script lang="ts">
  import { drawerStore } from "@skeletonlabs/skeleton";

  function drawerClose() {
    drawerStore.close();
  }
</script>

<nav class="list-nav p-4">
  <ul>
    <li><a href="/library" on:click={drawerClose}>Library Management</a></li>
    <li><a href="/files" on:click={drawerClose}>File Management</a></li>
    <li><a href="/tags" on:click={drawerClose}>Tag Management</a></li>
  </ul>
</nav>
