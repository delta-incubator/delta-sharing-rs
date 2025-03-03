import Editor, { useMonaco } from "@monaco-editor/react";
import { useEffect } from "react";
import schemas from "./schemas";

function JsonEditor() {
    const monaco = useMonaco();

    useEffect(() => {
        const infos = Object.entries(schemas).map(([key, value]) => {
            return {
                uri: value.$id,
                schema: value,
                fileMatch: [`${key}.json`],
            };
        });
        // or make sure that it exists by other ways
        if (monaco) {
            console.log("here is the monaco instance:", monaco);
            monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
                schemas: Object.entries(schemas).map(([key, value]) => ({
                    uri: value.$id,
                    schema: value,
                    fileMatch: [`${key}.json`],
                })),
            });
        }
    }, [monaco]);

    return (
        <Editor
            height="100%"
            path="CreateCredentialRequest.json"
            defaultLanguage="json"
            defaultValue="{}"
            theme="vs-dark"
        />
    );
}

export default JsonEditor;
