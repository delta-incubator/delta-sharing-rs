import { useCallback } from "react";
import ucClient, { CreateTableRequest } from "../../client";
import { InputChange } from "../../types";
import { CreateResource, CreateFormState, Input } from "../forms";
import { useExplorer } from "../../context";

function TableForm({ values, setValues }: CreateFormState<CreateTableRequest>) {
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

function CreateTable() {
    const { scope } = useExplorer();

    if (!scope) {
        return "No scope selected";
    }

    if (scope.length !== 3 || scope[0] !== "catalogs") {
        return "Invalid scope";
    }

    return (
        <CreateResource
            createFn={ucClient.tables.create}
            FormComponent={TableForm}
            resourceType="table"
            defaultValues={{
                catalogName: scope[1],
                schemaName: scope[2],
                properties: {},
            }}
            typeName="CreateTableRequest"
        />
    );
}

export default CreateTable;
