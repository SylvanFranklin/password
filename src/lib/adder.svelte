<script lang="ts">
    export let get_all_items: Function;
    import { invoke } from "@tauri-apps/api/tauri";
    import { quintIn, quintOut } from "svelte/easing";
    import { fly, scale, slide } from "svelte/transition";
    let appName = "";
    let username = "";
    let password = "";
</script>

<form
    class="bg-slate-200/5 p-2 flex flex-row shadow-lg text-gray-200 gap-2 rounded-lg font-mono w-full" >
    <!-- <h1 class="font-mono text-xl text-white mx-auto outline-none"> -->
    <!--     Add Password -->
    <!-- </h1> -->

    <span class="gap-3 flex flex-row mx-auto">
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            placeholder="App/Website Name"
            autoCapitalize="off"
            spellCheck="false"
            autoCorrect="off"
            bind:value={appName}
        />
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            autoCapitalize="off"
            spellCheck="false"
            autoCorrect="off"
            placeholder="username"
            bind:value={username}
        />
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            placeholder="password"
            autoCapitalize="off"
            spellCheck="false"
            autoCorrect="off"
            bind:value={password}
        />
        <button
            class="px-4 py-2 rounded-md bg-orange-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
            on:click={() => {
                invoke("write_to_file", {
                    appName,
                    username,
                    password,
                });
                get_all_items();
                appName = "";
                username = "";
                password = "";
            }}>add</button
        >
    </span>
</form>
