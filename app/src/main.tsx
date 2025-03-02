import { FluentProvider, webDarkTheme } from "@fluentui/react-components";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { NotifyProvider } from "./context";

const queryClient = new QueryClient();

createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <QueryClientProvider client={queryClient}>
            <FluentProvider theme={webDarkTheme}>
                <NotifyProvider>
                    <App />
                </NotifyProvider>
            </FluentProvider>
        </QueryClientProvider>
    </React.StrictMode>,
);
