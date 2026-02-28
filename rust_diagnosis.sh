#!/bin/bash
echo "🔍 Rust环境诊断报告"
echo "=================="
echo ""
echo "1. 检查rustc版本:"
~/rust-local/rustc/bin/rustc --version
echo ""
echo "2. 检查系统根目录:"
~/rust-local/rustc/bin/rustc --print sysroot
echo ""
echo "3. 检查标准库搜索路径:"
~/rust-local/rustc/bin/rustc --print target-libdir
echo ""
echo "4. 检查标准库文件是否存在:"
echo "查找libcore.rlib:"
find ~/rust-local -name "libcore*.rlib" 2>/dev/null | head -5
echo ""
echo "查找libstd.rlib:"
find ~/rust-local -name "libstd*.rlib" 2>/dev/null | head -5
echo ""
echo "5. 检查rustlib目录结构:"
ls -la ~/rust-local/rustc/lib/rustlib/x86_64-unknown-linux-gnu/
echo ""
echo "6. 尝试编译简单程序:"
cat > /tmp/test_rust.rs << 'TESTEOF'
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
TESTEOF

echo "尝试编译无标准库程序..."
~/rust-local/rustc/bin/rustc --target x86_64-unknown-linux-gnu -C linker=ld /tmp/test_rust.rs --crate-type staticlib 2>&1 | head -20
echo ""
echo "7. 检查环境变量:"
echo "RUSTUP_HOME: ${RUSTUP_HOME:-未设置}"
echo "CARGO_HOME: ${CARGO_HOME:-未设置}"
echo ""
echo "8. 检查cargo状态:"
which cargo || echo "cargo不在PATH中"
echo ""
echo "诊断完成。"
