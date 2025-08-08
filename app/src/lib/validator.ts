import { Result } from "./utils";

export class Validator {
  static string(): StringValidator {
    return new StringValidator();
  }

  static int(n: number): NumberValidator {
    return new NumberValidator(n);
  }

  static stringArray(arr: string[]): ArrayValidator {
    return new ArrayValidator(arr);
  }
}


enum StringValidationError {
  nonEmpty = "Field should not be empty",
  InvalidEmail = "Invalid email",
  LengthLowerThanMin = "Value is too short",
  LengthGreaterThanMax = "Value is too long",
  InvalidEnum = "Invalid entry",
  NotEqual = "Invalid entry",
  NotDefined = "Required field"
}

type StringValidatorType = (s: string | null | undefined) => Result<void, StringValidationError>;
export class StringValidator {
  private validators: StringValidatorType[];

  constructor() {
    this.validators = [];
  }

  nonEmpty(): StringValidator {
    const nonEmptyValidator: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s || s.length > 0) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.nonEmpty);
      }
    }
    this.validators.push(nonEmptyValidator);
    return this;
  }

  withMinSize(minSize: number): StringValidator {
    const minSizeValidator: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s || s.length >= minSize) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.LengthLowerThanMin);
      }
    }
    this.validators.push(minSizeValidator);
    return this;
  }

  withMaxSize(maxSize: number): StringValidator {
    const maxSizeValidator: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s || s.length <= maxSize) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.LengthGreaterThanMax)
      }
    }
    this.validators.push(maxSizeValidator);
    return this;
  }

  email(): StringValidator {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    const emailValidator: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s) return new Result<void, StringValidationError>();
      if (emailRegex.test(s)) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.InvalidEmail)
      }
    }
    this.validators.push(emailValidator);
    return this;
  }

  in(arr: string[]): StringValidator {
    const inValidator: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s || arr.includes(s)) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.InvalidEnum)
      }
    }
    this.validators.push(inValidator);
    return this;
  }

  equal(other: string): StringValidator {
    const equal: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (!s || s === other) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.NotEqual)
      }
    }
    this.validators.push(equal);
    return this;
  }

  required(): StringValidator {
    const required: StringValidatorType = (s: string | undefined | null): Result<void, StringValidationError> => {
      if (s != null && s != undefined) {
        return new Result<void, StringValidationError>();
      } else return new Result<void, StringValidationError>(undefined, StringValidationError.NotDefined);
    }
    this.validators.push(required);
    return this;
  }

  validate(s: string | undefined | null): StringValidationError[] {
    const results = this.validators.map((validator) => validator(s));
    const errors = results
      .map((error) => error.error)
      .filter((error) => error != undefined);
    return errors;
  }
}

class NumberValidator {
  value: number

  constructor(n: number) {
    this.value = n;
  }
}

class ArrayValidator {
  value: string[]

  constructor(arr: string[]) {
    this.value = arr;
  }
}
