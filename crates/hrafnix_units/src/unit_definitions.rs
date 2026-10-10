use hrafnix_keys::{ConstUnitKey, unit_key};

/// `UnitFamilyId` is an enum that represents the different families of units.
/// Each family has a unique identifier that can be used to group units together.
/// The `UnitFamilyId` enum is used in the Unit struct to specify the family of a unit.
/// IDs list supported SI base quantities first, named SI derived quantities next,
/// and remaining compound quantities last, following the unitless sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UnitFamilyId {
    /// The family of units that do not belong to any specific category.
    None = 0,
    /// The family of units that measure length.
    Length = 1,
    /// The family of units that measure mass.
    Mass = 2,
    /// The family of units that measure time.
    Time = 3,
    /// The family of units that measure electric current.
    Current = 4,
    /// The family of units that measure temperature.
    Temperature = 5,
    /// The family of units that measure the amount of substance.
    Amount = 6,
    /// The family of units that measure frequency.
    Frequency = 7,
    /// The family of units that measure plane angle.
    Angle = 8,
    /// The family of units that measure solid angle.
    SolidAngle = 9,
    /// The family of units that measure force or weight.
    Force = 10,
    /// The family of units that measure pressure or stress.
    Pressure = 11,
    /// The family of units that measure energy, work, or heat.
    Energy = 12,
    /// The family of units that measure power or radiant flux.
    Power = 13,
    /// The family of units that measure electric charge.
    ElectricCharge = 14,
    /// The family of units that measure voltage or electric potential.
    Voltage = 15,
    /// The family of units that measure electrical resistance, reactance, or impedance.
    Resistance = 16,
    /// The family of units that measure electrical conductance, susceptance, or admittance.
    Conductance = 17,
    /// The family of units that measure capacitance.
    Capacitance = 18,
    /// The family of units that measure inductance or permeance.
    Inductance = 19,
    /// The family of units that measure magnetic flux density.
    MagneticFluxDensity = 20,
    /// The family of units that measure magnetic flux.
    MagneticFlux = 21,
    /// The family of units that measure area.
    Area = 22,
    /// The family of units that measure volume.
    Volume = 23,
    /// The family of units that measure speed.
    Speed = 24,
    /// The family of units that measure acceleration.
    Acceleration = 25,
    /// The family of units that measure angular velocity.
    AngularVelocity = 26,
    /// The family of units that measure torque.
    Torque = 27,
    /// The family of units that measure mass density.
    Density = 28,
    /// The family of units that measure volume flow rate.
    VolumeFlowRate = 29,
    /// The family of units that measure pump or motor displacement per revolution.
    DisplacementPerRevolution = 30,
    /// The family of units that measure angular frequency, distinct from frequency and angular velocity.
    AngularFrequency = 31,
    /// The family of units that measure rotational inertia.
    MomentOfInertia = 32,
    /// The family of units that measure force per linear displacement.
    LinearStiffness = 33,
    /// The family of units that measure force per linear velocity.
    LinearDamping = 34,
    /// The family of units that measure torque per angular displacement.
    RotationalStiffness = 35,
    /// The family of units that measure torque per angular velocity.
    RotationalDamping = 36,
    /// The family of units that measure dynamic viscosity.
    DynamicViscosity = 37,
    /// The family of units that measure kinematic viscosity.
    KinematicViscosity = 38,
    /// The family of units that measure mass flow rate.
    MassFlowRate = 39,
    /// The family of units that measure linear momentum.
    Momentum = 40,
    /// The family of units that measure hydraulic leakage flow per pressure difference.
    HydraulicLeakageCoefficient = 41,
    /// The family of units that measure heat capacity per unit mass.
    SpecificHeatCapacity = 42,
    /// The family of units that measure the gas constant per unit mass.
    SpecificGasConstant = 43,
    /// The family of units that measure heat flow per temperature difference.
    ThermalConductance = 44,
    /// The family of units that measure pressure difference per volume flow.
    HydraulicResistance = 45,
    /// The family of units that measure pneumatic pressure per energy flow.
    PneumaticCharacteristicImpedance = 46,
    /// The family of units that measure volume flow per square root of pressure difference.
    TurbulentFlowCoefficient = 47,
    /// The family of units that measure back electromotive force per angular velocity.
    MotorBackEmfConstant = 48,
    /// The family of units that measure linear travel per angular displacement.
    ScrewPitch = 49,
    /// The family of units that measure fuel mass flow per thrust.
    ThrustSpecificFuelConsumption = 50,
    /// The family of units that measure the time derivative of volume flow.
    VolumeFlowAcceleration = 51,
}

impl UnitFamilyId {
    /// All supported unit families in numeric and display order.
    pub const ALL: [Self; 52] = [
        Self::None,
        Self::Length,
        Self::Mass,
        Self::Time,
        Self::Current,
        Self::Temperature,
        Self::Amount,
        Self::Frequency,
        Self::Angle,
        Self::SolidAngle,
        Self::Force,
        Self::Pressure,
        Self::Energy,
        Self::Power,
        Self::ElectricCharge,
        Self::Voltage,
        Self::Resistance,
        Self::Conductance,
        Self::Capacitance,
        Self::Inductance,
        Self::MagneticFluxDensity,
        Self::MagneticFlux,
        Self::Area,
        Self::Volume,
        Self::Speed,
        Self::Acceleration,
        Self::AngularVelocity,
        Self::Torque,
        Self::Density,
        Self::VolumeFlowRate,
        Self::DisplacementPerRevolution,
        Self::AngularFrequency,
        Self::MomentOfInertia,
        Self::LinearStiffness,
        Self::LinearDamping,
        Self::RotationalStiffness,
        Self::RotationalDamping,
        Self::DynamicViscosity,
        Self::KinematicViscosity,
        Self::MassFlowRate,
        Self::Momentum,
        Self::HydraulicLeakageCoefficient,
        Self::SpecificHeatCapacity,
        Self::SpecificGasConstant,
        Self::ThermalConductance,
        Self::HydraulicResistance,
        Self::PneumaticCharacteristicImpedance,
        Self::TurbulentFlowCoefficient,
        Self::MotorBackEmfConstant,
        Self::ScrewPitch,
        Self::ThrustSpecificFuelConsumption,
        Self::VolumeFlowAcceleration,
    ];

    /// Returns the `UnitFamilyId` corresponding to the given u8 value.
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(UnitFamilyId::None),
            1 => Some(UnitFamilyId::Length),
            2 => Some(UnitFamilyId::Mass),
            3 => Some(UnitFamilyId::Time),
            4 => Some(UnitFamilyId::Current),
            5 => Some(UnitFamilyId::Temperature),
            6 => Some(UnitFamilyId::Amount),
            7 => Some(UnitFamilyId::Frequency),
            8 => Some(UnitFamilyId::Angle),
            9 => Some(UnitFamilyId::SolidAngle),
            10 => Some(UnitFamilyId::Force),
            11 => Some(UnitFamilyId::Pressure),
            12 => Some(UnitFamilyId::Energy),
            13 => Some(UnitFamilyId::Power),
            14 => Some(UnitFamilyId::ElectricCharge),
            15 => Some(UnitFamilyId::Voltage),
            16 => Some(UnitFamilyId::Resistance),
            17 => Some(UnitFamilyId::Conductance),
            18 => Some(UnitFamilyId::Capacitance),
            19 => Some(UnitFamilyId::Inductance),
            20 => Some(UnitFamilyId::MagneticFluxDensity),
            21 => Some(UnitFamilyId::MagneticFlux),
            22 => Some(UnitFamilyId::Area),
            23 => Some(UnitFamilyId::Volume),
            24 => Some(UnitFamilyId::Speed),
            25 => Some(UnitFamilyId::Acceleration),
            26 => Some(UnitFamilyId::AngularVelocity),
            27 => Some(UnitFamilyId::Torque),
            28 => Some(UnitFamilyId::Density),
            29 => Some(UnitFamilyId::VolumeFlowRate),
            30 => Some(UnitFamilyId::DisplacementPerRevolution),
            31 => Some(UnitFamilyId::AngularFrequency),
            32 => Some(UnitFamilyId::MomentOfInertia),
            33 => Some(UnitFamilyId::LinearStiffness),
            34 => Some(UnitFamilyId::LinearDamping),
            35 => Some(UnitFamilyId::RotationalStiffness),
            36 => Some(UnitFamilyId::RotationalDamping),
            37 => Some(UnitFamilyId::DynamicViscosity),
            38 => Some(UnitFamilyId::KinematicViscosity),
            39 => Some(UnitFamilyId::MassFlowRate),
            40 => Some(UnitFamilyId::Momentum),
            41 => Some(UnitFamilyId::HydraulicLeakageCoefficient),
            42 => Some(UnitFamilyId::SpecificHeatCapacity),
            43 => Some(UnitFamilyId::SpecificGasConstant),
            44 => Some(UnitFamilyId::ThermalConductance),
            45 => Some(UnitFamilyId::HydraulicResistance),
            46 => Some(UnitFamilyId::PneumaticCharacteristicImpedance),
            47 => Some(UnitFamilyId::TurbulentFlowCoefficient),
            48 => Some(UnitFamilyId::MotorBackEmfConstant),
            49 => Some(UnitFamilyId::ScrewPitch),
            50 => Some(UnitFamilyId::ThrustSpecificFuelConsumption),
            51 => Some(UnitFamilyId::VolumeFlowAcceleration),
            _ => None,
        }
    }

    /// Returns the u8 value corresponding to the given `UnitFamilyId`.
    #[must_use]
    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Length => 1,
            Self::Mass => 2,
            Self::Time => 3,
            Self::Current => 4,
            Self::Temperature => 5,
            Self::Amount => 6,
            Self::Frequency => 7,
            Self::Angle => 8,
            Self::SolidAngle => 9,
            Self::Force => 10,
            Self::Pressure => 11,
            Self::Energy => 12,
            Self::Power => 13,
            Self::ElectricCharge => 14,
            Self::Voltage => 15,
            Self::Resistance => 16,
            Self::Conductance => 17,
            Self::Capacitance => 18,
            Self::Inductance => 19,
            Self::MagneticFluxDensity => 20,
            Self::MagneticFlux => 21,
            Self::Area => 22,
            Self::Volume => 23,
            Self::Speed => 24,
            Self::Acceleration => 25,
            Self::AngularVelocity => 26,
            Self::Torque => 27,
            Self::Density => 28,
            Self::VolumeFlowRate => 29,
            Self::DisplacementPerRevolution => 30,
            Self::AngularFrequency => 31,
            Self::MomentOfInertia => 32,
            Self::LinearStiffness => 33,
            Self::LinearDamping => 34,
            Self::RotationalStiffness => 35,
            Self::RotationalDamping => 36,
            Self::DynamicViscosity => 37,
            Self::KinematicViscosity => 38,
            Self::MassFlowRate => 39,
            Self::Momentum => 40,
            Self::HydraulicLeakageCoefficient => 41,
            Self::SpecificHeatCapacity => 42,
            Self::SpecificGasConstant => 43,
            Self::ThermalConductance => 44,
            Self::HydraulicResistance => 45,
            Self::PneumaticCharacteristicImpedance => 46,
            Self::TurbulentFlowCoefficient => 47,
            Self::MotorBackEmfConstant => 48,
            Self::ScrewPitch => 49,
            Self::ThrustSpecificFuelConsumption => 50,
            Self::VolumeFlowAcceleration => 51,
        }
    }

    /// Returns the display name for this family.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            UnitFamilyId::None => "None",
            UnitFamilyId::Length => "Length",
            UnitFamilyId::Mass => "Mass",
            UnitFamilyId::Time => "Time",
            UnitFamilyId::Current => "Current",
            UnitFamilyId::Temperature => "Temperature",
            UnitFamilyId::Amount => "Amount",
            UnitFamilyId::Frequency => "Frequency",
            UnitFamilyId::Angle => "Angle",
            UnitFamilyId::SolidAngle => "Solid Angle",
            UnitFamilyId::Force => "Force",
            UnitFamilyId::Pressure => "Pressure",
            UnitFamilyId::Energy => "Energy",
            UnitFamilyId::Power => "Power",
            UnitFamilyId::ElectricCharge => "Electric Charge",
            UnitFamilyId::Voltage => "Voltage",
            UnitFamilyId::Resistance => "Resistance",
            UnitFamilyId::Conductance => "Conductance",
            UnitFamilyId::Capacitance => "Capacitance",
            UnitFamilyId::Inductance => "Inductance",
            UnitFamilyId::MagneticFluxDensity => "Magnetic Flux Density",
            UnitFamilyId::MagneticFlux => "Magnetic Flux",
            UnitFamilyId::Area => "Area",
            UnitFamilyId::Volume => "Volume",
            UnitFamilyId::Speed => "Speed",
            UnitFamilyId::Acceleration => "Acceleration",
            UnitFamilyId::AngularVelocity => "Angular Velocity",
            UnitFamilyId::Torque => "Torque",
            UnitFamilyId::Density => "Density",
            UnitFamilyId::VolumeFlowRate => "Volume Flow Rate",
            UnitFamilyId::DisplacementPerRevolution => "Displacement per Revolution",
            UnitFamilyId::AngularFrequency => "Angular Frequency",
            UnitFamilyId::MomentOfInertia => "Moment of Inertia",
            UnitFamilyId::LinearStiffness => "Linear Stiffness",
            UnitFamilyId::LinearDamping => "Linear Damping",
            UnitFamilyId::RotationalStiffness => "Rotational Stiffness",
            UnitFamilyId::RotationalDamping => "Rotational Damping",
            UnitFamilyId::DynamicViscosity => "Dynamic Viscosity",
            UnitFamilyId::KinematicViscosity => "Kinematic Viscosity",
            UnitFamilyId::MassFlowRate => "Mass Flow Rate",
            UnitFamilyId::Momentum => "Momentum",
            UnitFamilyId::HydraulicLeakageCoefficient => "Hydraulic Leakage Coefficient",
            UnitFamilyId::SpecificHeatCapacity => "Specific Heat Capacity",
            UnitFamilyId::SpecificGasConstant => "Specific Gas Constant",
            UnitFamilyId::ThermalConductance => "Thermal Conductance",
            UnitFamilyId::HydraulicResistance => "Hydraulic Resistance",
            UnitFamilyId::PneumaticCharacteristicImpedance => "Pneumatic Characteristic Impedance",
            UnitFamilyId::TurbulentFlowCoefficient => "Turbulent Flow Coefficient",
            UnitFamilyId::MotorBackEmfConstant => "Motor Back EMF Constant",
            UnitFamilyId::ScrewPitch => "Screw Pitch",
            UnitFamilyId::ThrustSpecificFuelConsumption => "Thrust-Specific Fuel Consumption",
            UnitFamilyId::VolumeFlowAcceleration => "Volume Flow Acceleration",
        }
    }
    /// Returns the unit identifiers for all units in this family.
    #[must_use]
    pub const fn unit_ids(&self) -> &[UnitId] {
        match self {
            UnitFamilyId::None => &[UnitId::None],
            UnitFamilyId::Length => &[
                UnitId::Length_Meter,
                UnitId::Length_Picometer,
                UnitId::Length_Nanometer,
                UnitId::Length_Micrometer,
                UnitId::Length_Millimeter,
                UnitId::Length_Centimeter,
                UnitId::Length_Kilometer,
                UnitId::Length_Foot,
                UnitId::Length_Inch,
                UnitId::Length_Yard,
                UnitId::Length_Mile,
            ],
            UnitFamilyId::Mass => &[
                UnitId::Mass_Kilogram,
                UnitId::Mass_Gram,
                UnitId::Mass_Picogram,
                UnitId::Mass_Nanogram,
                UnitId::Mass_Microgram,
                UnitId::Mass_Milligram,
                UnitId::Mass_Megagram,
                UnitId::Mass_Tonne,
                UnitId::Mass_Pound,
                UnitId::Mass_Ounce,
                UnitId::Mass_Stone,
            ],
            UnitFamilyId::Time => &[
                UnitId::Time_Second,
                UnitId::Time_PicoSecond,
                UnitId::Time_NanoSecond,
                UnitId::Time_MicroSecond,
                UnitId::Time_MilliSecond,
                UnitId::Time_KiloSecond,
                UnitId::Time_Minute,
                UnitId::Time_Hour,
                UnitId::Time_Day,
                UnitId::Time_Week,
                UnitId::Time_Year,
            ],
            UnitFamilyId::Current => &[
                UnitId::Current_Ampere,
                UnitId::Current_Nanoampere,
                UnitId::Current_Microampere,
                UnitId::Current_Milliampere,
                UnitId::Current_Kiloampere,
                UnitId::Current_Megaampere,
            ],
            UnitFamilyId::Temperature => &[
                UnitId::Temperature_Kelvin,
                UnitId::Temperature_Nanokelvin,
                UnitId::Temperature_Microkelvin,
                UnitId::Temperature_Millikelvin,
                UnitId::Temperature_Kilokelvin,
                UnitId::Temperature_Celsius,
                UnitId::Temperature_Fahrenheit,
            ],
            UnitFamilyId::Amount => &[
                UnitId::Amount_Mole,
                UnitId::Amount_Picomole,
                UnitId::Amount_Nanomole,
                UnitId::Amount_Micromole,
                UnitId::Amount_Millimole,
                UnitId::Amount_Kilomole,
            ],
            UnitFamilyId::Frequency => &[
                UnitId::Frequency_Hertz,
                UnitId::Frequency_Picohertz,
                UnitId::Frequency_Nanohertz,
                UnitId::Frequency_Microhertz,
                UnitId::Frequency_Millihertz,
                UnitId::Frequency_Kilohertz,
                UnitId::Frequency_Megahertz,
                UnitId::Frequency_Gigahertz,
                UnitId::Frequency_Terahertz,
            ],
            UnitFamilyId::Angle => &[
                UnitId::Angle_Radian,
                UnitId::Angle_Picoradian,
                UnitId::Angle_Nanoradian,
                UnitId::Angle_Microradian,
                UnitId::Angle_Milliradian,
                UnitId::Angle_Kiloradian,
                UnitId::Angle_Megaradian,
                UnitId::Angle_Gigaradian,
                UnitId::Angle_Teraradian,
                UnitId::Angle_Degree,
                UnitId::Angle_Turn,
                UnitId::Angle_Arcminute,
                UnitId::Angle_Arcsecond,
            ],
            UnitFamilyId::SolidAngle => &[
                UnitId::SolidAngle_Steradian,
                UnitId::SolidAngle_Picosteradian,
                UnitId::SolidAngle_Nanosteradian,
                UnitId::SolidAngle_Microsteradian,
                UnitId::SolidAngle_Millisteradian,
            ],
            UnitFamilyId::Force => &[
                UnitId::Force_Newton,
                UnitId::Force_Piconewton,
                UnitId::Force_Nanonewton,
                UnitId::Force_Micronewton,
                UnitId::Force_Millinewton,
                UnitId::Force_Kilonewton,
                UnitId::Force_Meganewton,
                UnitId::Force_Giganewton,
                UnitId::Force_Teranewton,
                UnitId::Force_KilogramForce,
                UnitId::Force_PoundForce,
            ],
            UnitFamilyId::Pressure => &[
                UnitId::Pressure_Pascal,
                UnitId::Pressure_Picopascal,
                UnitId::Pressure_Nanopascal,
                UnitId::Pressure_Micropascal,
                UnitId::Pressure_Millipascal,
                UnitId::Pressure_Kilopascal,
                UnitId::Pressure_Megapascal,
                UnitId::Pressure_Gigapascal,
                UnitId::Pressure_Terapascal,
                UnitId::Pressure_Bar,
                UnitId::Pressure_Millibar,
                UnitId::Pressure_StandardAtmosphere,
                UnitId::Pressure_PoundPerSquareInch,
                UnitId::Pressure_MillimeterOfMercury,
            ],
            UnitFamilyId::Energy => &[
                UnitId::Energy_Joule,
                UnitId::Energy_Picojoule,
                UnitId::Energy_Nanojoule,
                UnitId::Energy_Microjoule,
                UnitId::Energy_Millijoule,
                UnitId::Energy_Kilojoule,
                UnitId::Energy_Megajoule,
                UnitId::Energy_Gigajoule,
                UnitId::Energy_Terajoule,
                UnitId::Energy_WattHour,
                UnitId::Energy_KilowattHour,
                UnitId::Energy_Electronvolt,
            ],
            UnitFamilyId::Power => &[
                UnitId::Power_Watt,
                UnitId::Power_Picowatt,
                UnitId::Power_Nanowatt,
                UnitId::Power_Microwatt,
                UnitId::Power_Milliwatt,
                UnitId::Power_Kilowatt,
                UnitId::Power_Megawatt,
                UnitId::Power_Gigawatt,
                UnitId::Power_Terawatt,
                UnitId::Power_MechanicalHorsepower,
                UnitId::Power_MetricHorsepower,
            ],
            UnitFamilyId::ElectricCharge => &[
                UnitId::ElectricCharge_Coulomb,
                UnitId::ElectricCharge_Picocoulomb,
                UnitId::ElectricCharge_Nanocoulomb,
                UnitId::ElectricCharge_Microcoulomb,
                UnitId::ElectricCharge_Millicoulomb,
                UnitId::ElectricCharge_Kilocoulomb,
                UnitId::ElectricCharge_Megacoulomb,
                UnitId::ElectricCharge_Gigacoulomb,
                UnitId::ElectricCharge_Teracoulomb,
                UnitId::ElectricCharge_AmpereHour,
                UnitId::ElectricCharge_MilliampereHour,
            ],
            UnitFamilyId::Voltage => &[
                UnitId::Voltage_Volt,
                UnitId::Voltage_Picovolt,
                UnitId::Voltage_Nanovolt,
                UnitId::Voltage_Microvolt,
                UnitId::Voltage_Millivolt,
                UnitId::Voltage_Kilovolt,
                UnitId::Voltage_Megavolt,
                UnitId::Voltage_Gigavolt,
                UnitId::Voltage_Teravolt,
            ],
            UnitFamilyId::Resistance => &[
                UnitId::Resistance_Ohm,
                UnitId::Resistance_Picoohm,
                UnitId::Resistance_Nanoohm,
                UnitId::Resistance_Microohm,
                UnitId::Resistance_Milliohm,
                UnitId::Resistance_Kiloohm,
                UnitId::Resistance_Megaohm,
                UnitId::Resistance_Gigaohm,
                UnitId::Resistance_Teraohm,
            ],
            UnitFamilyId::Conductance => &[
                UnitId::Conductance_Siemens,
                UnitId::Conductance_Picosiemens,
                UnitId::Conductance_Nanosiemens,
                UnitId::Conductance_Microsiemens,
                UnitId::Conductance_Millisiemens,
                UnitId::Conductance_Kilosiemens,
                UnitId::Conductance_Megasiemens,
                UnitId::Conductance_Gigasiemens,
                UnitId::Conductance_Terasiemens,
            ],
            UnitFamilyId::Capacitance => &[
                UnitId::Capacitance_Farad,
                UnitId::Capacitance_Picofarad,
                UnitId::Capacitance_Nanofarad,
                UnitId::Capacitance_Microfarad,
                UnitId::Capacitance_Millifarad,
                UnitId::Capacitance_Kilofarad,
            ],
            UnitFamilyId::Inductance => &[
                UnitId::Inductance_Henry,
                UnitId::Inductance_Picohenry,
                UnitId::Inductance_Nanohenry,
                UnitId::Inductance_Microhenry,
                UnitId::Inductance_Millihenry,
            ],
            UnitFamilyId::MagneticFluxDensity => &[
                UnitId::MagneticFluxDensity_Tesla,
                UnitId::MagneticFluxDensity_Picotesla,
                UnitId::MagneticFluxDensity_Nanotesla,
                UnitId::MagneticFluxDensity_Microtesla,
                UnitId::MagneticFluxDensity_Millitesla,
            ],
            UnitFamilyId::MagneticFlux => &[
                UnitId::MagneticFlux_Weber,
                UnitId::MagneticFlux_Picoweber,
                UnitId::MagneticFlux_Nanoweber,
                UnitId::MagneticFlux_Microweber,
                UnitId::MagneticFlux_Milliweber,
                UnitId::MagneticFlux_Kiloweber,
                UnitId::MagneticFlux_Megaweber,
                UnitId::MagneticFlux_Gigaweber,
                UnitId::MagneticFlux_Teraweber,
            ],
            UnitFamilyId::Area => &[
                UnitId::Area_SquareMeter,
                UnitId::Area_SquareMicroMeter,
                UnitId::Area_SquareMilliMeter,
                UnitId::Area_SquareCentiMeter,
                UnitId::Area_SquareKiloMeter,
                UnitId::Area_SquareFoot,
                UnitId::Area_SquareInch,
                UnitId::Area_Acre,
                UnitId::Area_Hectare,
                UnitId::Area_SquareMile,
            ],
            UnitFamilyId::Volume => &[
                UnitId::Volume_CubicMeter,
                UnitId::Volume_CubicCentiMeter,
                UnitId::Volume_CubicMilliMeter,
                UnitId::Volume_Liter,
                UnitId::Volume_Picoliter,
                UnitId::Volume_Nanoliter,
                UnitId::Volume_Microliter,
                UnitId::Volume_Milliliter,
                UnitId::Volume_Kiloliter,
                UnitId::Volume_Megaliter,
                UnitId::Volume_Gallon,
                UnitId::Volume_ImperialGallon,
                UnitId::Volume_FluidOunce,
                UnitId::Volume_ImperialFluidOunce,
                UnitId::Volume_Cup,
                UnitId::Volume_Pint,
                UnitId::Volume_Quart,
            ],
            UnitFamilyId::Speed => &[
                UnitId::Speed_MeterPerSecond,
                UnitId::Speed_MillimeterPerSecond,
                UnitId::Speed_CentimeterPerSecond,
                UnitId::Speed_KilometerPerHour,
                UnitId::Speed_InchPerSecond,
                UnitId::Speed_FootPerSecond,
                UnitId::Speed_MilePerHour,
                UnitId::Speed_Knot,
            ],
            UnitFamilyId::Acceleration => &[
                UnitId::Acceleration_MeterPerSecondSquared,
                UnitId::Acceleration_MillimeterPerSecondSquared,
                UnitId::Acceleration_CentimeterPerSecondSquared,
                UnitId::Acceleration_InchPerSecondSquared,
                UnitId::Acceleration_FootPerSecondSquared,
                UnitId::Acceleration_StandardGravity,
            ],
            UnitFamilyId::AngularVelocity => &[
                UnitId::AngularVelocity_RadianPerSecond,
                UnitId::AngularVelocity_DegreePerSecond,
                UnitId::AngularVelocity_RevolutionPerMinute,
                UnitId::AngularVelocity_RevolutionPerSecond,
            ],
            UnitFamilyId::Torque => &[
                UnitId::Torque_NewtonMeter,
                UnitId::Torque_NewtonMillimeter,
                UnitId::Torque_NewtonCentimeter,
                UnitId::Torque_PoundForceInch,
                UnitId::Torque_PoundForceFoot,
            ],
            UnitFamilyId::Density => &[
                UnitId::Density_KilogramPerCubicMeter,
                UnitId::Density_GramPerCubicCentimeter,
                UnitId::Density_GramPerLiter,
                UnitId::Density_KilogramPerLiter,
                UnitId::Density_PoundPerCubicFoot,
            ],
            UnitFamilyId::VolumeFlowRate => &[
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                UnitId::VolumeFlowRate_LiterPerMinute,
            ],
            UnitFamilyId::DisplacementPerRevolution => &[
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
                UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution,
            ],
            UnitFamilyId::AngularFrequency => &[
                UnitId::AngularFrequency_RadianPerSecond,
                UnitId::AngularFrequency_PicoradianPerSecond,
                UnitId::AngularFrequency_NanoradianPerSecond,
                UnitId::AngularFrequency_MicroradianPerSecond,
                UnitId::AngularFrequency_MilliradianPerSecond,
                UnitId::AngularFrequency_KiloradianPerSecond,
                UnitId::AngularFrequency_MegaradianPerSecond,
                UnitId::AngularFrequency_GigaradianPerSecond,
                UnitId::AngularFrequency_TeraradianPerSecond,
            ],
            UnitFamilyId::MomentOfInertia => &[
                UnitId::MomentOfInertia_KilogramSquareMeter,
                UnitId::MomentOfInertia_KilogramSquareCentimeter,
                UnitId::MomentOfInertia_GramSquareCentimeter,
            ],
            UnitFamilyId::LinearStiffness => &[
                UnitId::LinearStiffness_NewtonPerMeter,
                UnitId::LinearStiffness_NewtonPerMillimeter,
                UnitId::LinearStiffness_KilonewtonPerMeter,
            ],
            UnitFamilyId::LinearDamping => &[
                UnitId::LinearDamping_NewtonSecondPerMeter,
                UnitId::LinearDamping_NewtonSecondPerMillimeter,
                UnitId::LinearDamping_KilonewtonSecondPerMeter,
            ],
            UnitFamilyId::RotationalStiffness => &[
                UnitId::RotationalStiffness_NewtonMeterPerRadian,
                UnitId::RotationalStiffness_NewtonMillimeterPerRadian,
                UnitId::RotationalStiffness_KilonewtonMeterPerRadian,
            ],
            UnitFamilyId::RotationalDamping => &[
                UnitId::RotationalDamping_NewtonMeterSecondPerRadian,
                UnitId::RotationalDamping_NewtonMillimeterSecondPerRadian,
                UnitId::RotationalDamping_KilonewtonMeterSecondPerRadian,
            ],
            UnitFamilyId::DynamicViscosity => &[
                UnitId::DynamicViscosity_PascalSecond,
                UnitId::DynamicViscosity_MillipascalSecond,
                UnitId::DynamicViscosity_Poise,
                UnitId::DynamicViscosity_Centipoise,
            ],
            UnitFamilyId::KinematicViscosity => &[
                UnitId::KinematicViscosity_SquareMeterPerSecond,
                UnitId::KinematicViscosity_SquareMillimeterPerSecond,
                UnitId::KinematicViscosity_Stokes,
                UnitId::KinematicViscosity_Centistokes,
            ],
            UnitFamilyId::MassFlowRate => &[
                UnitId::MassFlowRate_KilogramPerSecond,
                UnitId::MassFlowRate_GramPerSecond,
                UnitId::MassFlowRate_KilogramPerMinute,
                UnitId::MassFlowRate_KilogramPerHour,
            ],
            UnitFamilyId::Momentum => &[
                UnitId::Momentum_KilogramMeterPerSecond,
                UnitId::Momentum_GramCentimeterPerSecond,
                UnitId::Momentum_NewtonSecond,
            ],
            UnitFamilyId::HydraulicLeakageCoefficient => &[
                UnitId::HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal,
                UnitId::HydraulicLeakageCoefficient_LiterPerMinutePerBar,
            ],
            UnitFamilyId::SpecificHeatCapacity => &[
                UnitId::SpecificHeatCapacity_JoulePerKilogramKelvin,
                UnitId::SpecificHeatCapacity_KilojoulePerKilogramKelvin,
            ],
            UnitFamilyId::SpecificGasConstant => &[
                UnitId::SpecificGasConstant_JoulePerKilogramKelvin,
                UnitId::SpecificGasConstant_KilojoulePerKilogramKelvin,
            ],
            UnitFamilyId::ThermalConductance => &[
                UnitId::ThermalConductance_WattPerKelvin,
                UnitId::ThermalConductance_MilliwattPerKelvin,
                UnitId::ThermalConductance_KilowattPerKelvin,
            ],
            UnitFamilyId::HydraulicResistance => &[
                UnitId::HydraulicResistance_PascalSecondPerCubicMeter,
                UnitId::HydraulicResistance_BarMinutePerLiter,
            ],
            UnitFamilyId::PneumaticCharacteristicImpedance => &[
                UnitId::PneumaticCharacteristicImpedance_PascalSecondPerJoule,
                UnitId::PneumaticCharacteristicImpedance_SecondPerCubicMeter,
            ],
            UnitFamilyId::TurbulentFlowCoefficient => &[
                UnitId::TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal,
                UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar,
            ],
            UnitFamilyId::MotorBackEmfConstant => &[
                UnitId::MotorBackEmfConstant_VoltSecondPerRadian,
                UnitId::MotorBackEmfConstant_MillivoltSecondPerRadian,
                UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute,
            ],
            UnitFamilyId::ScrewPitch => &[
                UnitId::ScrewPitch_MeterPerRadian,
                UnitId::ScrewPitch_MillimeterPerRadian,
                UnitId::ScrewPitch_MillimeterPerRevolution,
            ],
            UnitFamilyId::ThrustSpecificFuelConsumption => &[
                UnitId::ThrustSpecificFuelConsumption_KilogramPerNewtonSecond,
                UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour,
            ],
            UnitFamilyId::VolumeFlowAcceleration => &[
                UnitId::VolumeFlowAcceleration_CubicMeterPerSecondSquared,
                UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond,
            ],
        }
    }
}

/// Defines [`UnitId`] variants and their associated metadata.
macro_rules! define_unit_ids {
    ($(
        $unit:ident = $value:expr => (
            $family:ident,
            $key:literal,
            $description:literal,
            $documentation:literal
        ),
    )*) => {
        /// Identifiers for the units supported by Hrafnix.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u16)]
        #[allow(non_camel_case_types)]
        pub enum UnitId {
            $(
                #[doc = $documentation]
                $unit = $value,
            )*
        }

        impl UnitId {
            /// All supported unit identifiers.
            pub const ALL: &[Self] = &[
                $(
                    Self::$unit,
                )*
            ];

            /// Returns the `UnitId` corresponding to the given string identifier.
            #[cfg_attr(feature = "hotpath", hotpath::measure)]
            pub fn from_unit_id_str(unit_id_str: &str) -> Option<Self> {
                match unit_id_str {
                    $(
                        $key => Some(Self::$unit),
                    )*
                    _ => None,
                }
            }

            /// Returns the `UnitId` corresponding to the given u16 value.
            pub const fn from_u16(value: u16) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Self::$unit),
                    )*
                    _ => None,
                }
            }

            /// Returns the u16 value corresponding to the given `UnitId`.
            pub const fn to_u16(&self) -> u16 {
                match self {
                    $(
                        Self::$unit => $value,
                    )*
                }
            }

            /// Returns the `UnitFamilyId` corresponding to the given `UnitId`.
            pub const fn family_id(&self) -> UnitFamilyId {
                match self {
                    $(
                        Self::$unit => UnitFamilyId::$family,
                    )*
                }
            }

            /// Returns the string identifier corresponding to the given `UnitId`.
            pub const fn string_id(&self) -> ConstUnitKey {
                match self {
                    $(
                        Self::$unit => unit_key!($key),
                    )*
                }
            }

            /// Returns the description corresponding to the given `UnitId`.
            pub const fn description(&self) -> &'static str {
                match self {
                    $(
                        Self::$unit => $description,
                    )*
                }
            }

            /// Returns the documentation corresponding to the given `UnitId`.
            #[cfg(test)]
            const fn documentation(&self) -> &'static str {
                match self {
                    $(
                        Self::$unit => $documentation,
                    )*
                }
            }
        }
    };
}

define_unit_ids! {
    None = 0 => (None, "u_none", "", "No unit."),

    Length_Meter = 100 => (Length, "u_length_meter", "m", "A meter."),
    Length_Picometer = 101 => (Length, "u_length_picometer", "pm", "A picometer."),
    Length_Nanometer = 102 => (Length, "u_length_nanometer", "nm", "A nanometer."),
    Length_Micrometer = 103 => (Length, "u_length_micrometer", "μm", "A micrometer."),
    Length_Millimeter = 104 => (Length, "u_length_millimeter", "mm", "A millimeter."),
    Length_Centimeter = 105 => (Length, "u_length_centimeter", "cm", "A centimeter."),
    Length_Kilometer = 106 => (Length, "u_length_kilometer", "km", "A kilometer."),
    Length_Foot = 107 => (Length, "u_length_foot", "ft", "An international foot."),
    Length_Inch = 108 => (Length, "u_length_inch", "in", "An international inch."),
    Length_Yard = 109 => (Length, "u_length_yard", "yd", "An international yard."),
    Length_Mile = 110 => (Length, "u_length_mile", "mi", "An international mile."),

    Mass_Kilogram = 200 => (Mass, "u_mass_kilogram", "kg", "A kilogram."),
    Mass_Gram = 201 => (Mass, "u_mass_gram", "g", "A gram."),
    Mass_Picogram = 202 => (Mass, "u_mass_picogram", "pg", "A picogram."),
    Mass_Nanogram = 203 => (Mass, "u_mass_nanogram", "ng", "A nanogram."),
    Mass_Microgram = 204 => (Mass, "u_mass_microgram", "μg", "A microgram."),
    Mass_Milligram = 205 => (Mass, "u_mass_milligram", "mg", "A milligram."),
    Mass_Megagram = 206 => (Mass, "u_mass_megagram", "Mg", "A megagram."),
    Mass_Tonne = 207 => (Mass, "u_mass_tonne", "t", "A metric tonne."),
    Mass_Pound = 208 => (Mass, "u_mass_pound", "lb", "An international avoirdupois pound."),
    Mass_Ounce = 209 => (Mass, "u_mass_ounce", "oz", "An international avoirdupois ounce."),
    Mass_Stone = 210 => (Mass, "u_mass_stone", "st", "A stone."),

    Time_Second = 300 => (Time, "u_time_second", "s", "A second."),
    Time_PicoSecond = 301 => (Time, "u_time_picosecond", "ps", "A picosecond."),
    Time_NanoSecond = 302 => (Time, "u_time_nanosecond", "ns", "A nanosecond."),
    Time_MicroSecond = 303 => (Time, "u_time_microsecond", "μs", "A microsecond."),
    Time_MilliSecond = 304 => (Time, "u_time_millisecond", "ms", "A millisecond."),
    Time_KiloSecond = 305 => (Time, "u_time_kilosecond", "ks", "A kilosecond."),
    Time_Minute = 306 => (Time, "u_time_minute", "min", "A minute."),
    Time_Hour = 307 => (Time, "u_time_hour", "h", "An hour."),
    Time_Day = 308 => (Time, "u_time_day", "day", "A day."),
    Time_Week = 309 => (Time, "u_time_week", "week", "A week."),
    Time_Year = 310 => (Time, "u_time_year", "year", "A common year of 365 days."),

    Current_Ampere = 400 => (Current, "u_current_ampere", "A", "An ampere."),
    Current_Nanoampere = 401 => (Current, "u_current_nanoampere", "nA", "A nanoampere."),
    Current_Microampere = 402 => (Current, "u_current_microampere", "μA", "A microampere."),
    Current_Milliampere = 403 => (Current, "u_current_milliampere", "mA", "A milliampere."),
    Current_Kiloampere = 404 => (Current, "u_current_kiloampere", "kA", "A kiloampere."),
    Current_Megaampere = 405 => (Current, "u_current_megaampere", "MA", "A megaampere."),

    Temperature_Kelvin = 500 => (Temperature, "u_temperature_kelvin", "K", "A kelvin."),
    Temperature_Nanokelvin = 501 => (Temperature, "u_temperature_nanokelvin", "nK", "A nanokelvin."),
    Temperature_Microkelvin = 502 => (Temperature, "u_temperature_microkelvin", "μK", "A microkelvin."),
    Temperature_Millikelvin = 503 => (Temperature, "u_temperature_millikelvin", "mK", "A millikelvin."),
    Temperature_Kilokelvin = 504 => (Temperature, "u_temperature_kilokelvin", "kK", "A kilokelvin."),
    Temperature_Celsius = 505 => (Temperature, "u_temperature_celsius", "°C", "A degree Celsius, the SI unit for temperature relative to 273.15 K. A temperature interval of one degree Celsius equals one kelvin."),
    Temperature_Fahrenheit = 506 => (Temperature, "u_temperature_fahrenheit", "°F", "A degree Fahrenheit."),

    Amount_Mole = 600 => (Amount, "u_amount_mole", "mol", "A mole."),
    Amount_Picomole = 601 => (Amount, "u_amount_picomole", "pmol", "A picomole."),
    Amount_Nanomole = 602 => (Amount, "u_amount_nanomole", "nmol", "A nanomole."),
    Amount_Micromole = 603 => (Amount, "u_amount_micromole", "μmol", "A micromole."),
    Amount_Millimole = 604 => (Amount, "u_amount_millimole", "mmol", "A millimole."),
    Amount_Kilomole = 605 => (Amount, "u_amount_kilomole", "kmol", "A kilomole."),

    Frequency_Hertz = 700 => (Frequency, "u_frequency_hertz", "Hz", "A hertz, the SI unit of frequency."),
    Frequency_Picohertz = 701 => (Frequency, "u_frequency_picohertz", "pHz", "A picohertz."),
    Frequency_Nanohertz = 702 => (Frequency, "u_frequency_nanohertz", "nHz", "A nanohertz."),
    Frequency_Microhertz = 703 => (Frequency, "u_frequency_microhertz", "μHz", "A microhertz."),
    Frequency_Millihertz = 704 => (Frequency, "u_frequency_millihertz", "mHz", "A millihertz."),
    Frequency_Kilohertz = 705 => (Frequency, "u_frequency_kilohertz", "kHz", "A kilohertz."),
    Frequency_Megahertz = 706 => (Frequency, "u_frequency_megahertz", "MHz", "A megahertz."),
    Frequency_Gigahertz = 707 => (Frequency, "u_frequency_gigahertz", "GHz", "A gigahertz."),
    Frequency_Terahertz = 708 => (Frequency, "u_frequency_terahertz", "THz", "A terahertz."),

    Angle_Radian = 800 => (Angle, "u_angle_radian", "rad", "A radian, the SI unit of plane angle."),
    Angle_Picoradian = 801 => (Angle, "u_angle_picoradian", "prad", "A picoradian."),
    Angle_Nanoradian = 802 => (Angle, "u_angle_nanoradian", "nrad", "A nanoradian."),
    Angle_Microradian = 803 => (Angle, "u_angle_microradian", "μrad", "A microradian."),
    Angle_Milliradian = 804 => (Angle, "u_angle_milliradian", "mrad", "A milliradian."),
    Angle_Kiloradian = 805 => (Angle, "u_angle_kiloradian", "krad", "A kiloradian."),
    Angle_Megaradian = 806 => (Angle, "u_angle_megaradian", "Mrad", "A megaradian."),
    Angle_Gigaradian = 807 => (Angle, "u_angle_gigaradian", "Grad", "A gigaradian."),
    Angle_Teraradian = 808 => (Angle, "u_angle_teraradian", "Trad", "A teraradian."),
    Angle_Degree = 809 => (Angle, "u_angle_degree", "°", "A degree of plane angle."),
    Angle_Turn = 810 => (Angle, "u_angle_turn", "rev", "A turn or revolution."),
    Angle_Arcminute = 811 => (Angle, "u_angle_arcminute", "′", "An arcminute."),
    Angle_Arcsecond = 812 => (Angle, "u_angle_arcsecond", "″", "An arcsecond."),

    SolidAngle_Steradian = 900 => (SolidAngle, "u_solid_angle_steradian", "sr", "A steradian, the SI unit of solid angle."),
    SolidAngle_Picosteradian = 901 => (SolidAngle, "u_solid_angle_picosteradian", "psr", "A picosteradian."),
    SolidAngle_Nanosteradian = 902 => (SolidAngle, "u_solid_angle_nanosteradian", "nsr", "A nanosteradian."),
    SolidAngle_Microsteradian = 903 => (SolidAngle, "u_solid_angle_microsteradian", "μsr", "A microsteradian."),
    SolidAngle_Millisteradian = 904 => (SolidAngle, "u_solid_angle_millisteradian", "msr", "A millisteradian."),

    Force_Newton = 1000 => (Force, "u_force_newton", "N", "A newton, the SI unit of force or weight."),
    Force_Piconewton = 1001 => (Force, "u_force_piconewton", "pN", "A piconewton."),
    Force_Nanonewton = 1002 => (Force, "u_force_nanonewton", "nN", "A nanonewton."),
    Force_Micronewton = 1003 => (Force, "u_force_micronewton", "μN", "A micronewton."),
    Force_Millinewton = 1004 => (Force, "u_force_millinewton", "mN", "A millinewton."),
    Force_Kilonewton = 1005 => (Force, "u_force_kilonewton", "kN", "A kilonewton."),
    Force_Meganewton = 1006 => (Force, "u_force_meganewton", "MN", "A meganewton."),
    Force_Giganewton = 1007 => (Force, "u_force_giganewton", "GN", "A giganewton."),
    Force_Teranewton = 1008 => (Force, "u_force_teranewton", "TN", "A teranewton."),
    Force_KilogramForce = 1009 => (Force, "u_force_kilogram_force", "kgf", "A kilogram-force."),
    Force_PoundForce = 1010 => (Force, "u_force_pound_force", "lbf", "A pound-force."),

    Pressure_Pascal = 1100 => (Pressure, "u_pressure_pascal", "Pa", "A pascal, the SI unit of pressure or stress."),
    Pressure_Picopascal = 1101 => (Pressure, "u_pressure_picopascal", "pPa", "A picopascal."),
    Pressure_Nanopascal = 1102 => (Pressure, "u_pressure_nanopascal", "nPa", "A nanopascal."),
    Pressure_Micropascal = 1103 => (Pressure, "u_pressure_micropascal", "μPa", "A micropascal."),
    Pressure_Millipascal = 1104 => (Pressure, "u_pressure_millipascal", "mPa", "A millipascal."),
    Pressure_Kilopascal = 1105 => (Pressure, "u_pressure_kilopascal", "kPa", "A kilopascal."),
    Pressure_Megapascal = 1106 => (Pressure, "u_pressure_megapascal", "MPa", "A megapascal."),
    Pressure_Gigapascal = 1107 => (Pressure, "u_pressure_gigapascal", "GPa", "A gigapascal."),
    Pressure_Terapascal = 1108 => (Pressure, "u_pressure_terapascal", "TPa", "A terapascal."),
    Pressure_Bar = 1109 => (Pressure, "u_pressure_bar", "bar", "A bar."),
    Pressure_Millibar = 1110 => (Pressure, "u_pressure_millibar", "mbar", "A millibar."),
    Pressure_StandardAtmosphere = 1111 => (Pressure, "u_pressure_standard_atmosphere", "atm", "A standard atmosphere."),
    Pressure_PoundPerSquareInch = 1112 => (Pressure, "u_pressure_pound_per_square_inch", "psi", "A pound-force per square inch, using the international inch and standard gravity."),
    Pressure_MillimeterOfMercury = 1113 => (Pressure, "u_pressure_millimeter_of_mercury", "mmHg", "A conventional millimeter of mercury."),

    Energy_Joule = 1200 => (Energy, "u_energy_joule", "J", "A joule, the SI unit of energy, work, or heat."),
    Energy_Picojoule = 1201 => (Energy, "u_energy_picojoule", "pJ", "A picojoule."),
    Energy_Nanojoule = 1202 => (Energy, "u_energy_nanojoule", "nJ", "A nanojoule."),
    Energy_Microjoule = 1203 => (Energy, "u_energy_microjoule", "μJ", "A microjoule."),
    Energy_Millijoule = 1204 => (Energy, "u_energy_millijoule", "mJ", "A millijoule."),
    Energy_Kilojoule = 1205 => (Energy, "u_energy_kilojoule", "kJ", "A kilojoule."),
    Energy_Megajoule = 1206 => (Energy, "u_energy_megajoule", "MJ", "A megajoule."),
    Energy_Gigajoule = 1207 => (Energy, "u_energy_gigajoule", "GJ", "A gigajoule."),
    Energy_Terajoule = 1208 => (Energy, "u_energy_terajoule", "TJ", "A terajoule."),
    Energy_WattHour = 1209 => (Energy, "u_energy_watt_hour", "Wh", "A watt-hour."),
    Energy_KilowattHour = 1210 => (Energy, "u_energy_kilowatt_hour", "kWh", "A kilowatt-hour."),
    Energy_Electronvolt = 1211 => (Energy, "u_energy_electronvolt", "eV", "An electronvolt."),

    Power_Watt = 1300 => (Power, "u_power_watt", "W", "A watt, the SI unit of power or radiant flux."),
    Power_Picowatt = 1301 => (Power, "u_power_picowatt", "pW", "A picowatt."),
    Power_Nanowatt = 1302 => (Power, "u_power_nanowatt", "nW", "A nanowatt."),
    Power_Microwatt = 1303 => (Power, "u_power_microwatt", "μW", "A microwatt."),
    Power_Milliwatt = 1304 => (Power, "u_power_milliwatt", "mW", "A milliwatt."),
    Power_Kilowatt = 1305 => (Power, "u_power_kilowatt", "kW", "A kilowatt."),
    Power_Megawatt = 1306 => (Power, "u_power_megawatt", "MW", "A megawatt."),
    Power_Gigawatt = 1307 => (Power, "u_power_gigawatt", "GW", "A gigawatt."),
    Power_Terawatt = 1308 => (Power, "u_power_terawatt", "TW", "A terawatt."),
    Power_MechanicalHorsepower = 1309 => (Power, "u_power_mechanical_horsepower", "hp", "A mechanical horsepower."),
    Power_MetricHorsepower = 1310 => (Power, "u_power_metric_horsepower", "PS", "A metric horsepower."),

    ElectricCharge_Coulomb = 1400 => (ElectricCharge, "u_electric_charge_coulomb", "C", "A coulomb, the SI unit of electric charge."),
    ElectricCharge_Picocoulomb = 1401 => (ElectricCharge, "u_electric_charge_picocoulomb", "pC", "A picocoulomb."),
    ElectricCharge_Nanocoulomb = 1402 => (ElectricCharge, "u_electric_charge_nanocoulomb", "nC", "A nanocoulomb."),
    ElectricCharge_Microcoulomb = 1403 => (ElectricCharge, "u_electric_charge_microcoulomb", "μC", "A microcoulomb."),
    ElectricCharge_Millicoulomb = 1404 => (ElectricCharge, "u_electric_charge_millicoulomb", "mC", "A millicoulomb."),
    ElectricCharge_Kilocoulomb = 1405 => (ElectricCharge, "u_electric_charge_kilocoulomb", "kC", "A kilocoulomb."),
    ElectricCharge_Megacoulomb = 1406 => (ElectricCharge, "u_electric_charge_megacoulomb", "MC", "A megacoulomb."),
    ElectricCharge_Gigacoulomb = 1407 => (ElectricCharge, "u_electric_charge_gigacoulomb", "GC", "A gigacoulomb."),
    ElectricCharge_Teracoulomb = 1408 => (ElectricCharge, "u_electric_charge_teracoulomb", "TC", "A teracoulomb."),
    ElectricCharge_AmpereHour = 1409 => (ElectricCharge, "u_electric_charge_ampere_hour", "Ah", "An ampere-hour."),
    ElectricCharge_MilliampereHour = 1410 => (ElectricCharge, "u_electric_charge_milliampere_hour", "mAh", "A milliampere-hour."),

    Voltage_Volt = 1500 => (Voltage, "u_voltage_volt", "V", "A volt, the SI unit of voltage, electric potential, or electromotive force."),
    Voltage_Picovolt = 1501 => (Voltage, "u_voltage_picovolt", "pV", "A picovolt."),
    Voltage_Nanovolt = 1502 => (Voltage, "u_voltage_nanovolt", "nV", "A nanovolt."),
    Voltage_Microvolt = 1503 => (Voltage, "u_voltage_microvolt", "μV", "A microvolt."),
    Voltage_Millivolt = 1504 => (Voltage, "u_voltage_millivolt", "mV", "A millivolt."),
    Voltage_Kilovolt = 1505 => (Voltage, "u_voltage_kilovolt", "kV", "A kilovolt."),
    Voltage_Megavolt = 1506 => (Voltage, "u_voltage_megavolt", "MV", "A megavolt."),
    Voltage_Gigavolt = 1507 => (Voltage, "u_voltage_gigavolt", "GV", "A gigavolt."),
    Voltage_Teravolt = 1508 => (Voltage, "u_voltage_teravolt", "TV", "A teravolt."),

    Resistance_Ohm = 1600 => (Resistance, "u_resistance_ohm", "Ω", "An ohm, the SI unit of electrical resistance, reactance, or impedance."),
    Resistance_Picoohm = 1601 => (Resistance, "u_resistance_picoohm", "pΩ", "A picoohm."),
    Resistance_Nanoohm = 1602 => (Resistance, "u_resistance_nanoohm", "nΩ", "A nanoohm."),
    Resistance_Microohm = 1603 => (Resistance, "u_resistance_microohm", "μΩ", "A microohm."),
    Resistance_Milliohm = 1604 => (Resistance, "u_resistance_milliohm", "mΩ", "A milliohm."),
    Resistance_Kiloohm = 1605 => (Resistance, "u_resistance_kiloohm", "kΩ", "A kiloohm."),
    Resistance_Megaohm = 1606 => (Resistance, "u_resistance_megaohm", "MΩ", "A megaohm."),
    Resistance_Gigaohm = 1607 => (Resistance, "u_resistance_gigaohm", "GΩ", "A gigaohm."),
    Resistance_Teraohm = 1608 => (Resistance, "u_resistance_teraohm", "TΩ", "A teraohm."),

    Conductance_Siemens = 1700 => (Conductance, "u_conductance_siemens", "S", "A siemens, the SI unit of electrical conductance, susceptance, or admittance."),
    Conductance_Picosiemens = 1701 => (Conductance, "u_conductance_picosiemens", "pS", "A picosiemens."),
    Conductance_Nanosiemens = 1702 => (Conductance, "u_conductance_nanosiemens", "nS", "A nanosiemens."),
    Conductance_Microsiemens = 1703 => (Conductance, "u_conductance_microsiemens", "μS", "A microsiemens."),
    Conductance_Millisiemens = 1704 => (Conductance, "u_conductance_millisiemens", "mS", "A millisiemens."),
    Conductance_Kilosiemens = 1705 => (Conductance, "u_conductance_kilosiemens", "kS", "A kilosiemens."),
    Conductance_Megasiemens = 1706 => (Conductance, "u_conductance_megasiemens", "MS", "A megasiemens."),
    Conductance_Gigasiemens = 1707 => (Conductance, "u_conductance_gigasiemens", "GS", "A gigasiemens."),
    Conductance_Terasiemens = 1708 => (Conductance, "u_conductance_terasiemens", "TS", "A terasiemens."),

    Capacitance_Farad = 1800 => (Capacitance, "u_capacitance_farad", "F", "A farad, the SI unit of capacitance."),
    Capacitance_Picofarad = 1801 => (Capacitance, "u_capacitance_picofarad", "pF", "A picofarad."),
    Capacitance_Nanofarad = 1802 => (Capacitance, "u_capacitance_nanofarad", "nF", "A nanofarad."),
    Capacitance_Microfarad = 1803 => (Capacitance, "u_capacitance_microfarad", "μF", "A microfarad."),
    Capacitance_Millifarad = 1804 => (Capacitance, "u_capacitance_millifarad", "mF", "A millifarad."),
    Capacitance_Kilofarad = 1805 => (Capacitance, "u_capacitance_kilofarad", "kF", "A kilofarad."),

    Inductance_Henry = 1900 => (Inductance, "u_inductance_henry", "H", "A henry, the SI unit of inductance or permeance."),
    Inductance_Picohenry = 1901 => (Inductance, "u_inductance_picohenry", "pH", "A picohenry."),
    Inductance_Nanohenry = 1902 => (Inductance, "u_inductance_nanohenry", "nH", "A nanohenry."),
    Inductance_Microhenry = 1903 => (Inductance, "u_inductance_microhenry", "μH", "A microhenry."),
    Inductance_Millihenry = 1904 => (Inductance, "u_inductance_millihenry", "mH", "A millihenry."),

    MagneticFluxDensity_Tesla = 2000 => (MagneticFluxDensity, "u_magnetic_flux_density_tesla", "T", "A tesla, the SI unit of magnetic flux density."),
    MagneticFluxDensity_Picotesla = 2001 => (MagneticFluxDensity, "u_magnetic_flux_density_picotesla", "pT", "A picotesla."),
    MagneticFluxDensity_Nanotesla = 2002 => (MagneticFluxDensity, "u_magnetic_flux_density_nanotesla", "nT", "A nanotesla."),
    MagneticFluxDensity_Microtesla = 2003 => (MagneticFluxDensity, "u_magnetic_flux_density_microtesla", "μT", "A microtesla."),
    MagneticFluxDensity_Millitesla = 2004 => (MagneticFluxDensity, "u_magnetic_flux_density_millitesla", "mT", "A millitesla."),

    MagneticFlux_Weber = 2100 => (MagneticFlux, "u_magnetic_flux_weber", "Wb", "A weber, the SI unit of magnetic flux."),
    MagneticFlux_Picoweber = 2101 => (MagneticFlux, "u_magnetic_flux_picoweber", "pWb", "A picoweber."),
    MagneticFlux_Nanoweber = 2102 => (MagneticFlux, "u_magnetic_flux_nanoweber", "nWb", "A nanoweber."),
    MagneticFlux_Microweber = 2103 => (MagneticFlux, "u_magnetic_flux_microweber", "μWb", "A microweber."),
    MagneticFlux_Milliweber = 2104 => (MagneticFlux, "u_magnetic_flux_milliweber", "mWb", "A milliweber."),
    MagneticFlux_Kiloweber = 2105 => (MagneticFlux, "u_magnetic_flux_kiloweber", "kWb", "A kiloweber."),
    MagneticFlux_Megaweber = 2106 => (MagneticFlux, "u_magnetic_flux_megaweber", "MWb", "A megaweber."),
    MagneticFlux_Gigaweber = 2107 => (MagneticFlux, "u_magnetic_flux_gigaweber", "GWb", "A gigaweber."),
    MagneticFlux_Teraweber = 2108 => (MagneticFlux, "u_magnetic_flux_teraweber", "TWb", "A teraweber."),

    Area_SquareMeter = 2200 => (Area, "u_area_square_meter", "m²", "A square meter."),
    Area_SquareMicroMeter = 2201 => (Area, "u_area_square_micrometer", "μm²", "A square micrometer."),
    Area_SquareMilliMeter = 2202 => (Area, "u_area_square_millimeter", "mm²", "A square millimeter."),
    Area_SquareCentiMeter = 2203 => (Area, "u_area_square_centimeter", "cm²", "A square centimeter."),
    Area_SquareKiloMeter = 2204 => (Area, "u_area_square_kilometer", "km²", "A square kilometer."),
    Area_SquareFoot = 2205 => (Area, "u_area_square_foot", "ft²", "A square foot."),
    Area_SquareInch = 2206 => (Area, "u_area_square_inch", "in²", "A square inch."),
    Area_Acre = 2207 => (Area, "u_area_acre", "ac", "An international acre."),
    Area_Hectare = 2208 => (Area, "u_area_hectare", "ha", "A hectare."),
    Area_SquareMile = 2209 => (Area, "u_area_square_mile", "mi²", "A square international mile."),

    Volume_CubicMeter = 2300 => (Volume, "u_volume_cubic_meter", "m³", "A cubic meter, the SI base for volume conversions."),
    Volume_CubicCentiMeter = 2301 => (Volume, "u_volume_cubic_centimeter", "cm³", "A cubic centimeter."),
    Volume_CubicMilliMeter = 2302 => (Volume, "u_volume_cubic_millimeter", "mm³", "A cubic millimeter."),
    Volume_Liter = 2303 => (Volume, "u_volume_liter", "l", "A liter."),
    Volume_Picoliter = 2304 => (Volume, "u_volume_picoliter", "pl", "A picoliter."),
    Volume_Nanoliter = 2305 => (Volume, "u_volume_nanoliter", "nl", "A nanoliter."),
    Volume_Microliter = 2306 => (Volume, "u_volume_microliter", "μl", "A microliter."),
    Volume_Milliliter = 2307 => (Volume, "u_volume_milliliter", "ml", "A milliliter."),
    Volume_Kiloliter = 2308 => (Volume, "u_volume_kiloliter", "kl", "A kiloliter."),
    Volume_Megaliter = 2309 => (Volume, "u_volume_megaliter", "Ml", "A megaliter."),
    Volume_Gallon = 2310 => (Volume, "u_volume_gallon", "gal", "A US liquid gallon."),
    Volume_ImperialGallon = 2311 => (Volume, "u_volume_imperial_gallon", "gal_uk", "An imperial gallon."),
    Volume_FluidOunce = 2312 => (Volume, "u_volume_fluid_ounce", "fl_oz", "A US fluid ounce."),
    Volume_ImperialFluidOunce = 2313 => (Volume, "u_volume_imperial_fluid_ounce", "fl_oz_uk", "An imperial fluid ounce."),
    Volume_Cup = 2314 => (Volume, "u_volume_cup", "cup", "A US customary cup."),
    Volume_Pint = 2315 => (Volume, "u_volume_pint", "pt", "A US liquid pint."),
    Volume_Quart = 2316 => (Volume, "u_volume_quart", "qt", "A US liquid quart."),

    Speed_MeterPerSecond = 2400 => (Speed, "u_speed_meter_per_second", "m/s", "A meter per second, the SI unit of speed."),
    Speed_MillimeterPerSecond = 2401 => (Speed, "u_speed_millimeter_per_second", "mm/s", "A millimeter per second."),
    Speed_CentimeterPerSecond = 2402 => (Speed, "u_speed_centimeter_per_second", "cm/s", "A centimeter per second."),
    Speed_KilometerPerHour = 2403 => (Speed, "u_speed_kilometer_per_hour", "km/h", "A kilometer per hour."),
    Speed_InchPerSecond = 2404 => (Speed, "u_speed_inch_per_second", "in/s", "An international inch per second."),
    Speed_FootPerSecond = 2405 => (Speed, "u_speed_foot_per_second", "ft/s", "An international foot per second."),
    Speed_MilePerHour = 2406 => (Speed, "u_speed_mile_per_hour", "mph", "An international mile per hour."),
    Speed_Knot = 2407 => (Speed, "u_speed_knot", "kn", "A knot."),

    Acceleration_MeterPerSecondSquared = 2500 => (Acceleration, "u_acceleration_meter_per_second_squared", "m/s²", "A meter per second squared, the SI unit of acceleration."),
    Acceleration_MillimeterPerSecondSquared = 2501 => (Acceleration, "u_acceleration_millimeter_per_second_squared", "mm/s²", "A millimeter per second squared."),
    Acceleration_CentimeterPerSecondSquared = 2502 => (Acceleration, "u_acceleration_centimeter_per_second_squared", "cm/s²", "A centimeter per second squared."),
    Acceleration_InchPerSecondSquared = 2503 => (Acceleration, "u_acceleration_inch_per_second_squared", "in/s²", "An international inch per second squared."),
    Acceleration_FootPerSecondSquared = 2504 => (Acceleration, "u_acceleration_foot_per_second_squared", "ft/s²", "An international foot per second squared."),
    Acceleration_StandardGravity = 2505 => (Acceleration, "u_acceleration_standard_gravity", "g₀", "Standard gravity."),

    AngularVelocity_RadianPerSecond = 2600 => (AngularVelocity, "u_angular_velocity_radian_per_second", "rad/s", "A radian per second, the SI unit of angular velocity."),
    AngularVelocity_DegreePerSecond = 2601 => (AngularVelocity, "u_angular_velocity_degree_per_second", "°/s", "A degree per second."),
    AngularVelocity_RevolutionPerMinute = 2602 => (AngularVelocity, "u_angular_velocity_revolution_per_minute", "rpm", "A revolution per minute."),
    AngularVelocity_RevolutionPerSecond = 2603 => (AngularVelocity, "u_angular_velocity_revolution_per_second", "rev/s", "A revolution per second."),

    Torque_NewtonMeter = 2700 => (Torque, "u_torque_newton_meter", "N·m", "A newton meter, the SI unit of torque, distinct from energy despite sharing its SI dimensions."),
    Torque_NewtonMillimeter = 2701 => (Torque, "u_torque_newton_millimeter", "N·mm", "A newton millimeter."),
    Torque_NewtonCentimeter = 2702 => (Torque, "u_torque_newton_centimeter", "N·cm", "A newton centimeter."),
    Torque_PoundForceInch = 2703 => (Torque, "u_torque_pound_force_inch", "lbf·in", "A pound-force inch."),
    Torque_PoundForceFoot = 2704 => (Torque, "u_torque_pound_force_foot", "lbf·ft", "A pound-force foot."),

    Density_KilogramPerCubicMeter = 2800 => (Density, "u_density_kilogram_per_cubic_meter", "kg/m³", "A kilogram per cubic meter, the SI unit of mass density."),
    Density_GramPerCubicCentimeter = 2801 => (Density, "u_density_gram_per_cubic_centimeter", "g/cm³", "A gram per cubic centimeter."),
    Density_GramPerLiter = 2802 => (Density, "u_density_gram_per_liter", "g/l", "A gram per liter."),
    Density_KilogramPerLiter = 2803 => (Density, "u_density_kilogram_per_liter", "kg/l", "A kilogram per liter."),
    Density_PoundPerCubicFoot = 2804 => (Density, "u_density_pound_per_cubic_foot", "lb/ft³", "An international avoirdupois pound per cubic international foot."),

    VolumeFlowRate_CubicMeterPerSecond = 2900 => (VolumeFlowRate, "u_volume_flow_rate_cubic_meter_per_second", "m³/s", "A cubic meter per second, the SI unit of volume flow rate."),
    VolumeFlowRate_LiterPerMinute = 2901 => (VolumeFlowRate, "u_volume_flow_rate_liter_per_minute", "l/min", "A liter per minute."),

    DisplacementPerRevolution_CubicMeterPerRevolution = 3000 => (DisplacementPerRevolution, "u_displacement_per_revolution_cubic_meter_per_revolution", "m³/rev", "A cubic meter per revolution, measuring pump or motor displacement."),
    DisplacementPerRevolution_CubicCentimeterPerRevolution = 3001 => (DisplacementPerRevolution, "u_displacement_per_revolution_cubic_centimeter_per_revolution", "cm³/rev", "A cubic centimeter per revolution."),

    AngularFrequency_RadianPerSecond = 3100 => (AngularFrequency, "u_angular_frequency_radian_per_second", "rad/s", "A radian per second, the SI unit of angular frequency, related to cyclic frequency by omega = 2*pi*f."),
    AngularFrequency_PicoradianPerSecond = 3101 => (AngularFrequency, "u_angular_frequency_picoradian_per_second", "prad/s", "A picoradian per second."),
    AngularFrequency_NanoradianPerSecond = 3102 => (AngularFrequency, "u_angular_frequency_nanoradian_per_second", "nrad/s", "A nanoradian per second."),
    AngularFrequency_MicroradianPerSecond = 3103 => (AngularFrequency, "u_angular_frequency_microradian_per_second", "μrad/s", "A microradian per second."),
    AngularFrequency_MilliradianPerSecond = 3104 => (AngularFrequency, "u_angular_frequency_milliradian_per_second", "mrad/s", "A milliradian per second."),
    AngularFrequency_KiloradianPerSecond = 3105 => (AngularFrequency, "u_angular_frequency_kiloradian_per_second", "krad/s", "A kiloradian per second."),
    AngularFrequency_MegaradianPerSecond = 3106 => (AngularFrequency, "u_angular_frequency_megaradian_per_second", "Mrad/s", "A megaradian per second."),
    AngularFrequency_GigaradianPerSecond = 3107 => (AngularFrequency, "u_angular_frequency_gigaradian_per_second", "Grad/s", "A gigaradian per second."),
    AngularFrequency_TeraradianPerSecond = 3108 => (AngularFrequency, "u_angular_frequency_teraradian_per_second", "Trad/s", "A teraradian per second."),

    MomentOfInertia_KilogramSquareMeter = 3200 => (MomentOfInertia, "u_moment_of_inertia_kilogram_square_meter", "kg·m²", "A kilogram square meter, the SI unit of moment of inertia."),
    MomentOfInertia_KilogramSquareCentimeter = 3201 => (MomentOfInertia, "u_moment_of_inertia_kilogram_square_centimeter", "kg·cm²", "A kilogram square centimeter."),
    MomentOfInertia_GramSquareCentimeter = 3202 => (MomentOfInertia, "u_moment_of_inertia_gram_square_centimeter", "g·cm²", "A gram square centimeter."),

    LinearStiffness_NewtonPerMeter = 3300 => (LinearStiffness, "u_linear_stiffness_newton_per_meter", "N/m", "A newton per meter, the SI unit of linear stiffness."),
    LinearStiffness_NewtonPerMillimeter = 3301 => (LinearStiffness, "u_linear_stiffness_newton_per_millimeter", "N/mm", "A newton per millimeter."),
    LinearStiffness_KilonewtonPerMeter = 3302 => (LinearStiffness, "u_linear_stiffness_kilonewton_per_meter", "kN/m", "A kilonewton per meter."),

    LinearDamping_NewtonSecondPerMeter = 3400 => (LinearDamping, "u_linear_damping_newton_second_per_meter", "N·s/m", "A newton second per meter, the SI unit of linear damping."),
    LinearDamping_NewtonSecondPerMillimeter = 3401 => (LinearDamping, "u_linear_damping_newton_second_per_millimeter", "N·s/mm", "A newton second per millimeter."),
    LinearDamping_KilonewtonSecondPerMeter = 3402 => (LinearDamping, "u_linear_damping_kilonewton_second_per_meter", "kN·s/m", "A kilonewton second per meter."),

    RotationalStiffness_NewtonMeterPerRadian = 3500 => (RotationalStiffness, "u_rotational_stiffness_newton_meter_per_radian", "N·m/rad", "A newton meter per radian, the SI unit of rotational stiffness, distinct from torque."),
    RotationalStiffness_NewtonMillimeterPerRadian = 3501 => (RotationalStiffness, "u_rotational_stiffness_newton_millimeter_per_radian", "N·mm/rad", "A newton millimeter per radian."),
    RotationalStiffness_KilonewtonMeterPerRadian = 3502 => (RotationalStiffness, "u_rotational_stiffness_kilonewton_meter_per_radian", "kN·m/rad", "A kilonewton meter per radian."),

    RotationalDamping_NewtonMeterSecondPerRadian = 3600 => (RotationalDamping, "u_rotational_damping_newton_meter_second_per_radian", "N·m·s/rad", "A newton meter second per radian, the SI unit of rotational damping."),
    RotationalDamping_NewtonMillimeterSecondPerRadian = 3601 => (RotationalDamping, "u_rotational_damping_newton_millimeter_second_per_radian", "N·mm·s/rad", "A newton millimeter second per radian."),
    RotationalDamping_KilonewtonMeterSecondPerRadian = 3602 => (RotationalDamping, "u_rotational_damping_kilonewton_meter_second_per_radian", "kN·m·s/rad", "A kilonewton meter second per radian."),

    DynamicViscosity_PascalSecond = 3700 => (DynamicViscosity, "u_dynamic_viscosity_pascal_second", "Pa·s", "A pascal second, the SI unit of dynamic viscosity."),
    DynamicViscosity_MillipascalSecond = 3701 => (DynamicViscosity, "u_dynamic_viscosity_millipascal_second", "mPa·s", "A millipascal second."),
    DynamicViscosity_Poise = 3702 => (DynamicViscosity, "u_dynamic_viscosity_poise", "P", "A poise."),
    DynamicViscosity_Centipoise = 3703 => (DynamicViscosity, "u_dynamic_viscosity_centipoise", "cP", "A centipoise."),

    KinematicViscosity_SquareMeterPerSecond = 3800 => (KinematicViscosity, "u_kinematic_viscosity_square_meter_per_second", "m²/s", "A square meter per second, the SI unit of kinematic viscosity."),
    KinematicViscosity_SquareMillimeterPerSecond = 3801 => (KinematicViscosity, "u_kinematic_viscosity_square_millimeter_per_second", "mm²/s", "A square millimeter per second."),
    KinematicViscosity_Stokes = 3802 => (KinematicViscosity, "u_kinematic_viscosity_stokes", "St", "A stokes."),
    KinematicViscosity_Centistokes = 3803 => (KinematicViscosity, "u_kinematic_viscosity_centistokes", "cSt", "A centistokes."),

    MassFlowRate_KilogramPerSecond = 3900 => (MassFlowRate, "u_mass_flow_rate_kilogram_per_second", "kg/s", "A kilogram per second, the SI unit of mass flow rate."),
    MassFlowRate_GramPerSecond = 3901 => (MassFlowRate, "u_mass_flow_rate_gram_per_second", "g/s", "A gram per second."),
    MassFlowRate_KilogramPerMinute = 3902 => (MassFlowRate, "u_mass_flow_rate_kilogram_per_minute", "kg/min", "A kilogram per minute."),
    MassFlowRate_KilogramPerHour = 3903 => (MassFlowRate, "u_mass_flow_rate_kilogram_per_hour", "kg/h", "A kilogram per hour."),

    Momentum_KilogramMeterPerSecond = 4000 => (Momentum, "u_momentum_kilogram_meter_per_second", "kg·m/s", "A kilogram meter per second, the SI unit of linear momentum."),
    Momentum_GramCentimeterPerSecond = 4001 => (Momentum, "u_momentum_gram_centimeter_per_second", "g·cm/s", "A gram centimeter per second."),
    Momentum_NewtonSecond = 4002 => (Momentum, "u_momentum_newton_second", "N·s", "A newton second."),

    HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal = 4100 => (HydraulicLeakageCoefficient, "u_hydraulic_leakage_coefficient_cubic_meter_per_second_per_pascal", "m³/s/Pa", "A cubic meter per second per pascal, the SI unit of hydraulic leakage flow per pressure difference."),
    HydraulicLeakageCoefficient_LiterPerMinutePerBar = 4101 => (HydraulicLeakageCoefficient, "u_hydraulic_leakage_coefficient_liter_per_minute_per_bar", "l/min/bar", "A liter per minute per bar."),

    SpecificHeatCapacity_JoulePerKilogramKelvin = 4200 => (SpecificHeatCapacity, "u_specific_heat_capacity_joule_per_kilogram_kelvin", "J/kg/K", "A joule per kilogram per kelvin, the SI unit of specific heat capacity, distinct from the specific gas constant."),
    SpecificHeatCapacity_KilojoulePerKilogramKelvin = 4201 => (SpecificHeatCapacity, "u_specific_heat_capacity_kilojoule_per_kilogram_kelvin", "kJ/kg/K", "A kilojoule per kilogram per kelvin."),

    SpecificGasConstant_JoulePerKilogramKelvin = 4300 => (SpecificGasConstant, "u_specific_gas_constant_joule_per_kilogram_kelvin", "J/kg/K", "A joule per kilogram per kelvin, the SI unit of the specific gas constant, distinct from specific heat capacity."),
    SpecificGasConstant_KilojoulePerKilogramKelvin = 4301 => (SpecificGasConstant, "u_specific_gas_constant_kilojoule_per_kilogram_kelvin", "kJ/kg/K", "A kilojoule per kilogram per kelvin for the specific gas constant."),

    ThermalConductance_WattPerKelvin = 4400 => (ThermalConductance, "u_thermal_conductance_watt_per_kelvin", "W/K", "A watt per kelvin, the SI unit of heat flow per temperature difference."),
    ThermalConductance_MilliwattPerKelvin = 4401 => (ThermalConductance, "u_thermal_conductance_milliwatt_per_kelvin", "mW/K", "A milliwatt per kelvin."),
    ThermalConductance_KilowattPerKelvin = 4402 => (ThermalConductance, "u_thermal_conductance_kilowatt_per_kelvin", "kW/K", "A kilowatt per kelvin."),

    HydraulicResistance_PascalSecondPerCubicMeter = 4500 => (HydraulicResistance, "u_hydraulic_resistance_pascal_second_per_cubic_meter", "Pa·s/m³", "A pascal second per cubic meter, the SI unit of hydraulic resistance or hydraulic characteristic impedance, reciprocal in dimension to a hydraulic leakage coefficient."),
    HydraulicResistance_BarMinutePerLiter = 4501 => (HydraulicResistance, "u_hydraulic_resistance_bar_minute_per_liter", "bar·min/l", "A bar minute per liter."),

    PneumaticCharacteristicImpedance_PascalSecondPerJoule = 4600 => (PneumaticCharacteristicImpedance, "u_pneumatic_characteristic_impedance_pascal_second_per_joule", "Pa·s/J", "A pascal second per joule, the SI unit of pneumatic pressure per energy flow, distinct from hydraulic impedance."),
    PneumaticCharacteristicImpedance_SecondPerCubicMeter = 4601 => (PneumaticCharacteristicImpedance, "u_pneumatic_characteristic_impedance_second_per_cubic_meter", "s/m³", "A second per cubic meter."),

    TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal = 4700 => (TurbulentFlowCoefficient, "u_turbulent_flow_coefficient_cubic_meter_per_second_per_square_root_pascal", "m³/s/√Pa", "A cubic meter per second per square root of a pascal, measuring turbulent flow per square root of pressure difference, distinct from linear leakage."),
    TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar = 4701 => (TurbulentFlowCoefficient, "u_turbulent_flow_coefficient_liter_per_minute_per_square_root_bar", "l/min/√bar", "A liter per minute per square root of a bar."),

    MotorBackEmfConstant_VoltSecondPerRadian = 4800 => (MotorBackEmfConstant, "u_motor_back_emf_constant_volt_second_per_radian", "V·s/rad", "A volt second per radian, measuring motor back electromotive force per angular velocity."),
    MotorBackEmfConstant_MillivoltSecondPerRadian = 4801 => (MotorBackEmfConstant, "u_motor_back_emf_constant_millivolt_second_per_radian", "mV·s/rad", "A millivolt second per radian."),
    MotorBackEmfConstant_VoltPerRevolutionPerMinute = 4802 => (MotorBackEmfConstant, "u_motor_back_emf_constant_volt_per_revolution_per_minute", "V/rpm", "A volt per revolution per minute."),

    ScrewPitch_MeterPerRadian = 4900 => (ScrewPitch, "u_screw_pitch_meter_per_radian", "m/rad", "A meter per radian, measuring linear travel per angular displacement."),
    ScrewPitch_MillimeterPerRadian = 4901 => (ScrewPitch, "u_screw_pitch_millimeter_per_radian", "mm/rad", "A millimeter per radian."),
    ScrewPitch_MillimeterPerRevolution = 4902 => (ScrewPitch, "u_screw_pitch_millimeter_per_revolution", "mm/rev", "A millimeter per revolution."),

    ThrustSpecificFuelConsumption_KilogramPerNewtonSecond = 5000 => (ThrustSpecificFuelConsumption, "u_thrust_specific_fuel_consumption_kilogram_per_newton_second", "kg/N/s", "A kilogram per newton per second, measuring fuel mass flow per thrust."),
    ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour = 5001 => (ThrustSpecificFuelConsumption, "u_thrust_specific_fuel_consumption_kilogram_per_kilonewton_hour", "kg/kN/h", "A kilogram per kilonewton per hour."),

    VolumeFlowAcceleration_CubicMeterPerSecondSquared = 5100 => (VolumeFlowAcceleration, "u_volume_flow_acceleration_cubic_meter_per_second_squared", "m³/s²", "A cubic meter per second squared, the SI unit of the time derivative of volume flow."),
    VolumeFlowAcceleration_LiterPerMinutePerSecond = 5101 => (VolumeFlowAcceleration, "u_volume_flow_acceleration_liter_per_minute_per_second", "l/min/s", "A liter per minute per second."),
}

#[cfg(test)]
mod tests {
    use super::{UnitFamilyId, UnitId};
    use std::collections::HashSet;

    #[test]
    fn families_follow_si_category_order() {
        let expected = [
            UnitFamilyId::None,
            UnitFamilyId::Length,
            UnitFamilyId::Mass,
            UnitFamilyId::Time,
            UnitFamilyId::Current,
            UnitFamilyId::Temperature,
            UnitFamilyId::Amount,
            UnitFamilyId::Frequency,
            UnitFamilyId::Angle,
            UnitFamilyId::SolidAngle,
            UnitFamilyId::Force,
            UnitFamilyId::Pressure,
            UnitFamilyId::Energy,
            UnitFamilyId::Power,
            UnitFamilyId::ElectricCharge,
            UnitFamilyId::Voltage,
            UnitFamilyId::Resistance,
            UnitFamilyId::Conductance,
            UnitFamilyId::Capacitance,
            UnitFamilyId::Inductance,
            UnitFamilyId::MagneticFluxDensity,
            UnitFamilyId::MagneticFlux,
            UnitFamilyId::Area,
            UnitFamilyId::Volume,
            UnitFamilyId::Speed,
            UnitFamilyId::Acceleration,
            UnitFamilyId::AngularVelocity,
            UnitFamilyId::Torque,
            UnitFamilyId::Density,
            UnitFamilyId::VolumeFlowRate,
            UnitFamilyId::DisplacementPerRevolution,
            UnitFamilyId::AngularFrequency,
            UnitFamilyId::MomentOfInertia,
            UnitFamilyId::LinearStiffness,
            UnitFamilyId::LinearDamping,
            UnitFamilyId::RotationalStiffness,
            UnitFamilyId::RotationalDamping,
            UnitFamilyId::DynamicViscosity,
            UnitFamilyId::KinematicViscosity,
            UnitFamilyId::MassFlowRate,
            UnitFamilyId::Momentum,
            UnitFamilyId::HydraulicLeakageCoefficient,
            UnitFamilyId::SpecificHeatCapacity,
            UnitFamilyId::SpecificGasConstant,
            UnitFamilyId::ThermalConductance,
            UnitFamilyId::HydraulicResistance,
            UnitFamilyId::PneumaticCharacteristicImpedance,
            UnitFamilyId::TurbulentFlowCoefficient,
            UnitFamilyId::MotorBackEmfConstant,
            UnitFamilyId::ScrewPitch,
            UnitFamilyId::ThrustSpecificFuelConsumption,
            UnitFamilyId::VolumeFlowAcceleration,
        ];
        assert_eq!(UnitFamilyId::ALL, expected);
    }

    #[test]
    fn si_base_units_are_registered_first_in_their_families() {
        let cases = [
            (UnitId::Length_Meter, 100),
            (UnitId::Mass_Kilogram, 200),
            (UnitId::Time_Second, 300),
            (UnitId::Current_Ampere, 400),
            (UnitId::Temperature_Kelvin, 500),
            (UnitId::Amount_Mole, 600),
        ];
        for (unit, id) in cases {
            assert_eq!(unit.to_u16(), id);
            assert_eq!(UnitId::from_u16(id), Some(unit));
            assert_eq!(unit.family_id().unit_ids().first(), Some(&unit));
        }
    }

    #[test]
    fn hydraulic_units_are_registered() {
        let cases = [
            (
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                "u_volume_flow_rate_cubic_meter_per_second",
                2900,
                "m³/s",
            ),
            (
                UnitId::VolumeFlowRate_LiterPerMinute,
                "u_volume_flow_rate_liter_per_minute",
                2901,
                "l/min",
            ),
            (
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
                "u_displacement_per_revolution_cubic_meter_per_revolution",
                3000,
                "m³/rev",
            ),
            (
                UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution,
                "u_displacement_per_revolution_cubic_centimeter_per_revolution",
                3001,
                "cm³/rev",
            ),
        ];

        for (unit, key, id, symbol) in cases {
            assert_eq!(UnitId::from_unit_id_str(key), Some(unit));
            assert_eq!(UnitId::from_u16(id), Some(unit));
            assert_eq!(unit.description(), symbol);
        }
    }

    #[test]
    fn unit_family_ids_are_consistent() {
        let mut family_values = HashSet::new();
        let mut family_keys = HashSet::new();

        for family in UnitFamilyId::ALL {
            let value = family.to_u8();
            let key = family.description().to_string();

            assert!(
                family_values.insert(value),
                "duplicate family value: {value}"
            );
            assert!(
                family_keys.insert(key.clone()),
                "duplicate family key: {key}"
            );
            assert_eq!(UnitFamilyId::from_u8(value), Some(family));
            assert_ne!(family.description(), "");
            assert!(!family.description().contains(['(', ')']));

            let description = match family {
                UnitFamilyId::None => "None",
                UnitFamilyId::Length => "Length",
                UnitFamilyId::Mass => "Mass",
                UnitFamilyId::Time => "Time",
                UnitFamilyId::Current => "Current",
                UnitFamilyId::Temperature => "Temperature",
                UnitFamilyId::Amount => "Amount",
                UnitFamilyId::Frequency => "Frequency",
                UnitFamilyId::Angle => "Angle",
                UnitFamilyId::SolidAngle => "Solid Angle",
                UnitFamilyId::Force => "Force",
                UnitFamilyId::Pressure => "Pressure",
                UnitFamilyId::Energy => "Energy",
                UnitFamilyId::Power => "Power",
                UnitFamilyId::ElectricCharge => "Electric Charge",
                UnitFamilyId::Voltage => "Voltage",
                UnitFamilyId::Resistance => "Resistance",
                UnitFamilyId::Conductance => "Conductance",
                UnitFamilyId::Capacitance => "Capacitance",
                UnitFamilyId::Inductance => "Inductance",
                UnitFamilyId::MagneticFluxDensity => "Magnetic Flux Density",
                UnitFamilyId::MagneticFlux => "Magnetic Flux",
                UnitFamilyId::Area => "Area",
                UnitFamilyId::Volume => "Volume",
                UnitFamilyId::Speed => "Speed",
                UnitFamilyId::Acceleration => "Acceleration",
                UnitFamilyId::AngularVelocity => "Angular Velocity",
                UnitFamilyId::Torque => "Torque",
                UnitFamilyId::Density => "Density",
                UnitFamilyId::VolumeFlowRate => "Volume Flow Rate",
                UnitFamilyId::DisplacementPerRevolution => "Displacement per Revolution",
                UnitFamilyId::AngularFrequency => "Angular Frequency",
                UnitFamilyId::MomentOfInertia => "Moment of Inertia",
                UnitFamilyId::LinearStiffness => "Linear Stiffness",
                UnitFamilyId::LinearDamping => "Linear Damping",
                UnitFamilyId::RotationalStiffness => "Rotational Stiffness",
                UnitFamilyId::RotationalDamping => "Rotational Damping",
                UnitFamilyId::DynamicViscosity => "Dynamic Viscosity",
                UnitFamilyId::KinematicViscosity => "Kinematic Viscosity",
                UnitFamilyId::MassFlowRate => "Mass Flow Rate",
                UnitFamilyId::Momentum => "Momentum",
                UnitFamilyId::HydraulicLeakageCoefficient => "Hydraulic Leakage Coefficient",
                UnitFamilyId::SpecificHeatCapacity => "Specific Heat Capacity",
                UnitFamilyId::SpecificGasConstant => "Specific Gas Constant",
                UnitFamilyId::ThermalConductance => "Thermal Conductance",
                UnitFamilyId::HydraulicResistance => "Hydraulic Resistance",
                UnitFamilyId::PneumaticCharacteristicImpedance => {
                    "Pneumatic Characteristic Impedance"
                }
                UnitFamilyId::TurbulentFlowCoefficient => "Turbulent Flow Coefficient",
                UnitFamilyId::MotorBackEmfConstant => "Motor Back EMF Constant",
                UnitFamilyId::ScrewPitch => "Screw Pitch",
                UnitFamilyId::ThrustSpecificFuelConsumption => "Thrust-Specific Fuel Consumption",
                UnitFamilyId::VolumeFlowAcceleration => "Volume Flow Acceleration",
            };
            assert_eq!(
                family.description(),
                description,
                "family description does not match expected value: {}",
                family.description()
            );
        }
    }

    #[test]
    fn unit_family_id_from_u8_returns_none_for_invalid_values() {
        let invalid_values = [52, 53, 55, 56, 255];
        for &value in &invalid_values {
            assert_eq!(
                UnitFamilyId::from_u8(value),
                None,
                "from_u8({value}) should return None",
            );
        }
    }

    #[test]
    fn families_list_metric_variants_before_other_units() {
        let cases: &[(UnitFamilyId, &[&str])] = &[
            (UnitFamilyId::Current, &["A", "nA", "μA", "mA", "kA", "MA"]),
            (
                UnitFamilyId::Temperature,
                &["K", "nK", "μK", "mK", "kK", "°C", "°F"],
            ),
            (
                UnitFamilyId::Angle,
                &[
                    "rad", "prad", "nrad", "μrad", "mrad", "krad", "Mrad", "Grad", "Trad", "°",
                    "rev", "′", "″",
                ],
            ),
            (
                UnitFamilyId::Area,
                &[
                    "m²", "μm²", "mm²", "cm²", "km²", "ft²", "in²", "ac", "ha", "mi²",
                ],
            ),
            (
                UnitFamilyId::Volume,
                &[
                    "m³", "cm³", "mm³", "l", "pl", "nl", "μl", "ml", "kl", "Ml", "gal", "gal_uk",
                    "fl_oz", "fl_oz_uk", "cup", "pt", "qt",
                ],
            ),
            (
                UnitFamilyId::Speed,
                &["m/s", "mm/s", "cm/s", "km/h", "in/s", "ft/s", "mph", "kn"],
            ),
            (
                UnitFamilyId::Acceleration,
                &["m/s²", "mm/s²", "cm/s²", "in/s²", "ft/s²", "g₀"],
            ),
            (
                UnitFamilyId::AngularVelocity,
                &["rad/s", "°/s", "rpm", "rev/s"],
            ),
            (
                UnitFamilyId::Torque,
                &["N·m", "N·mm", "N·cm", "lbf·in", "lbf·ft"],
            ),
            (
                UnitFamilyId::Density,
                &["kg/m³", "g/cm³", "g/l", "kg/l", "lb/ft³"],
            ),
            (
                UnitFamilyId::AngularFrequency,
                &[
                    "rad/s", "prad/s", "nrad/s", "μrad/s", "mrad/s", "krad/s", "Mrad/s", "Grad/s",
                    "Trad/s",
                ],
            ),
        ];

        for (family, expected) in cases {
            let descriptions: Vec<_> = family.unit_ids().iter().map(UnitId::description).collect();
            assert_eq!(descriptions, *expected, "unexpected order for {family:?}");
        }
    }

    #[test]
    fn unit_from_invalid_u16_returns_none() {
        let invalid_values = [
            99, 111, 2210, 2317, 2805, 2902, 3002, 3109, 4202, 5102, 65535,
        ];
        for &value in &invalid_values {
            assert_eq!(
                UnitId::from_u16(value),
                None,
                "from_u16({value}) should return None",
            );
        }
    }

    #[test]
    fn removed_specialized_unit_keys_are_not_registered() {
        let removed_units = [
            ("luminous_intensity", "candela"),
            ("luminous_flux", "lumen"),
            ("illuminance", "lux"),
            ("radioactivity", "becquerel"),
            ("absorbed_dose", "gray"),
            ("equivalent_dose", "sievert"),
            ("catalytic_activity", "katal"),
        ];
        for (family, name) in removed_units {
            for prefix in [
                "", "pico", "nano", "micro", "milli", "kilo", "mega", "giga", "tera",
            ] {
                let key = format!("u_{family}_{prefix}{name}");
                assert_eq!(UnitId::from_unit_id_str(&key), None);
            }
        }
        for key in [
            "u_luminous_intensity_hefnerkerze",
            "u_luminous_intensity_international_candle",
            "u_luminous_intensity_decimal_candle",
        ] {
            assert_eq!(UnitId::from_unit_id_str(key), None);
        }
    }

    #[test]
    fn named_si_derived_units_are_registered() {
        let units = [
            ("u_temperature_celsius", 505, "°C"),
            ("u_frequency_hertz", 700, "Hz"),
            ("u_angle_radian", 800, "rad"),
            ("u_solid_angle_steradian", 900, "sr"),
            ("u_force_newton", 1000, "N"),
            ("u_pressure_pascal", 1100, "Pa"),
            ("u_energy_joule", 1200, "J"),
            ("u_power_watt", 1300, "W"),
            ("u_electric_charge_coulomb", 1400, "C"),
            ("u_voltage_volt", 1500, "V"),
            ("u_resistance_ohm", 1600, "Ω"),
            ("u_conductance_siemens", 1700, "S"),
            ("u_capacitance_farad", 1800, "F"),
            ("u_inductance_henry", 1900, "H"),
            ("u_magnetic_flux_density_tesla", 2000, "T"),
            ("u_magnetic_flux_weber", 2100, "Wb"),
        ];

        for (key, value, symbol) in units {
            let unit = UnitId::from_unit_id_str(key);
            assert!(unit.is_some(), "missing unit: {key}");
            assert_eq!(unit, UnitId::from_u16(value));
            if let Some(unit) = unit {
                assert_eq!(unit.description(), symbol);
                assert_eq!(unit.string_id().to_string(), key);
            }
        }
    }

    #[test]
    fn degree_is_registered_in_the_angle_family() {
        let degree = UnitId::Angle_Degree;
        assert_eq!(UnitId::from_unit_id_str("u_angle_degree"), Some(degree));
        assert_eq!(UnitId::from_u16(809), Some(degree));
        assert_eq!(degree.string_id().to_string(), "u_angle_degree");
        assert_eq!(degree.description(), "°");
        assert_eq!(degree.family_id(), UnitFamilyId::Angle);
        assert!(UnitFamilyId::Angle.unit_ids().contains(&degree));
    }

    #[test]
    fn derived_families_have_supported_engineering_prefixes() {
        let prefixes = [
            ("pico", "p"),
            ("nano", "n"),
            ("micro", "μ"),
            ("milli", "m"),
            ("kilo", "k"),
            ("mega", "M"),
            ("giga", "G"),
            ("tera", "T"),
        ];

        for family in UnitFamilyId::ALL {
            if !(UnitFamilyId::Frequency.to_u8()..=UnitFamilyId::MagneticFlux.to_u8())
                .contains(&family.to_u8())
            {
                continue;
            }

            let Some(base) = family.unit_ids().first() else {
                panic!("missing base unit for {family:?}");
            };
            let base_key = base.string_id();
            let Some((family_key, name)) = base_key.as_str().rsplit_once('_') else {
                panic!("invalid base unit key: {base_key}");
            };

            let prefix_count = if matches!(
                family,
                UnitFamilyId::SolidAngle
                    | UnitFamilyId::Inductance
                    | UnitFamilyId::MagneticFluxDensity
            ) {
                4
            } else if family == UnitFamilyId::Capacitance {
                5
            } else {
                prefixes.len()
            };

            for (index, (prefix, symbol)) in prefixes.iter().enumerate() {
                let key = format!("{family_key}_{prefix}{name}");
                if index >= prefix_count {
                    assert_eq!(UnitId::from_unit_id_str(&key), None);
                    let Ok(offset) = u16::try_from(index + 1) else {
                        panic!("prefix offset must fit in u16: {}", index + 1);
                    };
                    let value = u16::from(family.to_u8()) * 100 + offset;
                    assert_eq!(UnitId::from_u16(value), None);
                    continue;
                }
                let Some(unit) = UnitId::from_unit_id_str(&key) else {
                    panic!("missing prefixed unit: {key}");
                };
                assert_eq!(unit.string_id().to_string(), key);
                assert_eq!(
                    unit.description(),
                    format!("{symbol}{}", base.description())
                );
                assert_eq!(unit.family_id(), family);
                assert!(family.unit_ids().contains(&unit));
            }
        }

        assert_eq!(UnitId::Voltage_Millivolt.description(), "mV");
        assert_eq!(UnitId::Voltage_Megavolt.description(), "MV");
    }

    #[test]
    fn numeric_ids_and_registration_order_match_display_order() {
        let mut ordered_units = Vec::new();

        for (family_index, family) in UnitFamilyId::ALL.iter().enumerate() {
            let Ok(family_value) = u8::try_from(family_index) else {
                panic!("family index must fit in u8: {family_index}");
            };
            assert_eq!(family.to_u8(), family_value);
            assert_eq!(UnitFamilyId::from_u8(family_value), Some(*family));

            for (unit_index, unit) in family.unit_ids().iter().enumerate() {
                let Ok(unit_offset) = u16::try_from(unit_index) else {
                    panic!("unit index must fit in u16: {unit_index}");
                };
                let unit_value = u16::from(family_value) * 100 + unit_offset;
                assert_eq!(unit.to_u16(), unit_value, "unexpected ID for {unit:?}");
                assert_eq!(UnitId::from_u16(unit_value), Some(*unit));
                ordered_units.push(*unit);
            }
        }

        assert_eq!(UnitId::ALL, ordered_units.as_slice());
    }

    #[test]
    fn unit_definitions_are_consistent() {
        let mut unit_values = HashSet::new();
        let mut unit_string_ids = HashSet::new();

        for unit_id in UnitId::ALL {
            let value = unit_id.to_u16();
            let family = unit_id.family_id();
            let key = unit_id.string_id().to_string();

            assert!(unit_values.insert(value), "duplicate unit value: {value}");
            assert!(
                unit_string_ids.insert(key.clone()),
                "duplicate unit key: {key}"
            );
            assert_eq!(UnitId::from_u16(value), Some(*unit_id));
            assert_eq!(UnitId::from_unit_id_str(&key), Some(*unit_id));
            assert_eq!(value / 100, u16::from(family.to_u8()));
            assert!(family.unit_ids().contains(unit_id));
            if *unit_id != UnitId::None {
                assert_ne!(unit_id.description(), "");
            }
            assert_ne!(unit_id.documentation(), "");
            assert_ne!(unit_id.documentation(), unit_id.description());
            assert!(!unit_id.description().contains(['(', ')']));
        }

        for family in UnitFamilyId::ALL {
            let family_unit_ids = family.unit_ids();
            let family_units = UnitId::ALL
                .iter()
                .filter(|unit_id| unit_id.family_id() == family)
                .count();

            assert_eq!(UnitFamilyId::from_u8(family.to_u8()), Some(family));
            assert_eq!(family_unit_ids.len(), family_units);

            for unit_id in family_unit_ids {
                assert_eq!(
                    unit_id.family_id(),
                    family,
                    "family unit must belong to its family: {unit_id:?}"
                );
            }
        }
    }
}
