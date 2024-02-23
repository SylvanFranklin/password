<script lang="ts">
    export let lock: Function;
    import Password from "$lib/password.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    let appName = "";
    let username = "";
    let password = "";

    const get_all_passwords = async () => {
        const passwords = await invoke("get_all_passwords", {});
        console.log(passwords);
    };

    interface PasswordObject {
        website: string;
        username: string;
        password: string;
    }

    const passwords: PasswordObject[] = [];
</script>

<button
    class="p-2 w-min shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-90 duration-100 absolute left-10 top-10"
    on:click={() => lock()}
    >Lock
</button>
<form
    class=" w-2/3 flex bg-slate-600/10 shadow-lg flex-col text-gray-200 p-4 gap-2 rounded-lg font-mono mt-10"
>
    <h1 class="font-mono text-xl text-white mx-auto outline-none">
        Add Password
    </h1>

    <span class="gap-3 flex flex-col">
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            placeholder="App/Website Name"
            bind:value={appName}
        />
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            placeholder="username"
            bind:value={username}
        />
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none"
            placeholder="password"
            bind:value={password}
        />
        <button
            class="px-4 py-2 rounded-md bg-orange-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
            on:click={() =>
                invoke("write_to_file", { appName, username, password })}
            >add</button
        >
        <button
            class="px-4 py-2 rounded-md bg-red-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
            on:click={() => get_all_passwords()}>test button</button
        >
    </span>
</form>

<ol class="grid w-2/3 mt-20 gap-6">
    {#each passwords as password}
        <Password {password} />
    {/each}
</ol>
