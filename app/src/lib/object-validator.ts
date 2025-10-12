import { StringValidator, NumberValidator, StringArrayValidator } from './validator';

export type ValidationSchema<T> = {
    [K in keyof T]: StringValidator | NumberValidator | StringArrayValidator;
};

export type ValidationErrors<T> = {
    [K in keyof T]?: string[];
};

export function validateObject<T extends object>(
    obj: T,
    schema: ValidationSchema<T>
): ValidationErrors<T> {
    const errors: ValidationErrors<T> = {};
    for (const key in schema) {
        if (Object.prototype.hasOwnProperty.call(obj, key)) {
            const validator = schema[key];
            const value = obj[key];
            console.log(key, value)
            let validationErrors: string[] = [];

            if (validator instanceof StringValidator) {
                validationErrors = validator.validate(value as string | null | undefined).map(err => err.toString());
            } else if (validator instanceof NumberValidator) {
                validationErrors = validator.validate(value as number | null | undefined).map(err => err.toString());
            } else if (validator instanceof StringArrayValidator) {
                validationErrors = validator.validate(value as string[] | null | undefined).map(err => err.toString());
            }

            errors[key] = validationErrors;
        }
    }

    return errors;
}
