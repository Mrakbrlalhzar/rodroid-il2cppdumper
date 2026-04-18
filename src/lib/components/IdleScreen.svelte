<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { type } from "@tauri-apps/plugin-os";
  import { copyFile } from "@tauri-apps/plugin-fs";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { appState, currentScreen, config, binaryPath, metadataPath, binaryInfo, t } from "$lib/stores";
  import type { BinaryInfo } from "$lib/types";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import ConfigDialog from "./ConfigDialog.svelte";

  let showConfig = $state(false);

  async function pickBinary() {
    const osType = await type();
    const file = await open({
      multiple: false,
      ...(osType === "ios" ? {} : { filters: [{ name: "IL2CPP Binary", extensions: ["so", "dll", "exe", "dylib", "nso", "wasm", "*"] }] }),
    });
    if (file) {
      let finalPath = file;
      if (osType === "ios") {
        try {
          const appData = await appDataDir();
          const targetPath = await join(appData, "target_binary.bin");
          await copyFile(file, targetPath);
          finalPath = targetPath;
        } catch (e) {
          console.error("iOS copy error (binary):", e);
        }
      }

      binaryPath.set(finalPath);
      try {
        const info: BinaryInfo = await invoke("detect_binary", { path: finalPath });
        binaryInfo.set(info);
      } catch { binaryInfo.set(null); }
    }
  }

  async function pickMetadata() {
    const osType = await type();
    const file = await open({
      multiple: false,
      ...(osType === "ios" ? {} : { filters: [{ name: "Metadata", extensions: ["dat", "*"] }] }),
    });
    if (file) {
      let finalPath = file;
      if (osType === "ios") {
        try {
          const appData = await appDataDir();
          const targetPath = await join(appData, "target_metadata.dat");
          await copyFile(file, targetPath);
          finalPath = targetPath;
        } catch (e) {
          console.error("iOS copy error (metadata):", e);
        }
      }
      metadataPath.set(finalPath);
    }
  }

  function startDump() {
    appState.set("dumping");
    currentScreen.set("dumping");
  }
</script>

<div class="flex flex-col h-full p-4 gap-3 overflow-y-auto">
  <Card>
    <CardHeader class="pb-3">
      <div class="flex items-center gap-2">
        <div class="size-6 rounded-md bg-primary/10 flex items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-primary"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg>
        </div>
        <CardTitle class="text-xs font-semibold uppercase tracking-widest text-primary">{$t.label_binary}</CardTitle>
      </div>
    </CardHeader>
    <CardContent class="pt-0">
      <div class="flex items-center gap-2">
        <div class="flex-1 relative">
          <Input
            value={$binaryPath || ""}
            onchange={(e) => {
              const val = e.currentTarget.value;
              binaryPath.set(val);
              if (val) {
                invoke("detect_binary", { path: val })
                  .then((info: any) => binaryInfo.set(info))
                  .catch(() => binaryInfo.set(null));
              } else {
                binaryInfo.set(null);
              }
            }}
            placeholder="No file selected"
            class="pr-8 text-xs font-mono"
          />
          {#if $binaryPath}
            <div class="absolute right-3 top-1/2 -translate-y-1/2">
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-emerald-400"><polyline points="20 6 9 17 4 12"/></svg>
            </div>
          {/if}
        </div>
        <Button variant="outline" size="icon" class="shrink-0" onclick={pickBinary}>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><path d="M12 10v6"/><path d="m9 13 3-3 3 3"/></svg>
        </Button>
      </div>
      {#if $binaryInfo}
        <div class="flex flex-wrap gap-1.5 mt-3">
          <Badge variant="secondary" class="bg-primary/10 text-primary border-primary/20">{$binaryInfo.format}</Badge>
          {#if $binaryInfo.unity_version}
            <Badge variant="secondary" class="bg-emerald-500/10 text-emerald-400 border-emerald-500/20">Unity {$binaryInfo.unity_version}</Badge>
          {/if}
        </div>
      {/if}
    </CardContent>
  </Card>

  <Card>
    <CardHeader class="pb-3">
      <div class="flex items-center gap-2">
        <div class="size-6 rounded-md bg-primary/10 flex items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-primary"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/></svg>
        </div>
        <CardTitle class="text-xs font-semibold uppercase tracking-widest text-primary">{$t.label_metadata}</CardTitle>
      </div>
    </CardHeader>
    <CardContent class="pt-0">
      <div class="flex items-center gap-2">
        <div class="flex-1 relative">
          <Input
            value={$metadataPath || ""}
            onchange={(e) => metadataPath.set(e.currentTarget.value)}
            placeholder="No file selected"
            class="pr-8 text-xs font-mono"
          />
          {#if $metadataPath}
            <div class="absolute right-3 top-1/2 -translate-y-1/2">
              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-emerald-400"><polyline points="20 6 9 17 4 12"/></svg>
            </div>
          {/if}
        </div>
        <Button variant="outline" size="icon" class="shrink-0" onclick={pickMetadata}>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><path d="M12 10v6"/><path d="m9 13 3-3 3 3"/></svg>
        </Button>
      </div>
    </CardContent>
  </Card>

  <Button variant="outline" class="w-full justify-center gap-2" onclick={() => showConfig = true}>
    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
    {$t.dump_options}
  </Button>

  <div class="flex-1"></div>

  <Button
    size="lg"
    class="w-full py-5 text-sm font-semibold tracking-wide shrink-0"
    disabled={!$binaryPath || !$metadataPath}
    onclick={startDump}
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
    {$t.start_dump}
  </Button>
</div>

{#if showConfig}
  <ConfigDialog bind:config={$config} onclose={() => showConfig = false} />
{/if}
