<script>
    import { AspectRatio as AspectRatioPrimitive } from "bits-ui";
    export let ratio = 4 / 3;
</script>

<AspectRatioPrimitive.Root {ratio} {...$$restProps}>
    <slot />
</AspectRatioPrimitive.Root>
