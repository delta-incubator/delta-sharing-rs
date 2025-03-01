import { FluentProvider, webDarkTheme } from "@fluentui/react-components";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";

const queryClient = new QueryClient();

createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <QueryClientProvider client={queryClient}>
            <FluentProvider theme={webDarkTheme}>
                <App />
            </FluentProvider>
        </QueryClientProvider>
    </React.StrictMode>,
);
