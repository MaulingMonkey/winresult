microsoft_ntstatus_facilities! {
    #define FACILITY_ DEBUGGER                0x1
    #define FACILITY_ RPC_RUNTIME             0x2
    #define FACILITY_ RPC_STUBS               0x3
    #define FACILITY_ IO_ERROR_CODE           0x4
    #define FACILITY_ CODCLASS_ERROR_CODE     0x6
    #define FACILITY_ NTWIN32                 0x7
    #define FACILITY_ NTCERT                  0x8
    #define FACILITY_ NTSSPI                  0x9
    #define FACILITY_ TERMINAL_SERVER         0xA
    #define FACILTIY_ MUI_ERROR_CODE          0xB // NOTE: FACIL[TI]Y_ typo is in original ntstatus.h
    #define FACILITY_ USB_ERROR_CODE          0x10
    #define FACILITY_ HID_ERROR_CODE          0x11
    #define FACILITY_ FIREWIRE_ERROR_CODE     0x12
    #define FACILITY_ CLUSTER_ERROR_CODE      0x13
    #define FACILITY_ ACPI_ERROR_CODE         0x14
    #define FACILITY_ SXS_ERROR_CODE          0x15
    #define FACILITY_ TRANSACTION             0x19
    #define FACILITY_ COMMONLOG               0x1A
    #define FACILITY_ VIDEO                   0x1B
    #define FACILITY_ FILTER_MANAGER          0x1C
    #define FACILITY_ MONITOR                 0x1D
    #define FACILITY_ GRAPHICS_KERNEL         0x1E
    #define FACILITY_ DRIVER_FRAMEWORK        0x20
    #define FACILITY_ FVE_ERROR_CODE          0x21
    #define FACILITY_ FWP_ERROR_CODE          0x22
    #define FACILITY_ NDIS_ERROR_CODE         0x23
    #define FACILITY_ QUIC_ERROR_CODE         0x24
    #define FACILITY_ TPM                     0x29
    #define FACILITY_ RTPM                    0x2A
    #define FACILITY_ HYPERVISOR              0x35
    #define FACILITY_ IPSEC                   0x36
    #define FACILITY_ VIRTUALIZATION          0x37
    #define FACILITY_ VOLMGR                  0x38
    #define FACILITY_ BCD_ERROR_CODE          0x39
    #define FACILITY_ WIN32K_NTUSER           0x3E
    #define FACILITY_ WIN32K_NTGDI            0x3F
    #define FACILITY_ RESUME_KEY_FILTER       0x40
    #define FACILITY_ RDBSS                   0x41
    #define FACILITY_ BTH_ATT                 0x42
    #define FACILITY_ SECUREBOOT              0x43
    #define FACILITY_ AUDIO_KERNEL            0x44
    #define FACILITY_ VSM                     0x45
    #define FACILITY_ VOLSNAP                 0x50
    #define FACILITY_ SDBUS                   0x51
    #define FACILITY_ SHARED_VHDX             0x5C
    #define FACILITY_ SMB                     0x5D
    #define FACILITY_ XVS                     0x5E
    #define FACILITY_ INTERIX                 0x99
    #define FACILITY_ SPACES                  0xE7
    #define FACILITY_ SECURITY_CORE           0xE8
    #define FACILITY_ SYSTEM_INTEGRITY        0xE9
    #define FACILITY_ LICENSING               0xEA
    #define FACILITY_ PLATFORM_MANIFEST       0xEB
    #define FACILITY_ APP_EXEC                0xEC
    #define FACILITY_ MAXIMUM_VALUE           0xED
}
