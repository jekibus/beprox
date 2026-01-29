<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { ask } from "@tauri-apps/plugin-dialog";

  interface Site {
    id: string;
    domain: string;
    port: number;
    enabled: boolean;
  }

  interface Settings {
    default_tld: string;
  }

  let sites: Site[] = $state([]);
  let settings: Settings = $state({ default_tld: ".local" });
  let showSettings = $state(false);
  let newDomain = $state("");
  let newPort = $state(5173);
  let error = $state("");
  let loading = $state(false);

  async function loadSites() {
    try {
      sites = await invoke("get_sites");
    } catch (e) {
      error = String(e);
    }
  }

  async function loadSettings() {
    try {
        settings = await invoke("get_settings");
    } catch (e) {
        console.error("Failed to load settings", e);
    }
  }

  async function saveSettings(e: Event) {
    e.preventDefault();
    try {
        await invoke("save_settings", { settings });
        showSettings = false;
    } catch (e) {
        error = String(e);
    }
  }

  async function addSite(e: Event) {
    e.preventDefault();
    if (!newDomain) return;
    
    loading = true;
    error = "";
    
    // Simple validation
    if (!newDomain.includes(".")) {
        newDomain += settings.default_tld;
    }

    try {
      await invoke("create_site", { domain: newDomain, port: Number(newPort) });
      newDomain = "";
      // Keep the port as is or reset to default? The previous code reset to 3000.
      // Let's reset to a common default or keep last used?
      // Resetting to 5173 (Vite default) for now as per initial state.
      newPort = 5173; 
      await loadSites();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function deleteSite(id: string) {
    const yes = await ask("Are you sure you want to delete this site?", {
      title: "BeProx",
      kind: "warning",
    });
    
    if (!yes) return;
    
    try {
      await invoke("delete_site", { id });
      await loadSites();
    } catch (e) {
      error = String(e);
    }
  }

  async function toggleSite(id: string) {
    try {
      await invoke("toggle_site", { id });
      await loadSites();
    } catch (e) {
      error = String(e);
    }
  }

  onMount(() => {
    loadSites();
    loadSettings();
  });
</script>

<div class="container mx-auto px-6 py-12 max-w-6xl">
  <!-- Header -->
  <header class="mb-12 flex flex-col md:flex-row justify-between items-center gap-6">
    <div class="flex items-center gap-4">
      <div class="bg-indigo-500/20 p-3 rounded-2xl border border-indigo-500/30">
        <!-- Logo Icon -->
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-indigo-400"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.58-1.25.5-2.25.5-2.5 0-.5-1.5-2-1.5-2 .5-1.25.5-2.25.5-2.5-2 0-3.5 1.5-3.5 1.5a5.5 5.5 0 0 0-3.5 0c0 0-1.5-1.5-3.5-1.5-.25 0-.25 1.25.25 2.5 0 0-1.5 1.5-1.5 2 0 .25-.08 1.25.5 2.5C6.5 16.5 9.5 18.5 12.5 18.5c-1 1.5-1 3.5-1 3.5V22"/><path d="M9 18c-4.51 2-5-2-7-2"/></svg>
      </div>
      <div>
        <h1 class="text-4xl font-bold tracking-tight bg-clip-text text-transparent bg-linear-to-r from-white to-slate-400">BeProx</h1>
        <p class="text-slate-400 font-medium">Local Domain Manager</p>
      </div>
    </div>
    
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-3 bg-slate-800/50 backdrop-blur-sm border border-slate-700/50 px-4 py-2 rounded-full shadow-lg">
        <div class="relative flex h-3 w-3">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
        </div>
        <span class="text-sm font-medium text-slate-300">Proxy Active: Port 80</span>
      </div>
      
      <button 
        onclick={() => showSettings = true}
        class="bg-slate-800/50 hover:bg-slate-700/50 backdrop-blur-sm border border-slate-700/50 p-2 rounded-full shadow-lg text-slate-400 hover:text-white transition-all"
        aria-label="Settings"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
      </button>
    </div>
  </header>

  {#if error}
    <div class="mb-8 animate-in fade-in slide-in-from-top-4">
      <div class="bg-red-500/10 border border-red-500/20 text-red-400 px-6 py-4 rounded-xl flex items-center gap-3 shadow-lg">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
        <div>
          <strong class="font-semibold block">Something went wrong</strong>
          <span class="text-sm opacity-90">{error}</span>
        </div>
        <button onclick={() => error = ""} class="ml-auto hover:bg-red-500/10 p-1 rounded-lg transition-colors" aria-label="Dismiss error">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>
    </div>
  {/if}

  <div class="space-y-8">
    <!-- Add Site Form (Compact) -->
    <div class="bg-slate-800/50 backdrop-blur-xl border border-slate-700/50 rounded-2xl p-6 shadow-xl">
        <div class="flex items-center gap-3 mb-4">
            <div class="p-2 bg-indigo-500/10 rounded-lg text-indigo-400">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>
            </div>
            <h2 class="text-xl font-semibold text-white">Add New Site</h2>
        </div>

        <form onsubmit={addSite} class="flex flex-col md:flex-row gap-4 items-end">
          <div class="flex-1 w-full">
            <label for="domain" class="block text-sm font-medium text-slate-400 mb-2">Domain Name</label>
            <div class="relative group">
                <input
                  id="domain"
                  type="text"
                  bind:value={newDomain}
                  placeholder="myapp"
                  class="w-full bg-slate-900/50 border border-slate-700 text-white rounded-xl px-4 py-3 pl-11 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-slate-600 group-hover:border-slate-600"
                  required
                />
                <div class="absolute left-4 top-3.5 text-slate-500 pointer-events-none">
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
                </div>
                <div class="absolute right-4 top-3.5 text-slate-500 pointer-events-none text-xs font-mono">
                    {newDomain.includes(".") ? "" : settings.default_tld}
                </div>
            </div>
          </div>

          <div class="w-full md:w-48">
            <label for="port" class="block text-sm font-medium text-slate-400 mb-2">Target Port</label>
            <div class="relative group">
                <input
                  id="port"
                  type="number"
                  bind:value={newPort}
                  class="w-full bg-slate-900/50 border border-slate-700 text-white rounded-xl px-4 py-3 pl-11 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all placeholder:text-slate-600 group-hover:border-slate-600"
                  required
                />
                <div class="absolute left-4 top-3.5 text-slate-500 pointer-events-none">
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v20"/><path d="M2 12h20"/><path d="m2 2 20 20"/><path d="m20 2-20 20"/></svg>
                </div>
            </div>
          </div>

          <button
            type="submit"
            disabled={loading}
            class="w-full md:w-auto bg-indigo-600 hover:bg-indigo-500 text-white font-medium px-8 py-3 rounded-xl shadow-lg shadow-indigo-500/20 transition-all active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed flex justify-center items-center gap-2 whitespace-nowrap"
          >
            {#if loading}
                <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span>Adding...</span>
            {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                <span>Add Site</span>
            {/if}
          </button>
        </form>
    </div>

    <!-- Sites List (Table) -->
    <div class="bg-slate-800/30 backdrop-blur-md border border-slate-700/50 rounded-2xl overflow-hidden shadow-xl">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-sm">
          <thead>
            <tr class="border-b border-slate-700/50 text-slate-400 bg-slate-800/50">
              <th class="px-6 py-4 font-medium">Status</th>
              <th class="px-6 py-4 font-medium">Domain</th>
              <th class="px-6 py-4 font-medium">Target</th>
              <th class="px-6 py-4 font-medium text-right">Actions</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-700/50">
            {#each sites as site (site.id)}
              <tr class="group hover:bg-slate-800/50 transition-colors">
                <td class="px-6 py-4 whitespace-nowrap">
                   <button 
                        onclick={() => toggleSite(site.id)}
                        class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 {site.enabled ? 'bg-emerald-500' : 'bg-slate-700'}"
                        title={site.enabled ? "Disable site" : "Enable site"}
                    >
                        <span class="sr-only">Toggle site</span>
                        <span class="{site.enabled ? 'translate-x-6' : 'translate-x-1'} inline-block h-4 w-4 transform rounded-full bg-white transition-transform"></span>
                    </button>
                </td>
                <td class="px-6 py-4">
                  <div class="flex items-center gap-2">
                    <span class="font-medium text-white text-base">{site.domain}</span>
                    <a 
                        href="http://{site.domain}" 
                        target="_blank" 
                        class="text-slate-400 hover:text-indigo-400 transition-colors p-1 opacity-0 group-hover:opacity-100"
                        title="Open in browser"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
                    </a>
                  </div>
                </td>
                <td class="px-6 py-4 text-slate-400 font-mono">
                  localhost:{site.port}
                </td>
                <td class="px-6 py-4 text-right">
                    <button
                        onclick={() => deleteSite(site.id)}
                        class="text-slate-500 hover:text-red-400 transition-colors p-2 hover:bg-red-500/10 rounded-lg inline-flex"
                        title="Delete site"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>
                    </button>
                </td>
              </tr>
            {:else}
              <tr>
                <td colspan="4" class="px-6 py-16 text-center text-slate-500">
                    <div class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-slate-800/50 mb-3">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>
                    </div>
                    <p>No sites configured yet</p>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

{#if showSettings}
<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4 animate-in fade-in duration-200">
    <div class="bg-slate-900 border border-slate-700 rounded-2xl shadow-2xl w-full max-w-md overflow-hidden animate-in zoom-in-95 duration-200">
        <div class="p-6 border-b border-slate-800">
            <h3 class="text-xl font-semibold text-white">Settings</h3>
        </div>
        
        <form onsubmit={saveSettings} class="p-6 space-y-6">
            <div>
                <label for="default_tld" class="block text-sm font-medium text-slate-400 mb-2">Default Top-Level Domain</label>
                <div class="relative">
                    <input
                        id="default_tld"
                        type="text"
                        bind:value={settings.default_tld}
                        placeholder=".local"
                        class="w-full bg-slate-950 border border-slate-800 text-white rounded-xl px-4 py-3 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all"
                        required
                    />
                    <div class="absolute right-4 top-3.5 text-slate-600 pointer-events-none text-xs">
                        Default: .local
                    </div>
                </div>
                <p class="mt-2 text-xs text-slate-500">
                    This suffix will be automatically appended if you don't provide one when adding a new site.
                </p>
            </div>

            <div class="flex justify-end gap-3 pt-2">
                <button
                    type="button"
                    onclick={() => showSettings = false}
                    class="px-4 py-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition-colors"
                >
                    Cancel
                </button>
                <button
                    type="submit"
                    class="bg-indigo-600 hover:bg-indigo-500 text-white font-medium px-6 py-2 rounded-lg shadow-lg shadow-indigo-500/20 transition-all active:scale-[0.98]"
                >
                    Save Changes
                </button>
            </div>
        </form>
    </div>
</div>
{/if}
