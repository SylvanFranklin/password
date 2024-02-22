<script>
    import "../app.pcss";
    import Page from "./+page.svelte";
    import Dashboard from "./dashboard.svelte";
    let open = false;
</script>

<main class="flex flex-col w-full h-full mt-20 items-center">
    <!-- <h1 -->
    <!--     class="text-3xl font-bold font-mono p-2 bg-blue-400 text-white rounded-lg shadow-lg" -->
    <!-- > -->
    <!--     PASSWORD MANAGER -->
    <!-- </h1> -->
    {#if open}
        <Page lock={() => (open = false)}/>
    {:else}
        <Dashboard open={() => (open = true)} />
    {/if}
</main>

