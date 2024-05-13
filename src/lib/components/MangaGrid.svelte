<div>
    <slot />
</div>

<style>
    @import 'open-props/media';

    div {
        display: grid;
        gap: var(--size-3);
        grid-template-columns: repeat(2, minmax(0, 1fr));

        @media (--sm-n-above) {
            grid-template-columns: repeat(3, minmax(0, 1fr));
        }

        @media (--md-n-above) {
            grid-template-columns: repeat(4, minmax(0, 1fr));
        }

        @media (--lg-n-above) {
            grid-template-columns: repeat(5, minmax(0, 1fr));
        }

        @media (--xl-n-above) {
            grid-template-columns: repeat(6, minmax(0, 1fr));
        }

        @media (--xxl-n-above) {
            grid-template-columns: repeat(7, minmax(0, 1fr));
        }
    }
</style>
