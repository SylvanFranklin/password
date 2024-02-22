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
        class="text-5xl font-bold font-mono p-4 bg-blue-400 text-white rounded-lg shadow-lg"
    >
        PASSWORD MANAGER
    </h1>
    {#each passwords as password}
        <Password {password} />
    {/each}
</main>
