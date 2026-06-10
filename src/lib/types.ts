export interface DumperConfig {
  dumpMethod: boolean;
  dumpField: boolean;
  dumpProperty: boolean;
  dumpAttribute: boolean;
  dumpMethodOffset: boolean;
  dumpFieldOffset: boolean;
  dumpTypeDefIndex: boolean;
  dumpAssemblyName: boolean;
  generateStruct: boolean;
  generateDummyDll: boolean;
  dummyDllAddToken: boolean;
  forceIl2cppVersion: boolean;
  forceVersion: number;
  forceDump: boolean;
  noRedirectedPointer: boolean;
  splitDumpPerType: boolean;
  generateGenericsDump: boolean;
  dumpGenericsRgctx: boolean;
  dumpGenericsMethodSpecs: boolean;
  dumpGenericsCustomAttributes: boolean;
  dumpGenericsStringLiterals: boolean;
  dumpGenericsMetadataUsages: boolean;
  dumpGenericsVtables: boolean;
  dumpGenericsInterfaces: boolean;
  dumpDisassembly: boolean;
  dumpDisassemblyTarget: number;
  dumpDisassemblyHexBytes: boolean;
  dumpDisassemblyFieldNames: boolean;
  dumpDisassemblyAnnotations: boolean;
  dumpDisassemblyCfg: boolean;
  maxDisassemblyInstructions: number;
  generateCppScaffold: boolean;
  mangleNames: boolean;
  enhancedIdaMetadata: boolean;
  generateUnityHeaders: boolean;
  compilerLayout: string;
  useTopologicalSort: boolean;
  codm: boolean;
  dumpStaticFieldMetadata: boolean;
  dumpFieldRvaData: boolean;
  maxFieldRvaDumpBytes: number;
}

export interface BinaryInfo {
  format: string;
  unity_version: string;
}

export interface DumpCompleteEvent {
  success: boolean;
  output_path: string;
  error_message: string;
}

export interface InputRequestEvent {
  prompt_type: string;
}

export type AppState = "idle" | "dumping" | "result" | "error";

export const DEFAULT_CONFIG: DumperConfig = {
  dumpMethod: true,
  dumpField: true,
  dumpProperty: true,
  dumpAttribute: true,
  dumpMethodOffset: true,
  dumpFieldOffset: true,
  dumpTypeDefIndex: true,
  dumpAssemblyName: true,
  generateStruct: true,
  generateDummyDll: true,
  dummyDllAddToken: true,
  forceIl2cppVersion: false,
  forceVersion: 24.3,
  forceDump: false,
  noRedirectedPointer: false,
  splitDumpPerType: false,
  generateGenericsDump: true,
  dumpGenericsRgctx: true,
  dumpGenericsMethodSpecs: true,
  dumpGenericsCustomAttributes: true,
  dumpGenericsStringLiterals: true,
  dumpGenericsMetadataUsages: true,
  dumpGenericsVtables: true,
  dumpGenericsInterfaces: true,
  dumpDisassembly: false,
  dumpDisassemblyTarget: 0,
  dumpDisassemblyHexBytes: true,
  dumpDisassemblyFieldNames: true,
  dumpDisassemblyAnnotations: true,
  dumpDisassemblyCfg: true,
  maxDisassemblyInstructions: 512,
  generateCppScaffold: true,
  mangleNames: true,
  enhancedIdaMetadata: true,
  generateUnityHeaders: true,
  compilerLayout: "GCC",
  useTopologicalSort: true,
  codm: false,
  dumpStaticFieldMetadata: false,
  dumpFieldRvaData: false,
  maxFieldRvaDumpBytes: 4096,
};
