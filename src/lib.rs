mod amd;
mod intel;
mod nvidia;

use amd::AmdML;
use intel::IntelML;
use nvidia::NvidiaML;

pub struct GpuML {
    nvidia: Option<Box<NvidiaML>>,
    amd: Option<Box<AmdML>>,
    intel: Option<Box<IntelML>>,
}

impl GpuML {
    pub fn new() -> Self {
        todo!()
    }
}
