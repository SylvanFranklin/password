<script lang="ts">
    import { Input } from "postcss";
    import Password from "$lib/password.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    // this open function is just for getting into the password interface
    export let open: Function;
    let newPassword: "";
    let error: string;
</script>

<main class="flex flex-col mt-10 w-2/3 items-center">
    <form
        class="gap-4 flex flex-col w-full"
        on:submit|preventDefault={() => {
            invoke("file_check", { newPassword }).then((response) => {
                if (response) {
                    open();
                } else {
                    error = "Invalid Password";
                }
            });
        }}
    >
        <input
            class="p-2 w-full shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-[.99] duration-100 outline-none"
            type="password"
            placeholder="Encryption Key"
            bind:value={newPassword}
        />
        {#if error}
            <h1 class="font-mono text-red-500">{error}</h1>
        {/if}

        <button
            class="p-2 w-full shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-[.99] duration-100"
            type="submit"
        >
            Open Vault
        </button>
    </form>
</main>
