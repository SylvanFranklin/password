<script>
    import "../app.pcss";
    import Page from "./+page.svelte";
    import Dashboard from "./dashboard.svelte";
    let open = false;
    //  import { appWindow } from "@tauri-apps/api/window"; 
</script>

<!-- <nav class="flex w-full rounded-t-lg  p-2 space-x-2"> -->
<!--     <button -->
<!--         class="w-3 h-3 bg-red-500/90 rounded-full" -->
<!--         on:click={() => appWindow.close()} -->
<!--     /> -->
<!---->
<!--     <button -->
<!--         class="w-3 h-3 bg-yellow-500/90 rounded-full" -->
<!--         on:click={() => appWindow.minimize()} -->
<!--     /> -->
<!---->
<!--     <button -->
<!--         class="w-3 h-3 bg-green-500/90 rounded-full" -->
<!--         on:click={() => appWindow.toggleMaximize()} -->
<!--     /> -->
<!-- </nav> -->


<!-- nav bar for native macos support -->
<main class="flex flex-col w-full h-full mt-20 items-center ">
    <!-- <h1 -->
    <!--     class="text-3xl font-bold font-mono p-2 bg-blue-400 text-white rounded-lg shadow-lg" -->
    <!-- > -->
    <!--     PASSWORD MANAGER -->
    <!-- </h1> -->
    {#if open}
        <Page lock={() => (open = false)} />
    {:else}
        <Dashboard open={() => (open = true)} />
    {/if}
</main>
