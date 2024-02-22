<script lang="ts">
    export let password: {
        username: string;
        password: string;
        website: string;
    };
    // thought that index, might be useful in numbering ??
    export let index: number;
</script>

<div
    class="flex bg-slate-600/10 shadow-lg w-full flex-col text-gray-200 p-4 gap-2 rounded-lg font-mono"
>
    <div class="flex flex-row">
        <p class="text-xl font-bold w-32">Username</p>
        <h1
            class="text-center rounded-md p-2 bg-orange-400 w-24relative text-gray-200 ml-auto relative bottom-8 left-8"
        >
            {password.website}
        </h1>
    </div>
    <span class="rounded-md p-2 bg-slate-200/10 flex flex-row">
        {password.username}
        <button class="ml-auto">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="26"
                height="26"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M9 18q-.825 0-1.412-.587T7 16V4q0-.825.588-1.412T9 2h9q.825 0 1.413.588T20 4v12q0 .825-.587 1.413T18 18zm-4 4q-.825 0-1.412-.587T3 20V6h2v14h11v2z"
                /></svg
            >
        </button>
    </span>

    <p class="text-xl font-bold">Password</p>
    <span class="rounded-md p-2 bg-slate-200/10 flex flex-row">
        {password.password}
        <button class="ml-auto">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="26"
                height="26"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M9 18q-.825 0-1.412-.587T7 16V4q0-.825.588-1.412T9 2h9q.825 0 1.413.588T20 4v12q0 .825-.587 1.413T18 18zm-4 4q-.825 0-1.412-.587T3 20V6h2v14h11v2z"
                /></svg
            >
        </button>
    </span>
</div>
