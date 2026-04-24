<script lang="ts">
  import type { DumperConfig } from "$lib/types";
  import { t } from "$lib/stores";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Switch } from "$lib/components/ui/switch/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";

  let { config = $bindable(), onclose }: { config: DumperConfig; onclose: () => void } = $props();
</script>

<Dialog.Root open={true} onOpenChange={(v) => { if (!v) onclose(); }}>
  <Dialog.Content class="sm:max-w-lg">
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-2.5">
        <div class="size-8 rounded-lg bg-primary/10 flex items-center justify-center ring-1 ring-primary/20">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-primary"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
        </div>
        {$t.dump_options}
      </Dialog.Title>
      <Dialog.Description>Configure what gets included in the dump output.</Dialog.Description>
    </Dialog.Header>

    <div class="space-y-5">
        <section>
          <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-primary mb-3">{$t.label_output}</p>
          <div class="space-y-3">
            {#each [
              { label: $t.setting_dump_method, key: "dumpMethod" },
              { label: $t.setting_dump_field, key: "dumpField" },
              { label: $t.setting_dump_property, key: "dumpProperty" },
              { label: $t.setting_dump_attribute, key: "dumpAttribute" },
              { label: $t.setting_dump_method_offset, key: "dumpMethodOffset" },
              { label: $t.setting_dump_field_offset, key: "dumpFieldOffset" },
              { label: $t.setting_dump_typedef_index, key: "dumpTypeDefIndex" },
              { label: $t.setting_dump_assembly_name, key: "dumpAssemblyName" },
              { label: $t.setting_split_dump_per_type, key: "splitDumpPerType" },
            ] as item}
              <div class="flex items-center justify-between">
                <Label class="text-sm font-normal cursor-pointer">{item.label}</Label>
                <Switch checked={config[item.key]} onCheckedChange={(v) => config[item.key] = v} />
              </div>
            {/each}
          </div>
        </section>

        <Separator />

        <section>
          <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-primary mb-3">{$t.label_generation}</p>
          <div class="space-y-3">
            {#each [
              { label: $t.setting_generate_struct, key: "generateStruct" },
              { label: $t.setting_generate_dummy_dll, key: "generateDummyDll" },
              { label: $t.setting_dummy_dll_add_token, key: "dummyDllAddToken" },
              { label: $t.setting_generate_generics_dump, key: "generateGenericsDump" },
            ] as item}
              <div class="flex items-center justify-between">
                <Label class="text-sm font-normal cursor-pointer">{item.label}</Label>
                <Switch checked={config[item.key]} onCheckedChange={(v) => config[item.key] = v} />
              </div>
            {/each}
          </div>
        </section>

        <Separator />

        <section>
          <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-primary mb-3">{$t.label_cpp_headers}</p>
          <div class="space-y-3">
            {#each [
              { label: $t.setting_generate_cpp_scaffold, key: "generateCppScaffold" },
              { label: $t.setting_mangle_names, key: "mangleNames" },
              { label: $t.setting_enhanced_ida_metadata, key: "enhancedIdaMetadata" },
              { label: $t.setting_generate_unity_headers, key: "generateUnityHeaders" },
              { label: $t.setting_use_topological_sort, key: "useTopologicalSort" },
            ] as item}
              <div class="flex items-center justify-between">
                <Label class="text-sm font-normal cursor-pointer">{item.label}</Label>
                <Switch checked={config[item.key]} onCheckedChange={(v) => config[item.key] = v} />
              </div>
            {/each}
            <div class="space-y-2">
              <Label class="text-xs text-muted-foreground">{$t.setting_compiler_layout}</Label>
              <div class="flex rounded-lg overflow-hidden border border-border">
                {#each [$t.layout_gcc, $t.layout_msvc] as layout, i}
                  <button
                    class="flex-1 py-1.5 text-xs font-medium transition-all cursor-pointer
                      {config.compilerLayout === (i === 0 ? 'GCC' : 'MSVC')
                        ? 'bg-primary text-primary-foreground'
                        : 'bg-card text-muted-foreground hover:text-foreground hover:bg-muted/50'}"
                    onclick={() => config.compilerLayout = i === 0 ? 'GCC' : 'MSVC'}
                  >{layout}</button>
                {/each}
              </div>
            </div>
          </div>
        </section>

        <Separator />

        <section>
          <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-primary mb-3">{$t.label_disassembly}</p>
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label class="text-sm font-normal cursor-pointer">{$t.setting_dump_disassembly}</Label>
              <Switch checked={config.dumpDisassembly} onCheckedChange={(v) => config.dumpDisassembly = v} />
            </div>
            {#if config.dumpDisassembly}
              <div class="pl-4 space-y-3 border-l-2 border-primary/20">
                <div class="space-y-2">
                  <Label class="text-xs text-muted-foreground">Target</Label>
                  <div class="flex rounded-lg overflow-hidden border border-border">
                    {#each [$t.target_both, $t.target_dump_cs, $t.target_diffable_cs] as target, i}
                      <button
                        class="flex-1 py-1.5 text-xs font-medium transition-all cursor-pointer
                          {config.dumpDisassemblyTarget === i
                            ? 'bg-primary text-primary-foreground'
                            : 'bg-card text-muted-foreground hover:text-foreground hover:bg-muted/50'}"
                        onclick={() => config.dumpDisassemblyTarget = i}
                      >{target}</button>
                    {/each}
                  </div>
                </div>
                {#each [
                  { label: $t.setting_dump_disassembly_hex_bytes, key: "dumpDisassemblyHexBytes" },
                  { label: $t.setting_dump_disassembly_field_names, key: "dumpDisassemblyFieldNames" },
                  { label: $t.setting_dump_disassembly_annotations, key: "dumpDisassemblyAnnotations" },
                  { label: $t.setting_dump_disassembly_cfg, key: "dumpDisassemblyCfg" },
                ] as item}
                  <div class="flex items-center justify-between">
                    <Label class="text-sm font-normal cursor-pointer">{item.label}</Label>
                    <Switch checked={config[item.key]} onCheckedChange={(v) => config[item.key] = v} />
                  </div>
                {/each}
                <div class="flex items-center justify-between">
                  <Label class="text-sm font-normal">{$t.setting_max_disassembly_instructions}</Label>
                  <Input
                    type="number"
                    bind:value={config.maxDisassemblyInstructions}
                    min={32} max={4096}
                    class="w-24 text-right font-mono h-8"
                  />
                </div>
              </div>
            {/if}
          </div>
        </section>

        <Separator />

        <section>
          <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-primary mb-3">{$t.label_advanced}</p>
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label class="text-sm font-normal cursor-pointer">{$t.setting_force_il2cpp_version}</Label>
              <Switch checked={config.forceIl2cppVersion} onCheckedChange={(v) => config.forceIl2cppVersion = v} />
            </div>
            {#if config.forceIl2cppVersion}
              <div class="flex items-center justify-between pl-4 border-l-2 border-primary/20">
                <Label class="text-sm font-normal text-muted-foreground">{$t.setting_force_version_label}</Label>
                <Input type="number" bind:value={config.forceVersion} step={0.1} class="w-24 text-right font-mono h-8" />
              </div>
            {/if}
            {#each [
              { label: $t.setting_force_dump, key: "forceDump" },
              { label: $t.setting_no_redirected_pointer, key: "noRedirectedPointer" },
            ] as item}
              <div class="flex items-center justify-between">
                <Label class="text-sm font-normal cursor-pointer">{item.label}</Label>
                <Switch checked={config[item.key]} onCheckedChange={(v) => config[item.key] = v} />
              </div>
            {/each}
          </div>
        </section>
    </div>

    <Dialog.Footer class="pt-4">
      <Button class="w-full" onclick={onclose}>Done</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
