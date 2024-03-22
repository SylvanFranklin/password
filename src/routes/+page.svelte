<script lang="ts">
    interface PasswordObject {
        appname: string;
        username: string;
        password: string;
    }
    import Password from "$lib/password.svelte";
    import Adder from "$lib/adder.svelte";
    import Fuse from "fuse.js";
    import { invoke, type InvokeArgs } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { tweened } from "svelte/motion";
    import { slide } from "svelte/transition";
    export let lock: Function;
    let passwords: Array<PasswordObject> = [];
    let highlightedSearchItems: Array<PasswordObject> = [];
    let query = "";
    let adderActive = false;

    let slider = tweened(100, { duration: 200 });

    let fuser: Fuse<PasswordObject>;

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

    onMount(async () => {
        await get_all_items();
        highlightedSearchItems = [...passwords];
        fuser = new Fuse(passwords, {
            keys: ["appname", "username"],
        });
    });

    const delete_item = async (password: PasswordObject) => {
        await invoke("delete_item", { appname: password.appname });
        await get_all_items();
    };

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
        fuser = new Fuse(passwords, {
            keys: ["appname", "username"],
        });
    }
</script>

<nav
    class="flex top-0 fixed flex-row items-center px-2 pt-4 pb-2 w-full bg-[#060607] z-20"
>
    <button
        class="flex gap-2 items-center p-4 font-mono text-white duration-200 hover:scale-110"
        on:click={() => {
            adderActive = !adderActive;
            slider.set(10);
        }}
    >
        <span class="text-white">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="36"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M9.5 16q-2.725 0-4.612-1.888T3 9.5q0-2.725 1.888-4.612T9.5 3q2.725 0 4.613 1.888T16 9.5q0 1.1-.35 2.075T14.7 13.3l5.6 5.6q.275.275.275.7t-.275.7q-.275.275-.7.275t-.7-.275l-5.6-5.6q-.75.6-1.725.95T9.5 16m0-2q1.875 0 3.188-1.312T14 9.5q0-1.875-1.312-3.187T9.5 5Q7.625 5 6.313 6.313T5 9.5q0 1.875 1.313 3.188T9.5 14"
                /></svg
            >
        </span>
    </button>

    <div class="flex flex-row justify-center items-center w-full">
        {#if !adderActive}
            <input
                type="text"
                class={`bg-slate-200/5 p-4 rounded-lg outline-none h-12 placeholder-gray-200/20 overflow-x-scroll text-gray-200 text-xl font-mono flex w-3/4`}
                placeholder="search passwords"
                transition:slide={{ duration: 400, delay: 0, axis: "x" }}
                bind:value={query}
                use:focus
                autocorrect="off"
                autocapitalize="off"
                spellcheck="false"
                on:input|preventDefault={formChange}
            />
        {:else}
            <span transition:slide={{ duration: 400, delay: 0, axis: "x" }}>
                <Adder get_all_items={() => get_all_items()} />
            </span>
        {/if}
    </div>
    <button
        class="flex gap-2 items-center p-4 mr-2 ml-auto font-mono text-white duration-200 hover:scale-110"
        on:click={() => {
            adderActive = !adderActive;
        }}
    >
        <span class="text-white">
            <!-- adder, plus button, etc -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M7.5 3c2 0 3.6 1.2 4.2 3H21v3h-3v3h-3V9h-3.3c-.6 1.8-2.3 3-4.2 3C5 12 3 10 3 7.5S5 3 7.5 3m0 3C6.7 6 6 6.7 6 7.5S6.7 9 7.5 9S9 8.3 9 7.5S8.3 6 7.5 6M8 17h3v-3h2v3h3v2h-3v3h-2v-3H8z"
                /></svg
            >
        </span>
    </button>
</nav>

<span class="mt-44 w-full sm:w-4/5">
    <ol class="grid gap-6">
        {#each highlightedSearchItems as password, index}
            <Password
                {password}
                {query}
                delete_item={() => delete_item(password)}
            />
        {/each}
    </ol>
</span>

<svelte:window on:keydown={keydown} />
