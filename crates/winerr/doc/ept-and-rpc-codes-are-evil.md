EPT_S_\* and RPC_S_\* exist to haunt my nightmares.

First, winerror.h.  We have:

*   `RPC_E_*` - hresult error codes.
*   `RPC_X_*` - 16-bit error codes.
*   `RPC_S_*` - 16 bit **error** codes.
*   `EPT_S_*` - 16 bit **error** codes.

Who the hell puts `_S_` in their error code names?  And, yes, they're clearly error codes:

>   "Specifying a non-nil UUID causes the function to fail with the status code EPT_S_CANT_PERFORM_OP."<br>
>   <https://docs.microsoft.com/en-us/windows/win32/api/rpcdce/nf-rpcdce-rpcmgmtepunregister><br>
>   <https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rpce/764cb26c-cc00-4e76-8ca2-c5b840524e3f><br>
>   <https://docs.microsoft.com/en-us/windows/win32/rpc/rpc-return-values><br>

Okay, okay.  That's all fine and dandy.  I can just manually fix these up in my code scanner.
An edge case.  No problem.  But uh... just for sanity's sake... let's look a little closer.


### 10.0.19041.0\shared\winerror.h

```cpp
//
// MessageId: EPT_S_CANT_PERFORM_OP
//
// MessageText:
//
// The server endpoint cannot perform the operation.
//
#define EPT_S_CANT_PERFORM_OP            1752L
```

Yeah, that seems fine.



### 10.0.19041.0\shared\rpcnterr.h

```cpp
#if defined(EPT_S_CANT_PERFORM_OP)
#undef EPT_S_CANT_PERFORM_OP
#endif
#define EPT_S_CANT_PERFORM_OP               EPT_NT_CANT_PERFORM_OP
```

Okay, a little funky, but hey, let's look at EPT_NT_\*.  Maybe those are saner?  Should I just export those instead?



### 10.0.19041.0\shared\ntstatus.h

```cpp
//
// MessageId: EPT_NT_CANT_PERFORM_OP
//
// MessageText:
//
// The operation cannot be performed.
//
#define EPT_NT_CANT_PERFORM_OP           ((NTSTATUS)0xC0020035L)
```

Oh, cool!  It's an NTSTATUS!

...wait a sec.  Sometimes `EPT_S_CANT_PERFORM_OP` is either `1752` or `0xC0020035`... depending on what headers are included?  Those aren't the same value.  What the fuck.



### hresult.info

1.  **0x800706D8** - EPT_S_CANT_PERFORM_OP<br>
    <https://www.hresult.info/Search?q=EPT_S_CANT_PERFORM_OP>

Okay, so, that's `HRESULT_FROM_WIN32(EPT_S_CANT_PERFORM_OP)` with the winerror.h version.  But that's not the NTSTATUS value either.



### Recap

`EPT_S_CANT_PERFORM_OP` is definitely maybe *possibly* one of the following values:

| decimal      | hex        | where |
| ------------:| ----------:| ----- |
|       `1752` |      `6D8` | shared\\winerror.h
| `2147944152` | `800706D8` | [hresult.info](https://www.hresult.info/Search?q=EPT_S_CANT_PERFORM_OP)
| `2147550936` | `800106D8` | FACILITY_RPC instead of FACILITY_WIN32?
| `3221356597` | `C0020035` | shared\\{rpcnterr,ntstatus}.h

...wait, those last hex digits aren't consistent.  They didn't even maintain the same *code* for the HRESULTs and NTSTATUSes?



### Conclusion

Using `EPT_*` or `RPC_*` is a bug.  `winerr` will save you from making the mistake of using these error codes by banning them.  *You're welcome.*
