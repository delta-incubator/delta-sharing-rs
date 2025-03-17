import {
    Field,
    makeStyles,
    tokens,
    RadioGroup,
    Radio,
} from "@fluentui/react-components";
import { useCallback } from "react";
import { CreateResource, CreateFormState, Input } from "../forms";
import ucClient, {
    CreateRecipientRequest,
    AuthenticationType,
} from "../../client";
import { InputChange, RadioChange } from "../../types";

const useStyles = makeStyles({
    root: {
        display: "flex",
        height: "100%",
        width: "100%",
        flexDirection: "column",
        overflowY: "scroll",
    },

    toolbar: {
        justifyContent: "space-between",
        borderBottomColor: tokens.colorNeutralForeground4,
        borderBottomWidth: "1px",
        borderBottomStyle: "solid",
    },

    content: {
        flex: 1,
        padding: "25px 25px 10px 25px",
        display: "flex",
        flexDirection: "column",
        rowGap: "10px",
        overflowY: "auto",
    },

    line: {
        display: "flex",
        flexDirection: "row",
        gap: "10px",
    },
});

function RecipientForm({
    values,
    setValues,
}: CreateFormState<CreateRecipientRequest>) {
    const styles = useStyles();

    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);
    const onCommentChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, comment: data.value }));
    }, []);
    const onAuthenticationTypeChange: RadioChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            authenticationType: data.value as AuthenticationType,
        }));
    }, []);

    return (
        <>
            <div className={styles.line}>
                <Field label="Authentication type">
                    <RadioGroup
                        layout="horizontal"
                        value={values.authenticationType}
                        onChange={onAuthenticationTypeChange}
                    >
                        <Radio label="Token" value="TOKEN" />
                        <Radio
                            label="OAuth Credentials"
                            value="OAUTH_CLIENT_CREDENTIALS"
                        />
                    </RadioGroup>
                </Field>
                <Input
                    label="Name"
                    style={{ flex: "1" }}
                    value={values.name ?? ""}
                    onChange={onNameChange}
                />
            </div>
            <Input
                label="Comment"
                onChange={onCommentChange}
                value={values.comment ?? ""}
            />
        </>
    );
}

function CreateExternalLocation() {
    return (
        <CreateResource
            createFn={ucClient.recipients.create}
            FormComponent={RecipientForm}
            resourceType="recipient"
            defaultValues={{ authenticationType: "TOKEN" }}
            typeName="CreateRecipientRequest"
        />
    );
}

export default CreateExternalLocation;
