import { Tabs as TabsPrimitive } from "bits-ui";

import Content from "./tabs-content.svelte";
import List from "./tabs-list.svelte";
import Trigger from "./tabs-trigger.svelte";

const Root = TabsPrimitive.Root;

export {
    Content,
    Content as TabsContent,
    List,
    List as TabsList,
    Root,
    //
    Root as Tabs,
    Trigger,
    Trigger as TabsTrigger,
};
