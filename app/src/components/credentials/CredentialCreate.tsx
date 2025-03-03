import {
    Field,
    makeStyles,
    Toolbar,
    ToolbarButton,
    tokens,
    RadioGroup,
    Radio,
    Checkbox,
    TabList,
    Tab,
    Text,
} from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useCallback, useState } from "react";
import ucClient, { CreateCredentialRequest, Purpose } from "../../client";
import { useNotify, useExplorer, useTreeContext } from "../../context";
import {
    RadioChange,
    CheckboxChange,
    InputChange,
    TabSelect,
} from "../../types";
import { ArrowLeftRegular, AddRegular } from "@fluentui/react-icons";
import { Input, CreateFormState } from "../forms";

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

    tabsContent: {
        padding: "10px 0 0 12px",
        display: "flex",
        flexDirection: "column",
        rowGap: "10px",
    },

    footer: {
        justifyContent: "flex-end",
        borderTopColor: tokens.colorNeutralForeground4,
        borderTopWidth: "1px",
        borderTopStyle: "solid",
        padding: "10px",
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
                value={values.azureServicePrincipal?.directoryId}
                onChange={onDirectoryIdChange}
            />
            <Input
                label="Application ID"
                value={values.azureServicePrincipal?.applicationId}
                onChange={onApplicationIdChange}
            />
            <Input
                label="Client sectret"
                type="password"
                value={values.azureServicePrincipal?.clientSecret}
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
                value={values.azureManagedIdentity?.applicationId}
                onChange={onApplicationIdChange}
            />
            <Input
                label="Object ID"
                value={values.azureManagedIdentity?.objectId}
                onChange={onObjectIdChange}
            />
            <Input
                label="MSI Resource ID"
                value={values.azureManagedIdentity?.msiResourceId}
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
                value={values.azureStorageKey?.accountName}
                onChange={onAccountNameChange}
            />
            <Input
                label="Account Key"
                value={values.azureStorageKey?.accountKey}
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
                    value={values.name}
                    onChange={onNameChange}
                />
            </div>
            <Input
                label="Comment"
                onChange={onCommentChange}
                value={values.comment}
            />
        </>
    );
};

const CreateCredential = () => {
    const styles = useStyles();

    const [credType, setCredType] = useState<
        "az-client" | "az-managed" | "az-key"
    >("az-client");
    const onSelectTab: TabSelect = useCallback((_ev, data) => {
        setCredType(data.value as typeof credType);
    }, []);

    const [values, setValues] = useState<CreateCredentialRequest>({
        purpose: "STORAGE",
        readOnly: false,
        skipValidation: false,
    });

    const notify = useNotify();
    const queryClient = useQueryClient();
    const queryKey = useTreeContext();
    const { update } = useExplorer();
    const mutation = useMutation({
        mutationFn: ucClient.credentials.create,
        onError: () => notify("error", "Failed to create credential"),
        onSuccess: () => {
            notify("success", "Credential created successfully");
            queryClient.invalidateQueries({ queryKey });
            update({});
            setValues({});
        },
    });

    const onSubmit = useCallback(() => {
        mutation.mutate(values);
    }, [mutation, values]);

    const onCancel = useCallback(() => {
        update({});
        setValues({});
    }, [update]);

    return (
        <div className={styles.root}>
            <Toolbar className={styles.toolbar} size="medium">
                <ToolbarButton
                    appearance="subtle"
                    icon={<ArrowLeftRegular />}
                    onClick={onCancel}
                />
                <Text>Create Credential</Text>
                <ToolbarButton
                    appearance="subtle"
                    icon={<AddRegular />}
                    onClick={onSubmit}
                >
                    Create
                </ToolbarButton>
            </Toolbar>
            <div className={styles.content}>
                <RootProps values={values} setValues={setValues} />
                <TabList selectedValue={credType} onTabSelect={onSelectTab}>
                    <Tab value="az-client">Azure Identity</Tab>
                    <Tab value="az-managed">Azure Managed Identity</Tab>
                    {values.purpose === "STORAGE" && (
                        <Tab value="az-key">Azure Storage Key</Tab>
                    )}
                </TabList>
                <div className={styles.tabsContent}>
                    {credType === "az-client" && (
                        <AzureCredential
                            values={values}
                            setValues={setValues}
                        />
                    )}
                    {credType === "az-managed" && (
                        <AzureManagedIdentity
                            values={values}
                            setValues={setValues}
                        />
                    )}
                    {credType === "az-key" && (
                        <AzureStorageKey
                            values={values}
                            setValues={setValues}
                        />
                    )}
                </div>
            </div>
        </div>
    );
};

export default CreateCredential;
