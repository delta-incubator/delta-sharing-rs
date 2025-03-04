import {
    Field,
    makeStyles,
    RadioGroup,
    Radio,
    Checkbox,
    TabList,
    Tab,
} from "@fluentui/react-components";
import { useCallback, useState } from "react";
import ucClient, { CreateCredentialRequest, Purpose } from "../../client";
import {
    RadioChange,
    CheckboxChange,
    InputChange,
    TabSelect,
} from "../../types";
import { Input, CreateFormState, CreateResource } from "../forms";

const useStyles = makeStyles({
    tabsContent: {
        padding: "10px 0 0 12px",
        display: "flex",
        flexDirection: "column",
        rowGap: "10px",
    },

    line: {
        display: "flex",
        flexDirection: "row",
        gap: "10px",
    },
});

const AzureCredential = ({
    values,
    setValues,
}: CreateFormState<CreateCredentialRequest>) => {
    const onDirectoryIdChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureServicePrincipal: {
                ...curr.azureServicePrincipal,
                directoryId: data.value,
            },
        }));
    }, []);
    const onApplicationIdChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureServicePrincipal: {
                ...curr.azureServicePrincipal,
                applicationId: data.value,
            },
        }));
    }, []);
    const onClientSecretChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureServicePrincipal: {
                ...curr.azureServicePrincipal,
                clientSecret: data.value,
            },
        }));
    }, []);

    return (
        <>
            <Input
                label="Directory ID"
                value={values.azureServicePrincipal?.directoryId ?? ""}
                onChange={onDirectoryIdChange}
            />
            <Input
                label="Application ID"
                value={values.azureServicePrincipal?.applicationId ?? ""}
                onChange={onApplicationIdChange}
            />
            <Input
                label="Client sectret"
                type="password"
                value={values.azureServicePrincipal?.clientSecret ?? ""}
                onChange={onClientSecretChange}
            />
        </>
    );
};

const AzureManagedIdentity = ({
    values,
    setValues,
}: CreateFormState<CreateCredentialRequest>) => {
    const onApplicationIdChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureManagedIdentity: {
                ...curr.azureManagedIdentity,
                applicationId: data.value,
            },
        }));
    }, []);

    const onObjectIdChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureManagedIdentity: {
                ...curr.azureManagedIdentity,
                objectId: data.value,
            },
        }));
    }, []);

    const onMsiResourceIdChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureManagedIdentity: {
                ...curr.azureManagedIdentity,
                msiResourceId: data.value,
            },
        }));
    }, []);

    return (
        <>
            <Input
                label="Application ID"
                value={values.azureManagedIdentity?.applicationId ?? ""}
                onChange={onApplicationIdChange}
            />
            <Input
                label="Object ID"
                value={values.azureManagedIdentity?.objectId ?? ""}
                onChange={onObjectIdChange}
            />
            <Input
                label="MSI Resource ID"
                value={values.azureManagedIdentity?.msiResourceId ?? ""}
                onChange={onMsiResourceIdChange}
            />
        </>
    );
};

const AzureStorageKey = ({
    values,
    setValues,
}: CreateFormState<CreateCredentialRequest>) => {
    const onAccountNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureStorageKey: {
                ...curr.azureStorageKey,
                accountName: data.value,
            },
        }));
    }, []);

    const onAccountKeyChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            azureStorageKey: {
                ...curr.azureStorageKey,
                accountKey: data.value,
            },
        }));
    }, []);

    return (
        <>
            <Input
                label="Account Name"
                value={values.azureStorageKey?.accountName ?? ""}
                onChange={onAccountNameChange}
            />
            <Input
                label="Account Key"
                value={values.azureStorageKey?.accountKey ?? ""}
                onChange={onAccountKeyChange}
                type="password"
            />
        </>
    );
};

const RootProps = ({
    values,
    setValues,
}: CreateFormState<CreateCredentialRequest>) => {
    const styles = useStyles();

    const onPurposeChange: RadioChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, purpose: data.value as Purpose }));
    }, []);
    const onReadOnlyChange: CheckboxChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            readOnly: data.checked === "mixed" ? undefined : data.checked,
        }));
    }, []);
    const onSkipValidationChange: CheckboxChange = useCallback((_ev, data) => {
        setValues((curr) => ({
            ...curr,
            skipValidation: data.checked === "mixed" ? undefined : data.checked,
        }));
    }, []);
    const onNameChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, name: data.value }));
    }, []);
    const onCommentChange: InputChange = useCallback((_ev, data) => {
        setValues((curr) => ({ ...curr, comment: data.value }));
    }, []);

    return (
        <>
            <div className={styles.line}>
                <Field label="Purpose">
                    <RadioGroup
                        layout="horizontal"
                        value={values.purpose}
                        onChange={onPurposeChange}
                    >
                        <Radio label="Storage" value="STORAGE" />
                        <Radio label="Service" value="SERVICE" />
                    </RadioGroup>
                </Field>
                <Field label="Properties">
                    <div className={styles.line}>
                        <Checkbox
                            label={"read only"}
                            checked={values.readOnly ?? "mixed"}
                            onChange={onReadOnlyChange}
                        />
                        {false && (
                            <Checkbox
                                label={"skip validation"}
                                checked={values.skipValidation ?? "mixed"}
                                onChange={onSkipValidationChange}
                            />
                        )}
                    </div>
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
};

function CreateCredentialForm({
    values,
    setValues,
}: CreateFormState<CreateCredentialRequest>) {
    const styles = useStyles();

    const [credType, setCredType] = useState<
        "az-client" | "az-managed" | "az-key"
    >("az-client");
    const onSelectTab: TabSelect = useCallback((_ev, data) => {
        setCredType(data.value as typeof credType);
    }, []);

    return (
        <>
            <RootProps values={values} setValues={setValues} />
            <TabList selectedValue={credType} onTabSelect={onSelectTab}>
                <Tab value="az-client">Azure Identity</Tab>
                <Tab value="az-managed">Azure Managed Identity</Tab>
                <Tab value="az-key" disabled={values.purpose !== "STORAGE"}>
                    Azure Storage Key
                </Tab>
            </TabList>
            <div className={styles.tabsContent}>
                {credType === "az-client" && (
                    <AzureCredential values={values} setValues={setValues} />
                )}
                {credType === "az-managed" && (
                    <AzureManagedIdentity
                        values={values}
                        setValues={setValues}
                    />
                )}
                {credType === "az-key" && (
                    <AzureStorageKey values={values} setValues={setValues} />
                )}
            </div>
        </>
    );
}

function CreateCredential() {
    return (
        <CreateResource
            createFn={ucClient.credentials.create}
            FormComponent={CreateCredentialForm}
            resourceType="credential"
            typeName="CreateCredentialRequest"
            defaultValues={{
                readOnly: false,
                skipValidation: false,
                purpose: "STORAGE",
            }}
        />
    );
}

export default CreateCredential;
