mod amd;
mod intel;
mod nvidia;

use std::sync::Arc;

use amd::AmdML;
use intel::IntelML;
use nvidia::NvidiaML;

pub struct GpuML {
    nvidia: Option<Arc<NvidiaML>>,
    amd: Option<Arc<AmdML>>,
    intel: Option<Arc<IntelML>>,
}

pub enum GpuID {
    Nvidia(/* todo: NVIDIA IDs */),
    Amd(/* todo: AMD IDs */),
    Intel(/* todo: Intel IDs */),
}

impl GpuML {
    pub fn new() -> Self {
        Self {
            nvidia: NvidiaML::new(),
            amd: AmdML::new(),
            intel: IntelML::new(),
        }
    }
}
