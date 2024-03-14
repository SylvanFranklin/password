<script lang="ts">
    export let get_all_items: Function;
    import { invoke } from "@tauri-apps/api/tauri";
    import { scale } from "svelte/transition";
    let appname = "";
    let username = "";
    let password = "";
    let generatedPassword = "";
    let passwordLength = 20;
    let error = "";
    let showingPasswordGen = false;

    function focus(eml: HTMLElement) {
        eml.focus();
    }

    let fieldFocus = 0;

    const add_password = async () => {
        await invoke("validate_entry", {
            entry: { appname, username, password },
        }).then((err) => {
            if ((err as string).length > 0) {
                error = err as string;
            } else {
                err = "";
                invoke("write_to_file", {
                    entry: {
                        appname,
                        username,
                        password,
                    },
                });
                get_all_items();
                appname = "";
                username = "";
                password = "";
                showingPasswordGen = false;
            }
        });
    };

    const gen_password = () => {
        invoke("generate_password", { length: passwordLength }).then((res) => {
            generatedPassword = res as string;
        });
    };
</script>

<form
    class="bg-slate-200/5 p-2 shadow-lg text-gray-200 rounded-lg font-mono flex flex-row w-full"
>
    <ol class="gap-2 flex flex-1 mx-auto">
        <!-- svelte-ignore a11y-autofocus -->
        <li
            class={`flex flex-row bg-slate-200/10 p-1 rounded-md shadow-sm ${
                fieldFocus == 0 ? "w-full" : "w-2/5"
            }`}
        >
            <input
                type="text"
                use:focus
                class="outline-none bg-slate-200/0 p-2 flex w-full"
                placeholder="app name"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={appname}
                on:focus={() => (fieldFocus = 0)}
            />
        </li>

        <li
            class={`flex flex-row bg-slate-200/10 p-1 rounded-md shadow-sm ${
                fieldFocus == 1 ? "w-full" : "w-2/5"
            }`}
        >
            <input
                type="text"
                class="outline-none bg-slate-200/0 p-2 flex w-full"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                on:focus={() => (fieldFocus = 1)}
                placeholder="username"
                bind:value={username}
            />
        </li>

        <li
            class={`flex flex-row bg-slate-200/10 p-1 rounded-md shadow-sm ${
                fieldFocus == 2 ? "w-full" : "w-2/5"
            }`}
        >
            <input
                type="text"
                on:focus={() => (fieldFocus = 2)}
                class="outline-none bg-slate-200/0 p-2 flex w-full"
                placeholder="password"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={password}
            />

            {#if fieldFocus == 2}
                <button
                    class="text-white flex -my-2 items-center"
                    on:click|stopPropagation={() => {
                        showingPasswordGen = !showingPasswordGen;
                    }}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="32"
                        height="32"
                        viewBox="0 0 24 24"
                        ><path
                            fill="currentColor"
                            d="M16.59 8.59L12 13.17L7.41 8.59L6 10l6 6l6-6z"
                        /></svg
                    >
                </button>
            {/if}
        </li>
        <button
            class="px-4 py-2 rounded-md bg-orange-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
            on:click={() => {
                add_password();
            }}>add</button
        >
    </ol>
</form>

{#if error.length > 0}
    <h1 class="font-mono text-red-500">{error}</h1>
{/if}

{#if showingPasswordGen}
    <div
        class="absolute left-1/2 transform -translate-x-1/2 w-2/3 mx-auto mt-4 lg:w-1/2 rounded-md bg-slate-200/10 font-mono text-white flex flex-col p-2 gap-3"
        transition:scale={{ duration: 200, delay: 0, start: 0.5 }}
    >
        <h1 class="mx-auto">password generator</h1>

        <span class="flex flex-row w-full items-center gap-3">
            length
            <input
                type="number"
                inputmode="numeric"
                use:focus
                class="outline-none bg-slate-200/10 p-2 flex w-1/4 rounded-md"
                placeholder="20"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={passwordLength}
            />
            <input
                class="outline-none bg-slate-200/10 p-2 flex w-3/4 rounded-md"
                placeholder=""
                autoCapitalize="off"
                bind:value={generatedPassword}
            />
        </span>

        <span class="w-full flex flex-row gap-3 justify-center">
            <button
                class="p-3 rounded-md bg-pink-400/80 shadow-sm hover:scale-95 duration:100"
                on:click={() => {
                    gen_password();
                }}
            >
                new
            </button>
            <button
                class="p-3 rounded-md bg-green-500/80 shadow-sm hover:scale-95 duration:100"
                on:click={() => {
                    password = generatedPassword;
                    showingPasswordGen = false;
                }}
            >
                done
            </button>
        </span>
    </div>
{/if}
