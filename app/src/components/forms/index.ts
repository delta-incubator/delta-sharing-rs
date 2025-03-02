import { Dispatch, SetStateAction } from "react";
import Input from "./Input";

export { Input };

export type CreateFormState<T> = {
    values: T;
    setValues: Dispatch<SetStateAction<T>>;
};
