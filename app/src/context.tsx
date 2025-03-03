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
} from "react";

export const TreeContext = createContext<string[]>([]);
export const TreeProvider = TreeContext.Provider;
export const useTreeContext = () => React.useContext(TreeContext);

export const NotifyContext = createContext<
    (intent: ToastIntent, message: string) => void
>(() => {});
export const useNotify = () => React.useContext(NotifyContext);
export function NotifyProvider({ children }: { children: React.ReactNode }) {
    const toasterId = useId("toaster");
    const { dispatchToast } = useToastController(toasterId);
    const notify = useCallback(
        (intent: ToastIntent, message: string) =>
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

export const useTreeScope = (name: string) => {
    const parentScope = useTreeContext();
    const scope = useMemo(() => [...parentScope, name], [parentScope, name]);
    const parentValue = useMemo(() => scope.join("."), [scope]);
    const value = `${parentValue}.${name}`;
    return { scope, value, parentScope, parentValue };
};
