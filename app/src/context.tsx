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
} from "react";

export const TreeContext = createContext<string[]>([]);
export const TreeProvider = TreeContext.Provider;
export const useTreeContext = () => React.useContext(TreeContext);

export const NotifyContext = createContext<
    (intent: ToastIntent, message: string) => void
>(() => {});

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

export const useNotify = () => React.useContext(NotifyContext);

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
export const ExplorerProvider = ExplorerContext.Provider;

export const useExplorer = () => React.useContext(ExplorerContext);
