fn main() -> i32 {
    // Create memories
    if cfg!(unix) {
        mmap(
            // `addr` - let the system to choose the address at which to create the mapping.
            ptr::null_mut(),
            // the length of the mapping in bytes.
            len,
            // `prot` - protection flags: READ WRITE !EXECUTE
            libc::PROT_READ | libc::PROT_WRITE,
            // `flags`
            // `MAP_ANON` - mapping is not backed by any file and initial contents are
            // initialized to zero.
            // `MAP_PRIVATE` - the mapping is private to this process.
            libc::MAP_ANON | libc::MAP_PRIVATE,
            // `fildes` - a file descriptor. Pass -1 as this is required for some platforms
            // when the `MAP_ANON` is passed.
            -1,
            // `offset` - offset from the file.
            0,
        )
    }

    if cfg!(windows) {
        times!(
            factor = 3,
            VirtualAlloc(
                // `lpAddress` - let the system to choose the address at which to create the mapping.
                NULL,
                // `dwSize` - allocate the maximum number of wasm32 pages to bypass the overhead of rezising.
                len,
                // `flAllocationType` - reserve pages so that they are not committed to memory immediately.
                MEM_RESERVE,
                // `flProtect` - apply READ WRITE !EXECUTE protection.
                PAGE_READWRITE,
            )
        )
    }

    if cfg!(feature = aot) && options.aot {}

    // Create tables

    return 0;
}
