//! Admin privilege detection
//!
//! Windows-specific check for elevated (admin) privileges.

/// Check if the current process is running with elevated privileges
#[cfg(target_os = "windows")]
pub fn is_elevated() -> bool {
    use std::ptr;

    // Windows API types and functions
    #[repr(C)]
    struct SID_IDENTIFIER_AUTHORITY {
        value: [u8; 6],
    }

    const SECURITY_NT_AUTHORITY: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY {
        value: [0, 0, 0, 0, 0, 5],
    };
    const SECURITY_BUILTIN_DOMAIN_RID: u32 = 0x00000020;
    const DOMAIN_ALIAS_RID_ADMINS: u32 = 0x00000220;

    #[link(name = "advapi32")]
    extern "system" {
        fn AllocateAndInitializeSid(
            pIdentifierAuthority: *const SID_IDENTIFIER_AUTHORITY,
            nSubAuthorityCount: u8,
            nSubAuthority0: u32,
            nSubAuthority1: u32,
            nSubAuthority2: u32,
            nSubAuthority3: u32,
            nSubAuthority4: u32,
            nSubAuthority5: u32,
            nSubAuthority6: u32,
            nSubAuthority7: u32,
            pSid: *mut *mut std::ffi::c_void,
        ) -> i32;

        fn CheckTokenMembership(
            TokenHandle: *mut std::ffi::c_void,
            SidToCheck: *mut std::ffi::c_void,
            IsMember: *mut i32,
        ) -> i32;

        fn FreeSid(pSid: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    }

    unsafe {
        let mut admin_group: *mut std::ffi::c_void = ptr::null_mut();

        // Create a SID for the Administrators group
        let result = AllocateAndInitializeSid(
            &SECURITY_NT_AUTHORITY,
            2,
            SECURITY_BUILTIN_DOMAIN_RID,
            DOMAIN_ALIAS_RID_ADMINS,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut admin_group,
        );

        if result == 0 {
            return false;
        }

        let mut is_member: i32 = 0;
        let check_result = CheckTokenMembership(ptr::null_mut(), admin_group, &mut is_member);

        FreeSid(admin_group);

        check_result != 0 && is_member != 0
    }
}

/// Non-Windows fallback - always returns false
#[cfg(not(target_os = "windows"))]
pub fn is_elevated() -> bool {
    false
}
