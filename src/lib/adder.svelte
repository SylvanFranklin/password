<script lang="ts">
    export let get_all_items: Function;
    import { invoke } from "@tauri-apps/api/tauri";
    let appname = "";
    let username = "";
    let password = "";
    let error = "";

    let passwordFocus = false;

    const add_password = async () => {
        await invoke("validate_entry", {
            entry: { appname, username, password },
        }).then((err) => {
            if ((err as string).length > 0) {
                error = err as string;
            } else {
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
            }
        });
    };

    const gen_password = () => {
        invoke("generate_password", { length: 20 }).then((res) => {
            password = res as string;
        });
    };
</script>

<form
    class="bg-slate-200/5 p-2 flex flex-row shadow-lg text-gray-200 gap-2 rounded-lg font-mono"
>
    <!-- <h1 class="font-mono text-xl text-white mx-auto outline-none"> -->
    <!--     Add Password -->
    <!-- </h1> -->

    <span class="gap-3 flex flex-row mx-auto">
        <!-- svelte-ignore a11y-autofocus -->
        <input
            type="text"
            autofocus
            class="bg-slate-200/10 p-2 rounded-lg outline-none resizer"
            placeholder="app name"
            autoCapitalize="off"
            spellCheck="false"
            autoCorrect="off"
            bind:value={appname}
            on:focus={() => (passwordFocus = false)}
        />
        <input
            type="text"
            class="bg-slate-200/10 p-2 rounded-lg outline-none resizer"
            autoCapitalize="off"
            spellCheck="false"
            autoCorrect="off"
            on:focus={() => (passwordFocus = false)}
            placeholder="username"
            bind:value={username}
        />

        <span class="resizer p-2 flex bg-slate-200/10 rounded-lg overflow-clip">
            <input
                type="text"
                on:focus={() => (passwordFocus = true)}
                class="outline-none bg-slate-200/0 w-full"
                placeholder="password"
                autoCapitalize="off"
                spellCheck="false"
                autoCorrect="off"
                bind:value={password}
            />

            {#if passwordFocus}
                <button class="text-white flex -my-2 items-center">
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
        </span>

        <button
            class="px-4 py-2 rounded-md bg-orange-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
            on:click={() => {
                add_password();
            }}>add</button
        >
    </span>
</form>

{#if error.length > 0}
    <h1 class="font-mono text-red-500">{error}</h1>
{/if}
