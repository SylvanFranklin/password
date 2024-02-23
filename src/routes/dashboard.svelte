<script lang="ts">
    import { Input } from "postcss";
    import Password from "$lib/password.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    // this open function is just for getting into the password interface
    export let open: Function;
    let encryptionKey: string;

    const openApp = () => {
        invoke("create_if_not_exists");
        open();
    };
</script>

<main class="grid gap-4 mt-10 w-2/3">
    <button
        class="p-2 w-full shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-[.99] duration-100"
        on:click={() => openApp()}
    >
        Open Vault
    </button>
    <input
        class="p-2 w-full shadow-md rounded-md text-xl font-mono text-white bg-slate-600/10 hover:brightness-110 hover:scale-[.99] duration-100"
        type="password"
        placeholder="Encryption Key"
        bind:value={encryptionKey}
    />
</main>
