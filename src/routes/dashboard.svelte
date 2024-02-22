<script lang="ts">
    // this open function is just for getting into the password interface
    export let open: Function;

    const openNew = () => {
        // call tauri api here
        open();
    };

    const OpenLastOpened = () => {
        // call tauri api here
        open();
    };
</script>

<main class="grid gap-4 mt-10 w-2/3">
    <button
        class="p-2 w-full shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-90 duration-100"
        on:click={() => openNew()}
    >
        open last opened
    </button>
    <button
        class="p-2 shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-90 duration-100"
        on:click={() => OpenLastOpened()}
    >
        open new</button
    >
</main>
