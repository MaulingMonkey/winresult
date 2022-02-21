| Type                  | 0x00<br>0xFF       | 0x100<br>0xFFFF | 0x10000<br>0x7FFFFFFF | 0x80000000<br>0xFFFFFFFF |
| --------------------- | ------------------:| ---------------:| ---------------------:| -------------------------:|
| ErrorCodeMicrosoft    | code           ✔️ | code          ✔️|                     ❌|                         ❌|
| HResult               | HResult        ✔️ |               ✔️|             HResult ✔️| HResult                 ✔️|
| HResultSuccess        | HResult        ✔️ |               ✔️|             HResult ✔️|                         ❌|
| HResultError          |                ❌ |               ❌|                     ❌| HResult                 ✔️|
| NtStatus              |                ✔️ |               ✔️|                     ✔️|                         ✔️|
| WaitCode              | several holes  ⚠️ | mostly not    ⚠️|                     ❌| just 0xFFFFFFFF         ⚠️|
| ErrorHResultOrCode    | code (nonzero) ⚠️ | code          ✔️|                     ❌| HResult                 ✔️|
