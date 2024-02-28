<script lang="ts">
    import Password from "$lib/password.svelte";
    import Adder from "$lib/adder.svelte";
    import Fuse from "fuse.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { fade, fly, slide } from "svelte/transition";

    function focus(el: HTMLElement) {
        el.focus();
    }

    function keydown(e: KeyboardEvent) {
        // three important keymaps
        // CMD / CTRL + L -> lock
        // CMD / CTRL + N -> new
        // CMD / CTRL + K -> search

        switch (e.key) {
            case "l":
                if (e.ctrlKey || e.metaKey) {
                    lock();
                }
                break;
            case "n":
                if (e.ctrlKey || e.metaKey) {
                    adderActive = true;
                }
                break;
            case "f":
            case "k":
                if (e.ctrlKey || e.metaKey) {
                    adderActive = false;
                    // fix in the future (find a way to focus when not focused)
                }
                break;
        }
    }

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
    class="bg-slate-600/10 w-full absolute top-0 flex flex-row items-center pt-4 pb-2 px-2"
>
    <button
        class="text-white font-mono flex p-4 items-center gap-2 hover:scale-110 duration-200"
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
        <div class="w-full flex flex-row items-center gap-2">
            <input
                type="text"
                class="bg-slate-200/5 p-4 rounded-lg outline-none h-12 placeholder-gray-200/20 overflow-x-scroll w-full text-gray-200 text-xl font-mono .search"
                placeholder="search passwords"
                transition:slide={{ duration: 400, delay: 0, axis: "x" }}
                bind:value={query}
                autocomplete="off"
                use:focus
                autocorrect="off"
                autocapitalize="off"
                spellcheck="false"
                on:input|preventDefault={formChange}
            />
            <span
                class="text-gray-200/40 bg-slate-200/5 rounded-md shadow-lg p-3 flex flex-row gap-2"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="28"
                    height="28"
                    viewBox="0 0 24 24"
                    ><path
                        fill="currentColor"
                        d="M17.5 3C15.57 3 14 4.57 14 6.5V8h-4V6.5C10 4.57 8.43 3 6.5 3S3 4.57 3 6.5S4.57 10 6.5 10H8v4H6.5C4.57 14 3 15.57 3 17.5S4.57 21 6.5 21s3.5-1.57 3.5-3.5V16h4v1.5c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5s-1.57-3.5-3.5-3.5H16v-4h1.5c1.93 0 3.5-1.57 3.5-3.5S19.43 3 17.5 3M16 8V6.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5S18.33 8 17.5 8zM6.5 8C5.67 8 5 7.33 5 6.5S5.67 5 6.5 5S8 5.67 8 6.5V8zm3.5 6v-4h4v4zm7.5 5c-.83 0-1.5-.67-1.5-1.5V16h1.5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5m-11 0c-.83 0-1.5-.67-1.5-1.5S5.67 16 6.5 16H8v1.5c0 .83-.67 1.5-1.5 1.5"
                    /></svg
                >
                <p
                    class="text-lg rounded-md aspect-square border-2 border-gray-200/40 w-7 h-7 flex items-center justify-center"
                >
                    k
                </p>
            </span>
        </div>
    {:else}
        <Adder get_all_items={() => get_all_items()} />
    {/if}
    <button
        class="ml-auto mr-2 text-white font-mono flex p-4 items-center gap-2 hover:scale-110 duration-200"
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

{#if !adderActive}
    <span class="mt-20 w-2/3">
        <ol class="grid gap-6">
            {#each highlightedSearchItems as password, index}
                <Password {password} {query} />
            {/each}
        </ol>
    </span>
{/if}

<svelte:window on:keydown={keydown} />
