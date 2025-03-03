import { useMemo } from "react";
import { useTreeContext } from "./context";

export const useTreeScope = (name: string) => {
    const parentScope = useTreeContext();
    const scope = useMemo(() => [...parentScope, name], [parentScope, name]);
    const parentValue = useMemo(() => scope.join("."), [scope]);
    const value = `${parentValue}.${name}`;
    return { scope, value, parentScope, parentValue };
};
