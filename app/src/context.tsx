import {
    Toast,
    Toaster,
    ToastIntent,
    ToastTitle,
    useToastController,
    useId,
} from "@fluentui/react-components";
import React, { createContext, useCallback } from "react";

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
                { position: "top", intent },
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
