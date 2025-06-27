# General FFI Rules:

| Rule                                                                      | Violation Consequence |
| ------------------------------------------------------------------------- | --------------------- |
| ❗ FFI calls must occur in `unsafe` blocks                                 | Compile error         |
| ❗ Function signatures must **exactly** match (types, ABI)                 | UB or crash           |
| ❗ Never panic or unwind across FFI                                        | UB                    |
| ❗ Pointers must be valid and aligned                                      | UB                    |
| ❗ Assume C code can do anything (e.g. write out-of-bounds, write garbage) | UB                    |
| ✅ Use defensive slicing and length checking                               | Prevents memory bugs  |

