<script lang="ts">
    import Password from "$lib/password.svelte";
    import Adder from "$lib/adder.svelte";
    import Fuse from "fuse.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";

    export let lock: Function;

    let passwords: Array<PasswordObject> = [];
    let highlightedSearchItems: Array<PasswordObject> = [];
    let query = "";
    let adderActive = false;
    let fuser: Fuse<PasswordObject>;

    onMount(async () => {
        await get_all_items();
        highlightedSearchItems = [...passwords];
        fuser = new Fuse(passwords, {
            keys: ["appname", "username"],
        });
    });

    function formChange(e: Event) {
        if (e.target == null) {
            return;
        }

        const target: EventTarget = e.target;

        if (target instanceof HTMLInputElement) {
            query = target.value;
            highlightedSearchItems = fuser.search(query).map((x) => x.item);
        }

        if (query === "") {
            highlightedSearchItems = [...passwords];
        }
    }

    async function get_all_items() {
        passwords = [];
        const response: string[] = await invoke("get_json_items");

        for (const entry of response) {
            const entryObject = JSON.parse(entry) as PasswordObject;
            passwords.push(entryObject);
        }

        passwords = [...passwords];
        highlightedSearchItems = [...passwords];
    }

    interface PasswordObject {
        appname: string;
        username: string;
        password: string;
    }
</script>

<nav
    class="bg-slate-600/10 h-20 w-full absolute top-0 flex flex-row items-center"
>
    <button
        class="text-white font-mono flex p-4 items-center gap-2"
        on:click={() => {
            adderActive = !adderActive;
        }}
    >
        <span class="text-white">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M9.5 16q-2.725 0-4.612-1.888T3 9.5q0-2.725 1.888-4.612T9.5 3q2.725 0 4.613 1.888T16 9.5q0 1.1-.35 2.075T14.7 13.3l5.6 5.6q.275.275.275.7t-.275.7q-.275.275-.7.275t-.7-.275l-5.6-5.6q-.75.6-1.725.95T9.5 16m0-2q1.875 0 3.188-1.312T14 9.5q0-1.875-1.312-3.187T9.5 5Q7.625 5 6.313 6.313T5 9.5q0 1.875 1.313 3.188T9.5 14"
                /></svg
            >
        </span>
    </button>

    {#if !adderActive}
        <!-- svelte-ignore a11y-autofocus -->
        <input
            type="text"
            class="bg-slate-200/5 p-3 rounded-lg outline-none h-12 placeholder-gray-200/20 overflow-x-scroll w-full text-gray-200"
            placeholder="search passwords"
            transition:slide={{ duration: 300, delay: 10, axis: "x" }}
            bind:value={query}
            autocomplete="off"
            autofocus={true}
            autocorrect="off"
            autocapitalize="off"
            spellcheck="false"
            on:input|preventDefault={formChange}
        />
    {/if}
    <button
        class="ml-auto mr-2 text-white font-mono flex p-4 items-center gap-2"
        on:click={() => {
            adderActive = !adderActive;
        }}
    >
        <span class="text-white">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M13 14h2v-3h3V9h-3V6h-2v3h-3v2h3zm-5 4q-.825 0-1.412-.587T6 16V4q0-.825.588-1.412T8 2h12q.825 0 1.413.588T22 4v12q0 .825-.587 1.413T20 18zm0-2h12V4H8zm-4 6q-.825 0-1.412-.587T2 20V6h2v14h14v2zM8 4v12z"
                /></svg
            >
        </span>
    </button>
</nav>

{#if adderActive}
    <Adder get_all_items={() => get_all_items()} />
{:else}
    <ol class="grid w-2/3 mt-20 gap-6">
        {#each highlightedSearchItems as password, index}
            <span transition:fade={{ duration: 100 * (index+1), delay: 10*(index+2)}}>
                <Password {password} {query} />
            </span>
        {/each}
    </ol>
{/if}
