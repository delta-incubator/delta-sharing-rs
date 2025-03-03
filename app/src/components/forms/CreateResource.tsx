import {
    makeStyles,
    Toolbar,
    ToolbarButton,
    tokens,
    Text,
} from "@fluentui/react-components";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useCallback, useState, Dispatch, SetStateAction } from "react";
import { useNotify, useExplorer, useTreeContext } from "../../context";
import { ArrowLeftRegular, AddRegular } from "@fluentui/react-icons";

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
});

export type CreateFormState<T> = {
    values: T;
    setValues: Dispatch<SetStateAction<T>>;
};

type CreateResourceProps<Req, Res> = {
    createFn: (values: Req) => Promise<Res>;
    formComponent: (props: CreateFormState<Req>) => JSX.Element;
    defaultValues?: Req;
    resourceType?: string;
};

function CreateResource<Req, Res>({
    createFn,
    defaultValues,
    resourceType,
    formComponent,
}: CreateResourceProps<Req, Res>) {
    const styles = useStyles();
    const [values, setValues] = useState<Req>(defaultValues ?? ({} as Req));

    const notify = useNotify();
    const queryClient = useQueryClient();
    const queryKey = useTreeContext();
    const { update } = useExplorer();

    const mutation = useMutation({
        mutationFn: createFn,
        onError: () => notify("error", `Failed to create ${resourceType}`),
        onSuccess: () => {
            notify("success", `${resourceType} created successfully`);
            queryClient.invalidateQueries({ queryKey });
            update({});
            setValues({} as Req);
        },
    });

    const onSubmit = useCallback(() => {
        mutation.mutate(values);
    }, [mutation, values]);

    const onCancel = useCallback(() => {
        update({});
        setValues({} as Req);
    }, [update]);

    const FormComponent = formComponent;

    return (
        <div className={styles.root}>
            <Toolbar className={styles.toolbar} size="medium">
                <ToolbarButton
                    appearance="subtle"
                    icon={<ArrowLeftRegular />}
                    onClick={onCancel}
                />
                <Text>{`Create ${resourceType}`}</Text>
                <ToolbarButton
                    appearance="subtle"
                    icon={<AddRegular />}
                    onClick={onSubmit}
                >
                    Create
                </ToolbarButton>
            </Toolbar>
            <div className={styles.content}>
                <FormComponent values={values} setValues={setValues} />
            </div>
        </div>
    );
}

export default CreateResource;
