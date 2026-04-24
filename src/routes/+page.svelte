<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { downloadDir, documentDir, join } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount, onDestroy } from "svelte";
  import {
    appState, currentScreen, config, logs, binaryPath, metadataPath,
    binaryInfo, outputPath, errorMessage, inputRequest, elapsedSeconds,
    themeMode, applyTheme, t, crashLog, resetAll, outputDir, defaultOutputDir
  } from "$lib/stores";
  import type { DumperConfig, DumpCompleteEvent, InputRequestEvent } from "$lib/types";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import IdleScreen from "$lib/components/IdleScreen.svelte";
  import DumpingScreen from "$lib/components/DumpingScreen.svelte";
  import ResultScreen from "$lib/components/ResultScreen.svelte";
  import ErrorScreen from "$lib/components/ErrorScreen.svelte";
  import SettingsScreen from "$lib/components/SettingsScreen.svelte";
  import AboutScreen from "$lib/components/AboutScreen.svelte";
  import SplashScreen from "$lib/components/SplashScreen.svelte";
  import CrashScreen from "$lib/components/CrashScreen.svelte";
  import InputDialog from "$lib/components/InputDialog.svelte";

  let unlistenLog: (() => void) | null = null;
  let unlistenComplete: (() => void) | null = null;
  let unlistenInput: (() => void) | null = null;
  let unlistenCrash: (() => void) | null = null;
  let currentOs = $state("Desktop");

  onMount(async () => {
    try {
      const osType = await type();
      let baseDir = "";

      if (osType === "ios") {
        currentOs = "iOS";
        baseDir = await documentDir();
      } else if (osType === "android") {
        currentOs = "Android";
        baseDir = await documentDir();
      } else {
        currentOs = "Desktop";
        baseDir = await downloadDir();
      }

      const defaultPath = await join(baseDir, "IL2CppDumper");
      defaultOutputDir.set(defaultPath);
      let currentOutDir = "IL2CppDumper";
      outputDir.subscribe(v => currentOutDir = v)();
      if (currentOutDir === "IL2CppDumper") {
        outputDir.set(defaultPath);
      }
    } catch (e) {
      // Ignore if API fails
    }

    applyTheme($themeMode);

    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    mq.addEventListener("change", () => { if ($themeMode === "system") applyTheme("system"); });

    unlistenLog = await listen<{ message: string }>("dump-log", (event) => {
      logs.update(l => [...l, event.payload.message]);
    });

    unlistenComplete = await listen<DumpCompleteEvent>("dump-complete", (event) => {
      if (event.payload.success) {
        outputPath.set(event.payload.output_path);
        appState.set("result");
        currentScreen.set("result");
      } else {
        errorMessage.set(event.payload.error_message);
        appState.set("error");
        currentScreen.set("error");
      }
    });

    unlistenInput = await listen<InputRequestEvent>("dump-input-request", (event) => {
      inputRequest.set(event.payload.prompt_type);
    });

    unlistenCrash = await listen<{ crash_log: string }>("dump-crash", (event) => {
      crashLog.set(event.payload.crash_log);
      currentScreen.set("crash");
    });
  });

  onDestroy(() => {
    unlistenLog?.();
    unlistenComplete?.();
    unlistenInput?.();
    unlistenCrash?.();
  });

  let dumpStarted = false;

  $effect(() => {
    if ($currentScreen === "dumping" && !dumpStarted) {
      handleStartDump();
    }
  });

  async function handleStartDump() {
    if (dumpStarted) return;
    dumpStarted = true;

    const bp = $binaryPath;
    const mp = $metadataPath;
    let cfg: DumperConfig = {} as DumperConfig;
    config.subscribe(c => cfg = c)();

    let outDir = "IL2CppDumper";
    outputDir.subscribe(v => outDir = v)();

    logs.set([]);
    elapsedSeconds.set(0);

    try {
      await invoke("start_dump", {
        binaryPath: bp, metadataPath: mp,
        outputDir: outDir, configJson: JSON.stringify(cfg),
      });
    } catch (e) {
      errorMessage.set(String(e));
      appState.set("error");
      currentScreen.set("error");
    }
    dumpStarted = false;
  }

  function handleSplashFinished() {
    currentScreen.set("idle");
  }

  function handleCrashRestart() {
    crashLog.set("");
    resetAll();
  }
</script>

<svelte:head>
  <title>Rodroid IL2CPP Dumper</title>
</svelte:head>

{#if $currentScreen === "splash"}
  <SplashScreen onfinished={handleSplashFinished} />
{:else if $currentScreen === "crash"}
  <main class="h-[100dvh] flex flex-col overflow-hidden pb-[env(safe-area-inset-bottom)]">
    <CrashScreen crashLog={$crashLog} onrestart={handleCrashRestart} />
  </main>
{:else}
  <main class="h-[100dvh] flex flex-col overflow-hidden pb-[env(safe-area-inset-bottom)]">
    {#if $currentScreen !== "settings" && $currentScreen !== "about"}
      <header class="shrink-0 px-5 py-3 border-b border-border" data-tauri-drag-region>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="size-9 rounded-lg bg-primary/10 flex items-center justify-center ring-1 ring-primary/20">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="text-primary"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
            </div>
            <div>
              <h1 class="text-sm font-semibold text-foreground tracking-tight">{$t.app_name}</h1>
              <p class="text-[10px] text-muted-foreground">v4.0 {currentOs}</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            {#if $currentScreen === "dumping"}
              <Badge variant="outline" class="gap-1.5 border-primary/30 text-primary bg-primary/5">
                <span class="size-1.5 rounded-full bg-primary animate-pulse"></span>
                {$t.status_processing}
              </Badge>
            {:else if $currentScreen === "result"}
              <Badge variant="outline" class="gap-1.5 border-emerald-500/30 text-emerald-400 bg-emerald-500/5">
                <span class="size-1.5 rounded-full bg-emerald-400"></span>
                {$t.dump_complete}
              </Badge>
            {:else if $currentScreen === "error"}
              <Badge variant="outline" class="gap-1.5 border-destructive/30 text-destructive bg-destructive/5">
                <span class="size-1.5 rounded-full bg-destructive"></span>
                {$t.dump_failed}
              </Badge>
            {/if}
            {#if $currentScreen === "idle"}
              <Button variant="ghost" size="icon" onclick={() => currentScreen.set("settings")}>
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
              </Button>
            {/if}
          </div>
        </div>
      </header>
    {/if}

    <div class="flex-1 min-h-0">
      {#if $currentScreen === "idle"}
        <IdleScreen />
      {:else if $currentScreen === "dumping"}
        <DumpingScreen />
      {:else if $currentScreen === "result"}
        <ResultScreen />
      {:else if $currentScreen === "error"}
        <ErrorScreen />
      {:else if $currentScreen === "settings"}
        <SettingsScreen />
      {:else if $currentScreen === "about"}
        <AboutScreen />
      {/if}
    </div>
  </main>

  <InputDialog />
{/if}
