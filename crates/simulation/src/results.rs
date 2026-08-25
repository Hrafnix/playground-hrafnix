use crate::diagnostic::Diagnostic;
use crate::document::{PortEndpoint, SimulationSettings};
use crate::identity::{DocumentId, ProbeId, RunId};
use crate::value::RuntimeValue;
use shareable_string::ShareableString;
use std::io::{self, Write};
use units::UnitId;

/// Terminal status of a synchronous simulation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    /// Every requested timestep and sample completed.
    Completed,
    /// Initialization or stepping failed.
    Failed,
    /// Execution stopped cooperatively and retained valid partial results.
    Cancelled,
}

/// Values sampled from one persisted model probe.
#[derive(Debug, Clone, PartialEq)]
pub struct SignalSeries {
    /// Stable persisted probe identity.
    pub probe_id: ProbeId,
    /// Resolved component port sampled by the probe.
    pub source: PortEndpoint,
    /// User-facing series label.
    pub display_name: ShareableString,
    /// Fixed-grid sample times.
    pub timestamps: Vec<f64>,
    /// Runtime values corresponding one-to-one with timestamps.
    pub values: Vec<RuntimeValue>,
}

/// Scalar summary over one complete signal series.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarStatistics {
    /// Number of scalar samples.
    pub count: usize,
    /// Smallest value.
    pub minimum: f64,
    /// Largest value.
    pub maximum: f64,
    /// Arithmetic mean.
    pub mean: f64,
}

/// Failure while creating a converted result view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultViewError {
    /// A sample was not a unit-bearing scalar.
    ExpectedUnitScalar,
    /// Source and requested units are incompatible.
    IncompatibleUnit,
}

impl SignalSeries {
    /// Returns the sample at exactly `time` using total floating-point ordering.
    #[must_use]
    pub fn exact_sample(&self, time: f64) -> Option<&RuntimeValue> {
        self.timestamps
            .binary_search_by(|candidate| candidate.total_cmp(&time))
            .ok()
            .and_then(|index| self.values.get(index))
    }

    /// Returns the nearest sample, preferring the earlier sample on equal distance.
    #[must_use]
    #[allow(
        clippy::float_arithmetic,
        reason = "Nearest-sample lookup compares finite timestamp distances."
    )]
    pub fn nearest_sample(&self, time: f64) -> Option<(f64, &RuntimeValue)> {
        if self.timestamps.is_empty() || self.timestamps.len() != self.values.len() {
            return None;
        }
        let insertion = self
            .timestamps
            .partition_point(|candidate| candidate.total_cmp(&time).is_lt());
        let index = match (insertion.checked_sub(1), insertion < self.timestamps.len()) {
            (None, true) => insertion,
            (Some(previous), false) => previous,
            (Some(previous), true) => {
                let previous_time = *self.timestamps.get(previous)?;
                let next_time = *self.timestamps.get(insertion)?;
                let previous_distance = (time - previous_time).abs();
                let next_distance = (next_time - time).abs();
                if previous_distance <= next_distance {
                    previous
                } else {
                    insertion
                }
            }
            (None, false) => return None,
        };
        Some((*self.timestamps.get(index)?, self.values.get(index)?))
    }

    /// Returns the inclusive sampled time range.
    #[must_use]
    pub fn time_range(&self) -> Option<(f64, f64)> {
        Some((*self.timestamps.first()?, *self.timestamps.last()?))
    }

    /// Computes statistics when every sample is a finite scalar of one shape.
    #[must_use]
    #[allow(
        clippy::float_arithmetic,
        reason = "Result statistics perform finite scalar accumulation."
    )]
    pub fn scalar_statistics(&self) -> Option<ScalarStatistics> {
        let first = scalar_value(self.values.first()?)?;
        let mut minimum = first;
        let mut maximum = first;
        let mut sum = 0.0;
        for value in &self.values {
            let value = scalar_value(value)?;
            minimum = minimum.min(value);
            maximum = maximum.max(value);
            sum += value;
        }
        let count = self.values.len();
        let count_as_f64 = f64::from(u32::try_from(count).ok()?);
        let mean = sum / count_as_f64;
        mean.is_finite().then_some(ScalarStatistics {
            count,
            minimum,
            maximum,
            mean,
        })
    }

    /// Materializes this series in a compatible requested unit.
    ///
    /// # Errors
    ///
    /// Returns an error for unitless samples, mixed shapes, or incompatible units.
    pub fn values_in_unit(&self, target: UnitId) -> Result<Vec<f64>, ResultViewError> {
        self.values
            .iter()
            .map(|value| match value {
                RuntimeValue::ScalarWithUnit { value, unit } => {
                    units::conversion::convert(*value, *unit, target)
                        .map_err(|_| ResultViewError::IncompatibleUnit)
                }
                RuntimeValue::Boolean(_)
                | RuntimeValue::Integer(_)
                | RuntimeValue::Scalar(_)
                | RuntimeValue::String(_)
                | RuntimeValue::Identifier(_)
                | RuntimeValue::Path(_)
                | RuntimeValue::Table(_)
                | RuntimeValue::Unit(_) => Err(ResultViewError::ExpectedUnitScalar),
            })
            .collect()
    }
}

/// Metadata, diagnostics, and samples produced by one run.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationRun {
    /// Stable identity supplied by the application.
    pub run_id: RunId,
    /// Source model identity.
    pub source_document_id: DocumentId,
    /// Fixed-step settings used by the run.
    pub settings: SimulationSettings,
    /// Terminal run status.
    pub status: RunStatus,
    /// Runtime diagnostics in occurrence order.
    pub diagnostics: Vec<Diagnostic>,
    /// Probe series in persisted model order.
    pub series: Vec<SignalSeries>,
}

/// Structured CSV export failure.
#[derive(Debug)]
pub enum CsvExportError {
    /// Probe series do not share one timestamp grid or contain mismatched lengths.
    InconsistentSeries,
    /// A runtime value has no stable scalar CSV representation.
    UnsupportedValue,
    /// The destination returned an I/O failure.
    Io(io::Error),
}

impl From<io::Error> for CsvExportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl SimulationRun {
    /// Writes probe results as locale-independent CSV with time and unit rows.
    ///
    /// # Errors
    ///
    /// Returns a structured error for inconsistent series, unsupported values, or I/O failure.
    pub fn write_csv(&self, writer: &mut impl Write) -> Result<(), CsvExportError> {
        write!(writer, "time")?;
        for series in &self.series {
            write!(writer, ",{}", csv_field(series.display_name.as_str()))?;
        }
        writeln!(writer)?;

        write!(writer, "unit")?;
        for series in &self.series {
            write!(writer, ",{}", series_unit(series).unwrap_or(""))?;
        }
        writeln!(writer)?;

        let timestamps = self
            .series
            .first()
            .map_or(&[][..], |series| series.timestamps.as_slice());
        if self.series.iter().any(|series| {
            series.timestamps.as_slice() != timestamps || series.values.len() != timestamps.len()
        }) {
            return Err(CsvExportError::InconsistentSeries);
        }
        for (index, time) in timestamps.iter().enumerate() {
            write!(writer, "{time}")?;
            for series in &self.series {
                let value = series
                    .values
                    .get(index)
                    .ok_or(CsvExportError::InconsistentSeries)?;
                write!(writer, ",{}", csv_value(value)?)?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

/// Extracts either supported scalar representation.
const fn scalar_value(value: &RuntimeValue) -> Option<f64> {
    match value {
        RuntimeValue::Scalar(value) | RuntimeValue::ScalarWithUnit { value, .. } => Some(*value),
        RuntimeValue::Boolean(_)
        | RuntimeValue::Integer(_)
        | RuntimeValue::String(_)
        | RuntimeValue::Identifier(_)
        | RuntimeValue::Path(_)
        | RuntimeValue::Table(_)
        | RuntimeValue::Unit(_) => None,
    }
}

/// Returns stable unit metadata from the first sample when present.
fn series_unit(series: &SignalSeries) -> Option<&'static str> {
    match series.values.first() {
        Some(RuntimeValue::ScalarWithUnit { unit, .. }) => Some(unit.string_id().as_str()),
        Some(
            RuntimeValue::Boolean(_)
            | RuntimeValue::Integer(_)
            | RuntimeValue::Scalar(_)
            | RuntimeValue::String(_)
            | RuntimeValue::Identifier(_)
            | RuntimeValue::Path(_)
            | RuntimeValue::Table(_)
            | RuntimeValue::Unit(_),
        )
        | None => None,
    }
}

/// Formats one runtime value for a scalar CSV cell.
fn csv_value(value: &RuntimeValue) -> Result<String, CsvExportError> {
    match value {
        RuntimeValue::Boolean(value) => Ok(value.to_string()),
        RuntimeValue::Integer(value) => Ok(value.to_string()),
        RuntimeValue::Scalar(value) | RuntimeValue::ScalarWithUnit { value, .. } => {
            Ok(value.to_string())
        }
        RuntimeValue::String(value)
        | RuntimeValue::Identifier(value)
        | RuntimeValue::Path(value) => Ok(csv_field(value.as_str())),
        RuntimeValue::Table(_) | RuntimeValue::Unit(_) => Err(CsvExportError::UnsupportedValue),
    }
}

/// Escapes one RFC 4180-style CSV text field.
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::{RunStatus, ScalarStatistics, SignalSeries, SimulationRun};
    use crate::document::{LoggingPolicy, PortEndpoint, SimulationSettings};
    use crate::identity::{ComponentId, DocumentId, ProbeId, RunId};
    use crate::timing::FixedStepSemantics;
    use crate::value::RuntimeValue;
    use units::UnitId;

    fn series() -> SignalSeries {
        SignalSeries {
            probe_id: ProbeId::from_raw(1),
            source: PortEndpoint {
                component_id: ComponentId::from_raw(2),
                port_key: "out".into(),
            },
            display_name: "distance,m".into(),
            timestamps: vec![0.0, 0.5, 1.0],
            values: vec![1.0, 2.0, 4.0]
                .into_iter()
                .map(|value| RuntimeValue::ScalarWithUnit {
                    value,
                    unit: UnitId::Length_Meter,
                })
                .collect(),
        }
    }

    #[test]
    fn queries_samples_statistics_and_converted_view() {
        let series = series();

        assert_eq!(series.exact_sample(0.5), series.values.get(1));
        assert_eq!(series.nearest_sample(0.75), Some((0.5, &series.values[1])));
        assert_eq!(series.time_range(), Some((0.0, 1.0)));
        assert_eq!(
            series.scalar_statistics(),
            Some(ScalarStatistics {
                count: 3,
                minimum: 1.0,
                maximum: 4.0,
                mean: 7.0 / 3.0,
            })
        );
        assert_eq!(
            series.values_in_unit(UnitId::Length_Centimeter).unwrap(),
            vec![100.0, 200.0, 400.0]
        );
    }

    #[test]
    fn csv_has_stable_time_header_units_and_escaping() {
        let run = SimulationRun {
            run_id: RunId::from_raw(3),
            source_document_id: DocumentId::from_raw(4),
            settings: SimulationSettings {
                start_time: 0.0,
                stop_time: 1.0,
                timestep: 0.5,
                maximum_steps: 2,
                random_seed: 0,
                logging: LoggingPolicy::EveryStep,
                semantics: FixedStepSemantics::default(),
            },
            status: RunStatus::Completed,
            diagnostics: vec![],
            series: vec![series()],
        };
        let mut bytes = Vec::new();

        run.write_csv(&mut bytes).unwrap();

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            "time,\"distance,m\"\nunit,u_length_meter\n0,1\n0.5,2\n1,4\n"
        );
    }
}
