| Type                  | 0x00<br>0xFF       | 0x100<br>0xFFFF | 0x10000<br>0x7FFFFFFF | 0x80000000<br>0xFFFFFFFF |
| --------------------- | ------------------:| ---------------:| ---------------------:| -------------------------:|
| SuccessCodeMicrosoft  | code           ✔️ | code          ✔️|                     ❌|                         ❌|
| ErrorCodeMicrosoft    | code (nonzero) ⚠️ | code          ✔️|                     ❌|                         ❌|
| WaitCode              | several holes  ⚠️ | mostly not    ⚠️|                     ❌| just 0xFFFFFFFF         ⚠️|
| ErrorOrHresult        | code (nonzero) ⚠️ | code          ✔️|                     ❌| Hresult                 ✔️|
| SuccessHresult        | Hresult        ✔️ |               ✔️|             Hresult ✔️|                         ❌|
| ErrorHresult          |                ❌ |               ❌|                     ❌| Hresult                 ✔️|
| Hresult               | Hresult        ✔️ |               ✔️|             Hresult ✔️| Hresult                 ✔️|
| NtStatus              |                ✔️ |               ✔️|                     ✔️|                         ✔️|
