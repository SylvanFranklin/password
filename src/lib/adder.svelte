<script lang="ts">
    export let get_all_items: Function;
    import { invoke } from "@tauri-apps/api/tauri";
    import { scale } from "svelte/transition";
    let appname = "";
    let username = "";
    let password = "";
    let generatedPassword = "";
    let passwordLength = 20;
    let passwordStrength = "";
    let error = "";
    let showingPasswordGen = false;

    async function get_password_strength() {
        let scale: number = await invoke("password_strength", {
            password: password,
        });
        console.log(scale);
        switch (scale) {
            case 0:
                passwordStrength = "border-2 border-red-500";
                break;
            case 1:
                passwordStrength = "border-2 border-orange-500";
                break;
            case 2:
                passwordStrength = "border-2 border-yellow-500";
                break;
            case 3:
                passwordStrength = "border-2 border-green-500";
                break;
            case 3:
                passwordStrength = "border-2 border-emerald-700";
                break;
        }
    }

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
    class="flex flex-row p-2 w-full font-mono text-gray-200 rounded-lg shadow-lg bg-slate-200/5"
>
    <ol class="flex flex-1 gap-2 mx-auto">
        <!-- svelte-ignore a11y-autofocus -->
        <li
            class={`flex flex-row bg-slate-200/10 p-1 rounded-md shadow-sm ${
                fieldFocus == 0 ? "w-full" : "w-2/5"
            }`}
        >
            <input
                type="text"
                use:focus
                class="flex p-2 w-full outline-none bg-slate-200/0"
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
                class="flex p-2 w-full outline-none bg-slate-200/0"
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
            } ${passwordStrength}
            `}
        >
            <input
                type="text"
                on:focus={() => (fieldFocus = 2)}
                on:input={() => get_password_strength()}
                class="flex p-2 w-full outline-none bg-slate-200/0"
                placeholder="password"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={password}
            />

            {#if fieldFocus == 2}
                <button
                    class="flex items-center -my-2 text-white"
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
            class="py-2 px-4 mx-auto w-min bg-orange-400 rounded-md shadow-md duration-100 hover:scale-95"
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
        class="flex absolute left-1/2 flex-col gap-3 p-2 mx-auto mt-4 w-2/3 font-mono text-white rounded-md transform -translate-x-1/2 lg:w-1/2 bg-slate-200/10"
        transition:scale={{ duration: 200, delay: 0, start: 0.5 }}
    >
        <h1 class="mx-auto">password generator</h1>

        <span class="flex flex-row gap-3 items-center w-full">
            length
            <input
                type="number"
                inputmode="numeric"
                use:focus
                class="flex p-2 w-1/4 rounded-md outline-none bg-slate-200/10"
                placeholder="20"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={passwordLength}
            />
            <input
                class="flex p-2 w-3/4 rounded-md outline-none bg-slate-200/10"
                placeholder=""
                autoCapitalize="off"
                bind:value={generatedPassword}
            />
        </span>

        <span class="flex flex-row gap-3 justify-center w-full">
            <button
                class="p-3 rounded-md shadow-sm hover:scale-95 bg-pink-400/80 duration:100"
                on:click={() => {
                    gen_password();
                }}
            >
                new
            </button>
            <button
                class="p-3 rounded-md shadow-sm hover:scale-95 bg-green-500/80 duration:100"
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
