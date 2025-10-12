import { describe, it, expect } from 'vitest';
import { validateObject, type ValidationSchema } from './lib';
import { Validator } from './lib/validator';

describe('validateObject', () => {
    it('should return an empty object if the object is valid', () => {
        const obj = {
            name: 'John Doe',
            age: 30,
        };

        const schema: ValidationSchema<typeof obj> = {
            name: Validator.string().required(),
            age: Validator.number().required().isPositive(),
        };

        const errors = validateObject(obj, schema);

        expect(errors).toEqual({
            name: [], age: []
        });
    });

    it('should return an object with errors if the object is invalid', () => {
        const obj = {
            name: '',
            age: -1,
        };

        const schema: ValidationSchema<typeof obj> = {
            name: Validator.string().required().nonEmpty(),
            age: Validator.number().required().isPositive(),
        };

        const errors = validateObject(obj, schema);

        expect(errors).toEqual({
            name: ['Field should not be empty'],
            age: ['Not Positive'],
        });
    });
});