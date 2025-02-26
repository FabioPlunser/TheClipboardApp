<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow, PhysicalPosition } from "@tauri-apps/api/window";
  import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";

  listen("show_clipboard", async (event) => {
    console.log("event happened");
    console.log("show_clipboard", event.payload);
    let x = event.payload.x;
    let y = event.payload.y;
    const window = getCurrentWindow();
    await window.setPosition(new PhysicalPosition(x, y));
  });

  let tests = $state([{ name: "test1" }, { name: "test2" }, { name: "test3" }]);
  let search = $state("");

  let filteredTests = $derived.by(() => {
    return tests
      .map((test) => ({
        ...test,
        match: test.name.toLowerCase().includes(search.toLowerCase()),
      }))
      .sort((a, b) => {
        if (a.match && !b.match) return -1; // Move matching items to the top
        if (!a.match && b.match) return 1;
        return 0; // Keep the original order for non-matching items
      });
  });

  async function handleEntry(test) {
    // const window = getCurrentWindow();
    // await window.hide();

    await copyTextToClipboard(test.name);
  }
  $inspect(filteredTests);

  async function copyTextToClipboard(text: string) {
    await writeText(text);
  }
</script>

<div class="h-screen w-full">
  <div class="w-full">
    <label
      class="input w-full bg-gray-300/30 border-0 outline-0 focus-within:outline-none rounded-b-none"
    >
      <svg
        class="h-[1em] opacity-50"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        ><g
          stroke-linejoin="round"
          stroke-linecap="round"
          stroke-width="2.5"
          fill="none"
          stroke="currentColor"
          ><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"
          ></path></g
        ></svg
      >
      <input
        bind:value={search}
        type="search"
        class="grow bg-transparent text-white font-bold outline-hidden focus:outline-hidden rounded-b-none"
        placeholder="Search"
      />
    </label>
    <div class="pt-2">
      <ul class="w-full text-white">
        {#each filteredTests as test}
          <li class="px-2 w-full py-1">
            <button
              class="hover:bg-gray-500/30 bg-gray-800/20 p-2 rounded-md w-full text-left cursor-pointer transform transition duration-200 active:scale-90"
              onclick={async () => handleEntry(test)}>{test.name}</button
            >
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>
