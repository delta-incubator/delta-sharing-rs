import { Dispatch, SetStateAction } from "react";
import CreateResource from "./CreateResource";
import Input from "./Input";

export { CreateResource, Input };

export type CreateFormState<T> = {
    values: T;
    setValues: Dispatch<SetStateAction<T>>;
};
