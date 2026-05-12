<script lang="ts">
  import lottie from "lottie-web";
  import { onMount } from "svelte";

  let { onfinished }: { onfinished: () => void } = $props();
  let container: HTMLDivElement;
  let titleVisible = $state(false);
  let versionVisible = $state(false);
  let fadeOut = $state(false);

  onMount(() => {
    const anim = lottie.loadAnimation({
      container,
      path: "/Rocket Launch.json",
      loop: false,
      autoplay: true,
      rendererSettings: { preserveAspectRatio: "xMidYMid slice" },
    });
    anim.setSpeed(2.5);

    setTimeout(() => titleVisible = true, 400);
    setTimeout(() => versionVisible = true, 700);

    anim.addEventListener("complete", () => {
      setTimeout(() => {
        fadeOut = true;
        setTimeout(onfinished, 400);
      }, 200);
    });

    setTimeout(() => {
      fadeOut = true;
      setTimeout(onfinished, 400);
    }, 4000);
  });
</script>

<div
  class="fixed inset-0 z-[100] flex items-center justify-center transition-opacity duration-400"
  style="background: var(--app-bg); color: var(--text-primary);"
  style:opacity={fadeOut ? 0 : 1}
>
  <div class="flex flex-col items-center gap-5">
    <div
      bind:this={container}
      class="size-48 transition-transform duration-500 ease-out"
      style:transform="scale({titleVisible ? 1 : 0.6})"
    ></div>

    <h1
      class="text-2xl font-bold tracking-tight transition-all duration-400 ease-out"
      style:opacity={titleVisible ? 1 : 0}
      style:transform="translateY({titleVisible ? 0 : 20}px)"
    >
      Rodroid IL2CPP Dumper
    </h1>

    <div
      class="transition-all duration-350 ease-out"
      style:opacity={versionVisible ? 1 : 0}
      style:transform="scale({versionVisible ? 1 : 0.8})"
    >
      <span class="app-badge app-badge-muted text-xs px-4 py-1">v5.5</span>
    </div>
  </div>
</div>
