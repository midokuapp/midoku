/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    return {
        props: {
            id: params.id
        }
    };
}
