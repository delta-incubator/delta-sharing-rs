import { useMemo, RefObject } from "react";
import ucClient, { SchemaInfo } from "./client";

export const useTreeScope = (parent: string[], name: string) => {
    const scope = useMemo(() => [...parent, name], [parent, name]);
    const parentValue = useMemo(() => scope.join("."), [scope]);
    const value = `${parentValue}.${name}`;
    return { scope, value, parentValue };
};
