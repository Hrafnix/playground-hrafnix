/// The kind of component port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortKind {
    /// A port that accepts a signal input.
    SignalInput,
    /// A port that produces a signal output.
    SignalOutput,
    /// A power port for hydraulic pressure and fluid flow.
    Hydraulic,
    /// A power port for pneumatic pressure, energy flow, and mass flow.
    Pneumatic,
    /// A power port for translational force and velocity.
    TranslationalMechanical,
    /// A power port for rotational torque and angular velocity.
    RotationalMechanical,
    /// A power port for electric voltage and current.
    Electric,
    /// A power port for planar mechanical force, velocity, and torque.
    PlanarMechanical,
}

impl PortKind {
    /// Returns the port kind with the given stable identifier.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "signal_input" => Some(Self::SignalInput),
            "signal_output" => Some(Self::SignalOutput),
            "hydraulic" => Some(Self::Hydraulic),
            "pneumatic" => Some(Self::Pneumatic),
            "translational_mechanical" => Some(Self::TranslationalMechanical),
            "rotational_mechanical" => Some(Self::RotationalMechanical),
            "electric" => Some(Self::Electric),
            "planar_mechanical" => Some(Self::PlanarMechanical),
            _ => None,
        }
    }

    /// Returns the stable string representation of this port kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignalInput => "signal_input",
            Self::SignalOutput => "signal_output",
            Self::Hydraulic => "hydraulic",
            Self::Pneumatic => "pneumatic",
            Self::TranslationalMechanical => "translational_mechanical",
            Self::RotationalMechanical => "rotational_mechanical",
            Self::Electric => "electric",
            Self::PlanarMechanical => "planar_mechanical",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PortKind;

    #[test]
    fn identifiers_round_trip() {
        let port_kinds = [
            PortKind::SignalInput,
            PortKind::SignalOutput,
            PortKind::Hydraulic,
            PortKind::Pneumatic,
            PortKind::TranslationalMechanical,
            PortKind::RotationalMechanical,
            PortKind::Electric,
            PortKind::PlanarMechanical,
        ];

        for port_kind in port_kinds {
            assert_eq!(PortKind::from_id(port_kind.as_str()), Some(port_kind));
        }
    }

    #[test]
    fn unknown_identifier_returns_none() {
        assert_eq!(PortKind::from_id("unknown"), None);
    }
}
