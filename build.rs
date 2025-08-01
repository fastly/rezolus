fn main() {
    #[cfg(target_os = "linux")]
    bpf::generate();
}

#[cfg(target_os = "linux")]
mod bpf {
    use libbpf_cargo::SkeletonBuilder;

    // `SOURCES` lists all BPF programs and the sampler that contains them.
    // Each entry `(sampler, program)` maps to a unique path in the `samplers`
    // directory.
    const SOURCES: &[(&str, &str)] = &[
        ("blockio", "latency"),
        ("blockio", "requests"),
        ("cpu", "bandwidth"),
        ("cpu", "migrations"),
        ("cpu", "perf"),
        ("cpu", "tlb_flush"),
        ("cpu", "usage"),
        ("network", "interfaces"),
        ("network", "traffic"),
        ("scheduler", "runqueue"),
        ("syscall", "counts"),
        ("syscall", "latency"),
        ("tcp", "connect_latency"),
        ("tcp", "packet_latency"),
        ("tcp", "receive"),
        ("tcp", "retransmit"),
        ("tcp", "traffic"),
    ];

    pub fn generate() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        for (sampler, prog) in SOURCES {
            let src = format!("src/agent/samplers/{sampler}/linux/{prog}/mod.bpf.c");
            let tgt = format!("{out_dir}/{sampler}_{prog}.bpf.rs");

            if target_arch == "x86_64" {
                SkeletonBuilder::new()
                    .source(&src)
                    .clang_args([
                        "-Isrc/agent/bpf/x86_64",
                        "-fno-unwind-tables",
                        "-D__TARGET_ARCH_x86",
                    ])
                    .build_and_generate(&tgt)
                    .unwrap();
            } else if target_arch == "aarch64" {
                SkeletonBuilder::new()
                    .source(&src)
                    .clang_args([
                        "-Isrc/agent/bpf/aarch64",
                        "-fno-unwind-tables",
                        "-D__TARGET_ARCH_arm64",
                    ])
                    .build_and_generate(&tgt)
                    .unwrap();
            } else {
                panic!("BPF support only available for x86_64 and aarch64 architectures");
            }

            println!("cargo:rerun-if-changed={src}");
        }

        println!("cargo:rerun-if-changed=src/agent/bpf/histogram.h");
        println!("cargo:rerun-if-changed=src/agent/bpf/vmlinux.h");
    }
}
