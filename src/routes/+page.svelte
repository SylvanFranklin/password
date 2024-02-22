<script lang="ts">
    import Password from "$lib/password.svelte";

    interface PasswordObject {
        username: string;
        password: string;
    }

    const passwords: PasswordObject[] = [
        {
            username: "test",
            password: "test",
        },
        {
            username: "test2",
            password: "test2",
        },
    ];
</script>

<main class="flex flex-col w-full h-full mt-20 items-center">
    <h1
        class="text-3xl font-bold font-mono p-2 bg-blue-400 text-white rounded-lg shadow-lg"
    >
        PASSWORD MANAGER
    </h1>

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
    placeholder="username"
    />
    <input
    type="text"
    class="bg-slate-200/10 p-2 rounded-lg outline-none"
    placeholder="password"
    />
    <button
    class="px-4 py-2 rounded-md bg-orange-400 w-min mx-auto hover:scale-95 duration-100 shadow-md"
    type="submit">add</button
    >
    </span>
    </form>

    <ol class="grid w-2/3 mt-20 gap-4">
        {#each passwords as password, index}
            <Password {password} index={index + 1} />
        {/each}
    </ol>
</main>
