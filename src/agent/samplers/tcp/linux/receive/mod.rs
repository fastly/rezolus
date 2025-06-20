//! Collects TCP Receive stats using BPF and traces:
//! * `tcp_rcv_established`
//!
//! And produces these stats:
//! * `tcp/receive/jitter`
//! * `tcp/receive/srtt`

const NAME: &str = "tcp_receive";

mod bpf {
    include!(concat!(env!("OUT_DIR"), "/tcp_receive.bpf.rs"));
}

mod stats;

use bpf::*;
use stats::*;

use crate::agent::*;

use std::sync::Arc;

#[distributed_slice(SAMPLERS)]
fn init(config: Arc<Config>) -> SamplerResult {
    if !config.enabled(NAME) {
        return Ok(None);
    }

    let bpf = BpfBuilder::new(
        NAME,
        BpfProgStats {
            run_time: &BPF_RUN_TIME,
            run_count: &BPF_RUN_COUNT,
        },
        ModSkelBuilder::default,
    )
    .histogram("srtt", &TCP_SRTT)
    .histogram("jitter", &TCP_JITTER)
    .build()?;

    Ok(Some(Box::new(bpf)))
}

impl SkelExt for ModSkel<'_> {
    fn map(&self, name: &str) -> &libbpf_rs::Map {
        match name {
            "srtt" => &self.maps.srtt,
            "jitter" => &self.maps.jitter,
            _ => unimplemented!(),
        }
    }
}

impl OpenSkelExt for ModSkel<'_> {
    fn log_prog_instructions(&self) {
        debug!(
            "{NAME} tcp_rcv() BPF instruction count: {}",
            self.progs.tcp_rcv_kprobe.insn_cnt()
        );
    }
}
