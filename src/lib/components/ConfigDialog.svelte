<script lang="ts">
  import { Button, IconButton, SegmentedButton, TextField, Divider } from "noph-ui";
  import { Icon } from "noph-ui/icons";
  import type { DumperConfig } from "$lib/types";
  import { t } from "$lib/stores";
  import ConfigSwitch from "./ConfigSwitch.svelte";
  import AnimatedExpand from "./AnimatedExpand.svelte";

  let { config = $bindable(), onclose }: { config: DumperConfig; onclose: () => void } = $props();
</script>

<div class="m3-dialog-scrim m3-scrim-enter" role="presentation">
  <button type="button" class="absolute inset-0 cursor-default" aria-label="Close dialog" onclick={onclose}></button>
  <div class="m3-dialog m3-dialog-enter" role="dialog" aria-labelledby="config-dialog-title">
    <div class="p-6 pb-4 shrink-0">
      <div class="flex items-center gap-3">
        <div class="m3-icon-tile size-10">
          <Icon>tune</Icon>
        </div>
        <div class="flex-1 min-w-0">
          <h3 id="config-dialog-title" class="text-lg font-semibold">{$t.dump_options}</h3>
          <p class="text-xs m3-secondary">Configure what gets included in the dump output.</p>
        </div>
        <IconButton aria-label="Close" onclick={onclose}>
          <Icon>close</Icon>
        </IconButton>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto overflow-x-hidden px-6 pb-2 space-y-5">
      <section>
        <p class="m3-label mb-3">{$t.label_output}</p>
        <div class="space-y-1">
          <ConfigSwitch label={$t.setting_dump_method} bind:checked={config.dumpMethod} />
          <ConfigSwitch label={$t.setting_dump_field} bind:checked={config.dumpField} />
          <ConfigSwitch label={$t.setting_dump_property} bind:checked={config.dumpProperty} />
          <ConfigSwitch label={$t.setting_dump_attribute} bind:checked={config.dumpAttribute} />
          <ConfigSwitch label={$t.setting_dump_method_offset} bind:checked={config.dumpMethodOffset} />
          <ConfigSwitch label={$t.setting_dump_field_offset} bind:checked={config.dumpFieldOffset} />
          <ConfigSwitch label={$t.setting_dump_typedef_index} bind:checked={config.dumpTypeDefIndex} />
          <ConfigSwitch label={$t.setting_dump_assembly_name} bind:checked={config.dumpAssemblyName} />
          <ConfigSwitch label={$t.setting_split_dump_per_type} bind:checked={config.splitDumpPerType} />
        </div>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_generation}</p>
        <div class="space-y-1">
          <ConfigSwitch label={$t.setting_generate_struct} bind:checked={config.generateStruct} />
          <ConfigSwitch label={$t.setting_generate_dummy_dll} bind:checked={config.generateDummyDll} />
          <ConfigSwitch label={$t.setting_dummy_dll_add_token} bind:checked={config.dummyDllAddToken} />
        </div>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_advanced_generics}</p>
        <ConfigSwitch label={$t.setting_generate_generics_dump} bind:checked={config.generateGenericsDump} />
        <AnimatedExpand show={config.generateGenericsDump}>
          <div class="m3-nested-options space-y-1 pt-2">
            <ConfigSwitch label={$t.setting_dump_generics_rgctx} bind:checked={config.dumpGenericsRgctx} />
            <ConfigSwitch label={$t.setting_dump_generics_method_specs} bind:checked={config.dumpGenericsMethodSpecs} />
            <ConfigSwitch label={$t.setting_dump_generics_custom_attributes} bind:checked={config.dumpGenericsCustomAttributes} />
            <ConfigSwitch label={$t.setting_dump_generics_string_literals} bind:checked={config.dumpGenericsStringLiterals} />
            <ConfigSwitch label={$t.setting_dump_generics_metadata_usages} bind:checked={config.dumpGenericsMetadataUsages} />
            <ConfigSwitch label={$t.setting_dump_generics_vtables} bind:checked={config.dumpGenericsVtables} />
            <ConfigSwitch label={$t.setting_dump_generics_interfaces} bind:checked={config.dumpGenericsInterfaces} />
          </div>
        </AnimatedExpand>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_cpp_headers}</p>
        <div class="space-y-1">
          <ConfigSwitch label={$t.setting_generate_unity_headers} bind:checked={config.generateUnityHeaders} />
          <ConfigSwitch label={$t.setting_generate_cpp_scaffold} bind:checked={config.generateCppScaffold} />
          <ConfigSwitch label={$t.setting_mangle_names} bind:checked={config.mangleNames} />
          <ConfigSwitch label={$t.setting_enhanced_ida_metadata} bind:checked={config.enhancedIdaMetadata} />
          <ConfigSwitch label={$t.setting_use_topological_sort} bind:checked={config.useTopologicalSort} />
          <div class="space-y-2 pt-2">
            <span class="text-xs m3-secondary">{$t.setting_compiler_layout}</span>
            <SegmentedButton
              class="m3-segmented-inline"
              name="compiler-layout"
              options={[
                { label: $t.layout_gcc, selected: config.compilerLayout === "GCC", onclick: () => (config.compilerLayout = "GCC") },
                { label: $t.layout_msvc, selected: config.compilerLayout === "MSVC", onclick: () => (config.compilerLayout = "MSVC") },
              ]}
            />
          </div>
        </div>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_disassembly}</p>
        <ConfigSwitch label={$t.setting_dump_disassembly} bind:checked={config.dumpDisassembly} />
        <AnimatedExpand show={config.dumpDisassembly}>
          <div class="m3-nested-options space-y-3 pt-2">
            <div class="space-y-2">
              <span class="text-xs m3-secondary">{$t.setting_dump_disassembly_target}</span>
              <SegmentedButton
                class="m3-segmented-inline"
                name="disasm-target"
                options={[
                  { label: $t.target_both, selected: config.dumpDisassemblyTarget === 0, onclick: () => (config.dumpDisassemblyTarget = 0) },
                  { label: $t.target_dump_cs, selected: config.dumpDisassemblyTarget === 1, onclick: () => (config.dumpDisassemblyTarget = 1) },
                  { label: $t.target_diffable_cs, selected: config.dumpDisassemblyTarget === 2, onclick: () => (config.dumpDisassemblyTarget = 2) },
                ]}
              />
            </div>
            <ConfigSwitch label={$t.setting_dump_disassembly_hex_bytes} bind:checked={config.dumpDisassemblyHexBytes} />
            <ConfigSwitch label={$t.setting_dump_disassembly_field_names} bind:checked={config.dumpDisassemblyFieldNames} />
            <ConfigSwitch label={$t.setting_dump_disassembly_annotations} bind:checked={config.dumpDisassemblyAnnotations} />
            <ConfigSwitch label={$t.setting_dump_disassembly_cfg} bind:checked={config.dumpDisassemblyCfg} />
            <TextField
              type="number"
              variant="filled"
              label={$t.setting_max_disassembly_instructions}
              bind:value={config.maxDisassemblyInstructions}
            />
          </div>
        </AnimatedExpand>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_static_metadata}</p>
        <ConfigSwitch label={$t.setting_dump_static_metadata} bind:checked={config.dumpStaticFieldMetadata} />
        <AnimatedExpand show={config.dumpStaticFieldMetadata}>
          <div class="m3-nested-options space-y-3 pt-2">
            <ConfigSwitch label={$t.setting_dump_field_rva_data} bind:checked={config.dumpFieldRvaData} />
            <TextField
              type="number"
              variant="filled"
              label={$t.setting_max_field_rva_dump_bytes}
              bind:value={config.maxFieldRvaDumpBytes}
            />
          </div>
        </AnimatedExpand>
      </section>

      <Divider />

      <section>
        <p class="m3-label mb-3">{$t.label_advanced}</p>
        <div class="space-y-1">
          <ConfigSwitch label={$t.setting_force_il2cpp_version} bind:checked={config.forceIl2cppVersion} />
          <AnimatedExpand show={config.forceIl2cppVersion}>
            <div class="m3-nested-options pt-2 pb-1">
              <TextField
                type="number"
                variant="filled"
                label={$t.setting_force_version_label}
                bind:value={config.forceVersion}
              />
            </div>
          </AnimatedExpand>
          <ConfigSwitch label={$t.setting_force_dump} bind:checked={config.forceDump} />
          <ConfigSwitch label={$t.setting_no_redirected_pointer} bind:checked={config.noRedirectedPointer} />
          <ConfigSwitch label={$t.setting_codm} bind:checked={config.codm} />
        </div>
      </section>
    </div>

    <div class="p-6 pt-4 shrink-0 border-t m3-dialog-actions" style="border-color: var(--np-color-outline-variant);">
      <Button variant="text" onclick={onclose}>{$t.dialog_cancel}</Button>
      <Button variant="filled" onclick={onclose}>{$t.dialog_ok}</Button>
    </div>
  </div>
</div>
