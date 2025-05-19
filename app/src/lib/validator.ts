import { Result } from "./utils";

class Validator {
  static string(s: string): StringValidator {
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
  Empty = "EMPTY",
  nonEmpty = "NON_EMPTY",
  InvalidEmail = "INVALID_EMAIL",
  LengthLowerThanMin = "LENGTH_LT_MIN",
  LengthGreaterThanMax = "LENGTH_GT_MAX",
}

type StringValidatorType = (s: string) => Result<void, StringValidationError>;
class StringValidator {
  private validators: StringValidatorType[];

  constructor() {
    this.validators = [];
  }

  nonEmpty(): StringValidator {
    const nonEmptyValidator: StringValidatorType = (s: string): Result<void, StringValidationError> => {
      if (s.length > 0) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.Empty);
      }
    }
    this.validators.push(nonEmptyValidator);
    return this;
  }

  withMinSize(minSize: number): StringValidator {
    const minSizeValidator: StringValidatorType = (s: string): Result<void, StringValidationError> => {
      if (s.length >= minSize) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.LengthLowerThanMin);
      }
    }
    this.validators.push(minSizeValidator);
    return this;
  }

  withMaxSize(maxSize: number): StringValidator {
    const maxSizeValidator: StringValidatorType = (s: string): Result<void, StringValidationError> => {
      if (s.length <= maxSize) {
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
    const emailValidator: StringValidatorType = (s: string): Result<void, StringValidationError> => {
      if (emailRegex.test(s)) {
        return new Result<void, StringValidationError>();
      } else {
        return new Result<void, StringValidationError>(undefined, StringValidationError.InvalidEmail)
      }
    }
    this.validators.push(emailValidator);
    return this;
  }

  validate(s: string): StringValidationError[] {
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
