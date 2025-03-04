import { useCallback } from "react";
import ucClient, { CreateShareRequest } from "../../client";
import { InputChange } from "../../types";
import { CreateResource, CreateFormState, Input } from "../forms";

function ShareForm({ values, setValues }: CreateFormState<CreateShareRequest>) {
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

function CreateShare() {
    return (
        <CreateResource
            createFn={ucClient.shares.create}
            FormComponent={ShareForm}
            resourceType="share"
            defaultValues={{}}
            typeName="CreateShareRequest"
        />
    );
}

export default CreateShare;
