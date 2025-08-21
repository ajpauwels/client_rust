//! Module implementing an Open Metrics info metric.
//!
//! See [`Info`] for details.

use std::time::SystemTime;

use crate::{
    encoding::{EncodeLabelSet, EncodeMetric, MetricEncoder},
    metrics::{MetricType, TypedMetric},
};

/// Open Metrics [`Info`] metric "to expose textual information which SHOULD NOT
/// change during process lifetime".
///
/// ```
/// # use prometheus_client::metrics::info::Info;
///
/// let _info = Info::new(vec![("os", "GNU/linux")]);
/// ```
#[derive(Debug)]
pub struct Info<S>(pub(crate) S);

impl<S> Info<S> {
    /// Create [`Info`] metric with the provided label set.
    pub fn new(label_set: S) -> Self {
        Self(label_set)
    }
}

impl<S> TypedMetric for Info<S> {
    const TYPE: MetricType = MetricType::Info;
}

impl<S> EncodeMetric for Info<S>
where
    S: Clone + std::hash::Hash + Eq + EncodeLabelSet,
{
    fn encode_with_ts(
        &self,
        mut encoder: MetricEncoder,
        _ts: SystemTime,
    ) -> Result<(), std::fmt::Error> {
        encoder.encode_info(&self.0)
    }

    fn encode(&self, mut encoder: MetricEncoder) -> Result<(), std::fmt::Error> {
        encoder.encode_info(&self.0)
    }

    fn metric_type(&self) -> MetricType {
        Self::TYPE
    }
}
