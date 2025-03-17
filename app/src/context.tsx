import {
    Toast,
    Toaster,
    ToastIntent,
    ToastTitle,
    useToastController,
    useId,
} from "@fluentui/react-components";
import React, {
    createContext,
    useCallback,
    Dispatch,
    SetStateAction,
    useMemo,
    ReactNode,
} from "react";

export const TreeContext = createContext<string[]>([]);
export const TreeProvider = TreeContext.Provider;
export const useTreeContext = () => React.useContext(TreeContext);

export const NotifyContext = createContext<
    (intent: ToastIntent, message: ReactNode) => void
>(() => {});
export const useNotify = () => React.useContext(NotifyContext);
export function NotifyProvider({ children }: { children: React.ReactNode }) {
    const toasterId = useId("toaster");
    const { dispatchToast } = useToastController(toasterId);
    const notify = useCallback(
        (intent: ToastIntent, message: ReactNode) =>
            dispatchToast(
                <Toast>
                    <ToastTitle>{message}</ToastTitle>
                </Toast>,
                { position: "bottom-end", intent },
            ),
        [],
    );

    return (
        <NotifyContext.Provider value={notify}>
            <>
                <Toaster toasterId={toasterId} />
                {children}
            </>
        </NotifyContext.Provider>
    );
}

export type ExplorerPropsInner = {
    display?: "create" | "view";
    scope?: string[];
};
export type ExplorerProps = ExplorerPropsInner & {
    update: Dispatch<SetStateAction<ExplorerPropsInner>>;
};
export const ExplorerContext = createContext<ExplorerProps>({
    update: () => {},
});
export const useExplorer = () => React.useContext(ExplorerContext);
export const ExplorerProvider = ExplorerContext.Provider;

export const useTreeScope = () => {
    const scope = useTreeContext();
    const parentScope = useMemo(
        () => scope.slice(0, scope.length - 1),
        [scope],
    );
    const parentValue = useMemo(() => parentScope.join("."), [scope]);
    const value = useMemo(() => scope.join("."), [scope]);
    return { scope, value, parentScope, parentValue };
};

export const useTypeName = (scope: string[]) => {
    if (scope.length === 2) {
        switch (scope[0]) {
            case "catalogs":
                return "Catalog";
            case "external_locations":
                return "External location";
            case "shares":
                return "Share";
            case "credentials":
                return "Credential";
            case "recipients":
                return "Recipient";
        }
    }
    if (scope[0] === "catalogs") {
        if (scope.length === 3) return "Schema";
        if (scope.length === 4) return "Table";
    }
    throw new Error(`Unknown scope: ${scope}`);
};
