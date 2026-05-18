fn vm() -> bool {
    use std::arch::x86_64::__cpuid;

    unsafe {
        let result = __cpuid(1);

        (result.ecx & (1 << 31)) != 0
    }
}
