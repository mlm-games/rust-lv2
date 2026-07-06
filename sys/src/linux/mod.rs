#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
#[cfg_attr(target_arch = "arm", path = "arm.rs")]
#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
#[cfg_attr(target_arch = "riscv64", path = "riscv64.rs")]
#[cfg_attr(target_arch = "loongarch64", path = "loongarch64.rs")]
#[cfg_attr(target_arch = "sparc64", path = "sparc64.rs")]
#[cfg_attr(target_arch = "powerpc", path = "powerpc.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "powerpc64.rs")]
#[cfg_attr(target_arch = "s390x", path = "s390x.rs")]
#[cfg_attr(target_arch = "mips64", path = "mips64.rs")]
mod unsupported;
pub use unsupported::*;
