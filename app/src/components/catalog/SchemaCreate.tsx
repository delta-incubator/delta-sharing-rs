import { useCallback } from "react";
import ucClient, { CreateSchemaRequest } from "../../client";
import { InputChange } from "../../types";
import { CreateResource, CreateFormState, Input } from "../forms";
import { useExplorer } from "../../context";

function SchemaForm({
    values,
    setValues,
}: CreateFormState<CreateSchemaRequest>) {
    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);
    const onCommentChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, comment: data.value }));
    }, []);

    return (
        <>
            <Input
                label="Name"
                value={values.name ?? ""}
                onChange={onNameChange}
            />
            <Input
                label="Comment"
                onChange={onCommentChange}
                value={values.comment ?? ""}
            />
        </>
    );
}

function CreateSchema() {
    const { scope } = useExplorer();

    if (!scope) {
        return "No scope selected";
    }

    if (scope.length !== 2 || scope[0] !== "catalogs") {
        return "Invalid scope";
    }

    return (
        <CreateResource
            createFn={ucClient.schemas.create}
            FormComponent={SchemaForm}
            resourceType="schema"
            defaultValues={{
                catalogName: scope[1],
                properties: {},
            }}
            typeName="CreateSchemaRequest"
        />
    );
}

export default CreateSchema;
