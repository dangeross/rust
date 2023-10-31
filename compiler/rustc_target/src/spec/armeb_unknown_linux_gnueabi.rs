use crate::abi::Endian;
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "armeb-unknown-linux-gnueabi".into(),
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),
        options: TargetOptions {
            abi: "eabi".into(),
            features: "+strict-align,+v8,+crc".into(),
            endian: Endian::Big,
            max_atomic_width: Some(64),
            mcount: "\u{1}__gnu_mcount_nc".into(),
            llvm_mcount_intrinsic: Some("llvm.arm.gnu.eabi.mcount".into()),
            ..super::linux_gnu_base::opts()
        },
    }
}
