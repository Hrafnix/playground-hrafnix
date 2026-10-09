use crate::unit_definitions::{UnitFamilyId, UnitId};
use hrafnix_common::math::canonicalize_f64;
use std::ops::{Add, Div, Mul, Sub};

#[allow(
    clippy::match_same_arms,
    reason = "This is a simple conversion table, and the repetition is intentional."
)]
/// Converts a given unit to its base unit equivalent.
const fn convert_to_base(unit: UnitId) -> f64 {
    match unit {
        UnitId::None => 1.0,

        UnitId::Length_Meter => 1.0,
        UnitId::Length_Picometer => 0.000_000_000_001,
        UnitId::Length_Nanometer => 0.000_000_001,
        UnitId::Length_Micrometer => 0.000_001,
        UnitId::Length_Millimeter => 0.001,
        UnitId::Length_Centimeter => 0.01,
        UnitId::Length_Kilometer => 1000.0,
        UnitId::Length_Foot => 0.3048,
        UnitId::Length_Inch => 0.0254,
        UnitId::Length_Yard => 0.9144,
        UnitId::Length_Mile => 1609.34,

        UnitId::Mass_Kilogram => 1.0,
        UnitId::Mass_Gram => 0.001,
        UnitId::Mass_Picogram => 0.000_000_000_000_001,
        UnitId::Mass_Nanogram => 0.000_000_000_001,
        UnitId::Mass_Microgram => 0.000_000_001,
        UnitId::Mass_Milligram => 0.000_001,
        UnitId::Mass_Megagram => 1000.0,
        UnitId::Mass_Tonne => 1000.0,
        UnitId::Mass_Pound => 0.453_592,
        UnitId::Mass_Ounce => 0.028_349_5,
        UnitId::Mass_Stone => 6.35029,

        UnitId::Time_Second => 1.0,
        UnitId::Time_PicoSecond => 0.000_000_000_001,
        UnitId::Time_NanoSecond => 0.000_000_001,
        UnitId::Time_MicroSecond => 0.000_001,
        UnitId::Time_MilliSecond => 0.001,
        UnitId::Time_KiloSecond => 1000.0,
        UnitId::Time_Minute => 60.0,
        UnitId::Time_Hour => 3600.0,
        UnitId::Time_Day => 86400.0,
        UnitId::Time_Week => 604_800.0,
        UnitId::Time_Year => 31_536_000.0,

        UnitId::Current_Ampere => 1.0,
        UnitId::Current_Nanoampere => 0.000_000_001,
        UnitId::Current_Microampere => 0.000_001,
        UnitId::Current_Milliampere => 0.001,
        UnitId::Current_Kiloampere => 1000.0,
        UnitId::Current_Megaampere => 1_000_000.0,

        UnitId::Temperature_Kelvin => 1.0,
        UnitId::Temperature_Nanokelvin => 0.000_000_001,
        UnitId::Temperature_Microkelvin => 0.000_001,
        UnitId::Temperature_Millikelvin => 0.001,
        UnitId::Temperature_Kilokelvin => 1000.0,
        UnitId::Temperature_Celsius => 0.0,
        UnitId::Temperature_Fahrenheit => 0.0,

        UnitId::Amount_Mole => 1.0,
        UnitId::Amount_Picomole => 0.000_000_000_001,
        UnitId::Amount_Nanomole => 0.000_000_001,
        UnitId::Amount_Micromole => 0.000_001,
        UnitId::Amount_Millimole => 0.001,
        UnitId::Amount_Kilomole => 1000.0,

        UnitId::Frequency_Hertz => 1.0,
        UnitId::Frequency_Picohertz => 0.000_000_000_001,
        UnitId::Frequency_Nanohertz => 0.000_000_001,
        UnitId::Frequency_Microhertz => 0.000_001,
        UnitId::Frequency_Millihertz => 0.001,
        UnitId::Frequency_Kilohertz => 1000.0,
        UnitId::Frequency_Megahertz => 1_000_000.0,
        UnitId::Frequency_Gigahertz => 1_000_000_000.0,
        UnitId::Frequency_Terahertz => 1_000_000_000_000.0,

        UnitId::Angle_Radian => 1.0,
        UnitId::Angle_Picoradian => 0.000_000_000_001,
        UnitId::Angle_Nanoradian => 0.000_000_001,
        UnitId::Angle_Microradian => 0.000_001,
        UnitId::Angle_Milliradian => 0.001,
        UnitId::Angle_Kiloradian => 1000.0,
        UnitId::Angle_Megaradian => 1_000_000.0,
        UnitId::Angle_Gigaradian => 1_000_000_000.0,
        UnitId::Angle_Teraradian => 1_000_000_000_000.0,
        UnitId::Angle_Degree => 0.017_453_292_519_943_295,
        UnitId::Angle_Turn => std::f64::consts::TAU,
        UnitId::Angle_Arcminute => 0.000_290_888_208_665_721_6,
        UnitId::Angle_Arcsecond => 0.000_004_848_136_811_095_36,

        UnitId::SolidAngle_Steradian => 1.0,
        UnitId::SolidAngle_Picosteradian => 0.000_000_000_001,
        UnitId::SolidAngle_Nanosteradian => 0.000_000_001,
        UnitId::SolidAngle_Microsteradian => 0.000_001,
        UnitId::SolidAngle_Millisteradian => 0.001,

        UnitId::Force_Newton => 1.0,
        UnitId::Force_Piconewton => 0.000_000_000_001,
        UnitId::Force_Nanonewton => 0.000_000_001,
        UnitId::Force_Micronewton => 0.000_001,
        UnitId::Force_Millinewton => 0.001,
        UnitId::Force_Kilonewton => 1000.0,
        UnitId::Force_Meganewton => 1_000_000.0,
        UnitId::Force_Giganewton => 1_000_000_000.0,
        UnitId::Force_Teranewton => 1_000_000_000_000.0,
        UnitId::Force_KilogramForce => 9.80665,
        UnitId::Force_PoundForce => 4.448_221_615_260_5,

        UnitId::Pressure_Pascal => 1.0,
        UnitId::Pressure_Picopascal => 0.000_000_000_001,
        UnitId::Pressure_Nanopascal => 0.000_000_001,
        UnitId::Pressure_Micropascal => 0.000_001,
        UnitId::Pressure_Millipascal => 0.001,
        UnitId::Pressure_Kilopascal => 1000.0,
        UnitId::Pressure_Megapascal => 1_000_000.0,
        UnitId::Pressure_Gigapascal => 1_000_000_000.0,
        UnitId::Pressure_Terapascal => 1_000_000_000_000.0,
        UnitId::Pressure_Bar => 100_000.0,
        UnitId::Pressure_Millibar => 100.0,
        UnitId::Pressure_StandardAtmosphere => 101_325.0,
        UnitId::Pressure_PoundPerSquareInch => 6_894.757_293_168_361,
        UnitId::Pressure_MillimeterOfMercury => 133.322_387_415,

        UnitId::Energy_Joule => 1.0,
        UnitId::Energy_Picojoule => 0.000_000_000_001,
        UnitId::Energy_Nanojoule => 0.000_000_001,
        UnitId::Energy_Microjoule => 0.000_001,
        UnitId::Energy_Millijoule => 0.001,
        UnitId::Energy_Kilojoule => 1000.0,
        UnitId::Energy_Megajoule => 1_000_000.0,
        UnitId::Energy_Gigajoule => 1_000_000_000.0,
        UnitId::Energy_Terajoule => 1_000_000_000_000.0,
        UnitId::Energy_WattHour => 3600.0,
        UnitId::Energy_KilowattHour => 3_600_000.0,
        UnitId::Energy_Electronvolt => 1.602_176_634e-19,

        UnitId::Power_Watt => 1.0,
        UnitId::Power_Picowatt => 0.000_000_000_001,
        UnitId::Power_Nanowatt => 0.000_000_001,
        UnitId::Power_Microwatt => 0.000_001,
        UnitId::Power_Milliwatt => 0.001,
        UnitId::Power_Kilowatt => 1000.0,
        UnitId::Power_Megawatt => 1_000_000.0,
        UnitId::Power_Gigawatt => 1_000_000_000.0,
        UnitId::Power_Terawatt => 1_000_000_000_000.0,
        UnitId::Power_MechanicalHorsepower => 745.699_871_582_270_2,
        UnitId::Power_MetricHorsepower => 735.49875,

        UnitId::ElectricCharge_Coulomb => 1.0,
        UnitId::ElectricCharge_Picocoulomb => 0.000_000_000_001,
        UnitId::ElectricCharge_Nanocoulomb => 0.000_000_001,
        UnitId::ElectricCharge_Microcoulomb => 0.000_001,
        UnitId::ElectricCharge_Millicoulomb => 0.001,
        UnitId::ElectricCharge_Kilocoulomb => 1000.0,
        UnitId::ElectricCharge_Megacoulomb => 1_000_000.0,
        UnitId::ElectricCharge_Gigacoulomb => 1_000_000_000.0,
        UnitId::ElectricCharge_Teracoulomb => 1_000_000_000_000.0,
        UnitId::ElectricCharge_AmpereHour => 3600.0,
        UnitId::ElectricCharge_MilliampereHour => 3.6,

        UnitId::Voltage_Volt => 1.0,
        UnitId::Voltage_Picovolt => 0.000_000_000_001,
        UnitId::Voltage_Nanovolt => 0.000_000_001,
        UnitId::Voltage_Microvolt => 0.000_001,
        UnitId::Voltage_Millivolt => 0.001,
        UnitId::Voltage_Kilovolt => 1000.0,
        UnitId::Voltage_Megavolt => 1_000_000.0,
        UnitId::Voltage_Gigavolt => 1_000_000_000.0,
        UnitId::Voltage_Teravolt => 1_000_000_000_000.0,

        UnitId::Resistance_Ohm => 1.0,
        UnitId::Resistance_Picoohm => 0.000_000_000_001,
        UnitId::Resistance_Nanoohm => 0.000_000_001,
        UnitId::Resistance_Microohm => 0.000_001,
        UnitId::Resistance_Milliohm => 0.001,
        UnitId::Resistance_Kiloohm => 1000.0,
        UnitId::Resistance_Megaohm => 1_000_000.0,
        UnitId::Resistance_Gigaohm => 1_000_000_000.0,
        UnitId::Resistance_Teraohm => 1_000_000_000_000.0,

        UnitId::Conductance_Siemens => 1.0,
        UnitId::Conductance_Picosiemens => 0.000_000_000_001,
        UnitId::Conductance_Nanosiemens => 0.000_000_001,
        UnitId::Conductance_Microsiemens => 0.000_001,
        UnitId::Conductance_Millisiemens => 0.001,
        UnitId::Conductance_Kilosiemens => 1000.0,
        UnitId::Conductance_Megasiemens => 1_000_000.0,
        UnitId::Conductance_Gigasiemens => 1_000_000_000.0,
        UnitId::Conductance_Terasiemens => 1_000_000_000_000.0,

        UnitId::Capacitance_Farad => 1.0,
        UnitId::Capacitance_Picofarad => 0.000_000_000_001,
        UnitId::Capacitance_Nanofarad => 0.000_000_001,
        UnitId::Capacitance_Microfarad => 0.000_001,
        UnitId::Capacitance_Millifarad => 0.001,
        UnitId::Capacitance_Kilofarad => 1000.0,

        UnitId::Inductance_Henry => 1.0,
        UnitId::Inductance_Picohenry => 0.000_000_000_001,
        UnitId::Inductance_Nanohenry => 0.000_000_001,
        UnitId::Inductance_Microhenry => 0.000_001,
        UnitId::Inductance_Millihenry => 0.001,

        UnitId::MagneticFluxDensity_Tesla => 1.0,
        UnitId::MagneticFluxDensity_Picotesla => 0.000_000_000_001,
        UnitId::MagneticFluxDensity_Nanotesla => 0.000_000_001,
        UnitId::MagneticFluxDensity_Microtesla => 0.000_001,
        UnitId::MagneticFluxDensity_Millitesla => 0.001,

        UnitId::MagneticFlux_Weber => 1.0,
        UnitId::MagneticFlux_Picoweber => 0.000_000_000_001,
        UnitId::MagneticFlux_Nanoweber => 0.000_000_001,
        UnitId::MagneticFlux_Microweber => 0.000_001,
        UnitId::MagneticFlux_Milliweber => 0.001,
        UnitId::MagneticFlux_Kiloweber => 1000.0,
        UnitId::MagneticFlux_Megaweber => 1_000_000.0,
        UnitId::MagneticFlux_Gigaweber => 1_000_000_000.0,
        UnitId::MagneticFlux_Teraweber => 1_000_000_000_000.0,

        UnitId::Area_SquareMeter => 1.0,
        UnitId::Area_SquareMicroMeter => 0.000_000_000_001,
        UnitId::Area_SquareMilliMeter => 0.000_001,
        UnitId::Area_SquareCentiMeter => 0.0001,
        UnitId::Area_SquareKiloMeter => 1_000_000.0,
        UnitId::Area_SquareFoot => 0.092_903,
        UnitId::Area_SquareInch => 0.000_645_16,
        UnitId::Area_Acre => 4046.86,
        UnitId::Area_Hectare => 10000.0,
        UnitId::Area_SquareMile => 2_589_988.11,

        UnitId::Volume_CubicMeter => 1.0,
        UnitId::Volume_CubicCentiMeter => 0.000_001,
        UnitId::Volume_CubicMilliMeter => 0.000_000_001,
        UnitId::Volume_Liter => 0.001,
        UnitId::Volume_Picoliter => 0.000_000_000_000_001,
        UnitId::Volume_Nanoliter => 0.000_000_000_001,
        UnitId::Volume_Microliter => 0.000_000_001,
        UnitId::Volume_Milliliter => 0.000_001,
        UnitId::Volume_Kiloliter => 1.0,
        UnitId::Volume_Megaliter => 1000.0,
        UnitId::Volume_Gallon => 0.003_785_41,
        UnitId::Volume_ImperialGallon => 0.004_546_09,
        UnitId::Volume_FluidOunce => 0.000_029_573_5,
        UnitId::Volume_ImperialFluidOunce => 0.000_028_413_1,
        UnitId::Volume_Cup => 0.000_236_588_236_5,
        UnitId::Volume_Pint => 0.000_473_176,
        UnitId::Volume_Quart => 0.000_946_353,

        UnitId::Speed_MeterPerSecond => 1.0,
        UnitId::Speed_MillimeterPerSecond => 0.001,
        UnitId::Speed_CentimeterPerSecond => 0.01,
        UnitId::Speed_KilometerPerHour => 0.277_777_777_777_777_8,
        UnitId::Speed_InchPerSecond => 0.0254,
        UnitId::Speed_FootPerSecond => 0.3048,
        UnitId::Speed_MilePerHour => 0.44704,
        UnitId::Speed_Knot => 0.514_444_444_444_444_5,

        UnitId::Acceleration_MeterPerSecondSquared => 1.0,
        UnitId::Acceleration_MillimeterPerSecondSquared => 0.001,
        UnitId::Acceleration_CentimeterPerSecondSquared => 0.01,
        UnitId::Acceleration_InchPerSecondSquared => 0.0254,
        UnitId::Acceleration_FootPerSecondSquared => 0.3048,
        UnitId::Acceleration_StandardGravity => 9.80665,

        UnitId::AngularVelocity_RadianPerSecond => 1.0,
        UnitId::AngularVelocity_DegreePerSecond => 0.017_453_292_519_943_295,
        UnitId::AngularVelocity_RevolutionPerMinute => 0.104_719_755_119_659_77,
        UnitId::AngularVelocity_RevolutionPerSecond => std::f64::consts::TAU,

        UnitId::Torque_NewtonMeter => 1.0,
        UnitId::Torque_NewtonMillimeter => 0.001,
        UnitId::Torque_NewtonCentimeter => 0.01,
        UnitId::Torque_PoundForceInch => 0.112_984_829_027_616_7,
        UnitId::Torque_PoundForceFoot => 1.355_817_948_331_400_4,

        UnitId::Density_KilogramPerCubicMeter => 1.0,
        UnitId::Density_GramPerCubicCentimeter => 1000.0,
        UnitId::Density_GramPerLiter => 1.0,
        UnitId::Density_KilogramPerLiter => 1000.0,
        UnitId::Density_PoundPerCubicFoot => 16.018_463_373_960_138,

        UnitId::VolumeFlowRate_CubicMeterPerSecond => 1.0,
        UnitId::VolumeFlowRate_LiterPerMinute => 0.000_016_666_666_666_666_667,

        UnitId::DisplacementPerRevolution_CubicMeterPerRevolution => 1.0,
        UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution => 0.000_001,

        UnitId::AngularFrequency_RadianPerSecond => 1.0,
        UnitId::AngularFrequency_PicoradianPerSecond => 0.000_000_000_001,
        UnitId::AngularFrequency_NanoradianPerSecond => 0.000_000_001,
        UnitId::AngularFrequency_MicroradianPerSecond => 0.000_001,
        UnitId::AngularFrequency_MilliradianPerSecond => 0.001,
        UnitId::AngularFrequency_KiloradianPerSecond => 1000.0,
        UnitId::AngularFrequency_MegaradianPerSecond => 1_000_000.0,
        UnitId::AngularFrequency_GigaradianPerSecond => 1_000_000_000.0,
        UnitId::AngularFrequency_TeraradianPerSecond => 1_000_000_000_000.0,

        UnitId::MomentOfInertia_KilogramSquareMeter => 1.0,
        UnitId::MomentOfInertia_KilogramSquareCentimeter => 0.0001,
        UnitId::MomentOfInertia_GramSquareCentimeter => 0.000_000_1,

        UnitId::LinearStiffness_NewtonPerMeter => 1.0,
        UnitId::LinearStiffness_NewtonPerMillimeter => 1000.0,
        UnitId::LinearStiffness_KilonewtonPerMeter => 1000.0,

        UnitId::LinearDamping_NewtonSecondPerMeter => 1.0,
        UnitId::LinearDamping_NewtonSecondPerMillimeter => 1000.0,
        UnitId::LinearDamping_KilonewtonSecondPerMeter => 1000.0,

        UnitId::RotationalStiffness_NewtonMeterPerRadian => 1.0,
        UnitId::RotationalStiffness_NewtonMillimeterPerRadian => 0.001,
        UnitId::RotationalStiffness_KilonewtonMeterPerRadian => 1000.0,

        UnitId::RotationalDamping_NewtonMeterSecondPerRadian => 1.0,
        UnitId::RotationalDamping_NewtonMillimeterSecondPerRadian => 0.001,
        UnitId::RotationalDamping_KilonewtonMeterSecondPerRadian => 1000.0,

        UnitId::DynamicViscosity_PascalSecond => 1.0,
        UnitId::DynamicViscosity_MillipascalSecond => 0.001,
        UnitId::DynamicViscosity_Poise => 0.1,
        UnitId::DynamicViscosity_Centipoise => 0.001,

        UnitId::KinematicViscosity_SquareMeterPerSecond => 1.0,
        UnitId::KinematicViscosity_SquareMillimeterPerSecond => 0.000_001,
        UnitId::KinematicViscosity_Stokes => 0.0001,
        UnitId::KinematicViscosity_Centistokes => 0.000_001,

        UnitId::MassFlowRate_KilogramPerSecond => 1.0,
        UnitId::MassFlowRate_GramPerSecond => 0.001,
        UnitId::MassFlowRate_KilogramPerMinute => 0.016_666_666_666_666_666,
        UnitId::MassFlowRate_KilogramPerHour => 0.000_277_777_777_777_777_8,

        UnitId::Momentum_KilogramMeterPerSecond => 1.0,
        UnitId::Momentum_GramCentimeterPerSecond => 0.000_01,
        UnitId::Momentum_NewtonSecond => 1.0,

        UnitId::HydraulicLeakageCoefficient_CubicMeterPerSecondPerPascal => 1.0,
        UnitId::HydraulicLeakageCoefficient_LiterPerMinutePerBar => {
            0.000_000_000_166_666_666_666_666_66
        }

        UnitId::SpecificHeatCapacity_JoulePerKilogramKelvin => 1.0,
        UnitId::SpecificHeatCapacity_KilojoulePerKilogramKelvin => 1000.0,

        UnitId::SpecificGasConstant_JoulePerKilogramKelvin => 1.0,
        UnitId::SpecificGasConstant_KilojoulePerKilogramKelvin => 1000.0,

        UnitId::ThermalConductance_WattPerKelvin => 1.0,
        UnitId::ThermalConductance_MilliwattPerKelvin => 0.001,
        UnitId::ThermalConductance_KilowattPerKelvin => 1000.0,

        UnitId::HydraulicResistance_PascalSecondPerCubicMeter => 1.0,
        UnitId::HydraulicResistance_BarMinutePerLiter => 6_000_000_000.0,

        UnitId::PneumaticCharacteristicImpedance_PascalSecondPerJoule => 1.0,
        UnitId::PneumaticCharacteristicImpedance_SecondPerCubicMeter => 1.0,

        UnitId::TurbulentFlowCoefficient_CubicMeterPerSecondPerSquareRootPascal => 1.0,
        UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar => {
            0.000_000_052_704_627_669_472_99
        }

        UnitId::MotorBackEmfConstant_VoltSecondPerRadian => 1.0,
        UnitId::MotorBackEmfConstant_MillivoltSecondPerRadian => 0.001,
        UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute => 9.549_296_585_513_721,

        UnitId::ScrewPitch_MeterPerRadian => 1.0,
        UnitId::ScrewPitch_MillimeterPerRadian => 0.001,
        UnitId::ScrewPitch_MillimeterPerRevolution => 0.000_159_154_943_091_895_35,

        UnitId::ThrustSpecificFuelConsumption_KilogramPerNewtonSecond => 1.0,
        UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour => {
            0.000_000_277_777_777_777_777_76
        }

        UnitId::VolumeFlowAcceleration_CubicMeterPerSecondSquared => 1.0,
        UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond => 0.000_016_666_666_666_666_667,
    }
}

/// Converts a value from one unit to another, returning an error if the units are incompatible.
///
/// # Errors
///
/// Returns an error if the units are not compatible for conversion (i.e., they belong to different unit families).
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn convert(value: f64, from_unit: UnitId, to_unit: UnitId) -> Result<f64, String> {
    let value = canonicalize_f64(value);

    if !value.is_finite() {
        return Err("Unit conversion input must be finite".into());
    }

    if from_unit == to_unit {
        return Ok(value);
    }

    if to_unit == UnitId::None {
        return Ok(value);
    }

    if from_unit == UnitId::None {
        return Err("Cannot convert a unitless value to a unit".into());
    }

    if from_unit.family_id() != to_unit.family_id() {
        return Err("Units are not compatible for conversion".into());
    }

    if from_unit.family_id() == UnitFamilyId::Temperature {
        let converted = match (from_unit, to_unit) {
            (UnitId::Temperature_Celsius, UnitId::Temperature_Fahrenheit) => {
                value.mul_add(9.0.div(5.0), 32.0)
            }
            (UnitId::Temperature_Fahrenheit, UnitId::Temperature_Celsius) => {
                value.sub(32.0).mul(5.0.div(9.0))
            }
            (UnitId::Temperature_Celsius, UnitId::Temperature_Kelvin) => value.add(273.15),
            (UnitId::Temperature_Kelvin, UnitId::Temperature_Celsius) => value.sub(273.15),
            (UnitId::Temperature_Fahrenheit, UnitId::Temperature_Kelvin) => {
                value.sub(32.0).mul_add(5.0.div(9.0), 273.15)
            }
            (UnitId::Temperature_Kelvin, UnitId::Temperature_Fahrenheit) => {
                value.sub(273.15).mul_add(9.0.div(5.0), 32.0)
            }
            (UnitId::Temperature_Celsius, kelvin) => value.add(273.15).div(convert_to_base(kelvin)),
            (UnitId::Temperature_Fahrenheit, kelvin) => value
                .sub(32.0)
                .mul_add(5.0.div(9.0), 273.15)
                .div(convert_to_base(kelvin)),
            (kelvin, UnitId::Temperature_Celsius) => value.mul(convert_to_base(kelvin)).sub(273.15),
            (kelvin, UnitId::Temperature_Fahrenheit) => value
                .mul(convert_to_base(kelvin))
                .sub(273.15)
                .mul_add(9.0.div(5.0), 32.0),
            (from_kelvin, to_kelvin) => {
                value.mul(convert_to_base(from_kelvin).div(convert_to_base(to_kelvin)))
            }
        };
        let converted = canonicalize_f64(converted);
        return converted
            .is_finite()
            .then_some(converted)
            .ok_or_else(|| "Unit conversion result must be finite".into());
    }

    let from_base = convert_to_base(from_unit);
    let to_base = convert_to_base(to_unit);

    let converted = canonicalize_f64(value.mul(from_base.div(to_base)));
    converted
        .is_finite()
        .then_some(converted)
        .ok_or_else(|| "Unit conversion result must be finite".into())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{convert, convert_to_base};
    use crate::unit_definitions::{UnitFamilyId, UnitId};
    use std::ops::{Div, Mul, Sub};

    const COMPONENT_FAMILIES: [UnitFamilyId; 10] = [
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

    const SIMULATION_FAMILIES: [UnitFamilyId; 10] = [
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
    ];

    fn assert_approx_eq(actual: f64, expected: f64) {
        let tolerance = 1e-12_f64.mul(expected.abs().max(1.0));
        assert!(
            actual.sub(expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn first_unit_in_each_family_is_its_conversion_base() {
        for family in UnitFamilyId::ALL {
            let base = family.unit_ids().first().unwrap();
            assert_eq!(
                convert_to_base(*base),
                1.0,
                "unexpected base for {family:?}"
            );
        }
    }

    #[test]
    fn component_families_use_si_bases_and_engineering_scales() {
        let cases = [
            (UnitId::SpecificHeatCapacity_KilojoulePerKilogramKelvin, 1e3),
            (UnitId::SpecificGasConstant_KilojoulePerKilogramKelvin, 1e3),
            (UnitId::ThermalConductance_MilliwattPerKelvin, 1e-3),
            (UnitId::ThermalConductance_KilowattPerKelvin, 1e3),
            (
                UnitId::HydraulicResistance_BarMinutePerLiter,
                100_000.0_f64.mul(60.0).div(0.001),
            ),
            (
                UnitId::PneumaticCharacteristicImpedance_SecondPerCubicMeter,
                1.0,
            ),
            (
                UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar,
                0.001_f64.div(60.0).div(100_000.0_f64.sqrt()),
            ),
            (UnitId::MotorBackEmfConstant_MillivoltSecondPerRadian, 1e-3),
            (
                UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute,
                60.0_f64.div(std::f64::consts::TAU),
            ),
            (UnitId::ScrewPitch_MillimeterPerRadian, 1e-3),
            (
                UnitId::ScrewPitch_MillimeterPerRevolution,
                0.001_f64.div(std::f64::consts::TAU),
            ),
            (
                UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour,
                1.0_f64.div(1000.0).div(3600.0),
            ),
            (
                UnitId::VolumeFlowAcceleration_LiterPerMinutePerSecond,
                0.001_f64.div(60.0),
            ),
        ];

        assert_eq!(
            cases.len(),
            COMPONENT_FAMILIES
                .iter()
                .map(|family| family.unit_ids().len() - 1)
                .sum::<usize>()
        );
        for family in COMPONENT_FAMILIES {
            let base = family.unit_ids().first().unwrap();
            assert_eq!(convert_to_base(*base), 1.0);
        }

        for (unit, factor) in cases {
            let family = unit.family_id();
            let base = family.unit_ids().first().unwrap();
            assert_approx_eq(convert_to_base(unit).div(factor), 1.0);
            assert_approx_eq(convert(1.0, unit, *base).unwrap().div(factor), 1.0);
            assert_approx_eq(convert(factor, *base, unit).unwrap(), 1.0);
            assert_approx_eq(convert(-1.0, unit, *base).unwrap().div(factor), -1.0);
        }
    }

    #[test]
    fn component_families_are_not_interchangeable() {
        for family in COMPONENT_FAMILIES {
            for unit in family.unit_ids() {
                for other in UnitId::ALL {
                    if *other == UnitId::None || other.family_id() == family {
                        continue;
                    }
                    assert_eq!(
                        convert(1.0, *unit, *other),
                        Err("Units are not compatible for conversion".into())
                    );
                    assert_eq!(
                        convert(1.0, *other, *unit),
                        Err("Units are not compatible for conversion".into())
                    );
                }
            }
        }
    }

    #[test]
    fn component_compound_factors_are_available_in_const_context() {
        const FACTORS: [f64; 4] = [
            convert_to_base(UnitId::TurbulentFlowCoefficient_LiterPerMinutePerSquareRootBar),
            convert_to_base(UnitId::MotorBackEmfConstant_VoltPerRevolutionPerMinute),
            convert_to_base(UnitId::ScrewPitch_MillimeterPerRevolution),
            convert_to_base(UnitId::ThrustSpecificFuelConsumption_KilogramPerKilonewtonHour),
        ];
        let expected = [
            0.001_f64.div(60.0).div(100_000.0_f64.sqrt()),
            60.0_f64.div(std::f64::consts::TAU),
            0.001_f64.div(std::f64::consts::TAU),
            1.0_f64.div(3_600_000.0),
        ];

        for (actual, expected) in FACTORS.into_iter().zip(expected) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn simulation_families_use_si_bases_and_engineering_scales() {
        let cases = [
            (UnitId::MomentOfInertia_KilogramSquareCentimeter, 1e-4),
            (UnitId::MomentOfInertia_GramSquareCentimeter, 1e-7),
            (UnitId::LinearStiffness_NewtonPerMillimeter, 1e3),
            (UnitId::LinearStiffness_KilonewtonPerMeter, 1e3),
            (UnitId::LinearDamping_NewtonSecondPerMillimeter, 1e3),
            (UnitId::LinearDamping_KilonewtonSecondPerMeter, 1e3),
            (UnitId::RotationalStiffness_NewtonMillimeterPerRadian, 1e-3),
            (UnitId::RotationalStiffness_KilonewtonMeterPerRadian, 1e3),
            (
                UnitId::RotationalDamping_NewtonMillimeterSecondPerRadian,
                1e-3,
            ),
            (
                UnitId::RotationalDamping_KilonewtonMeterSecondPerRadian,
                1e3,
            ),
            (UnitId::DynamicViscosity_MillipascalSecond, 1e-3),
            (UnitId::DynamicViscosity_Poise, 0.1),
            (UnitId::DynamicViscosity_Centipoise, 1e-3),
            (UnitId::KinematicViscosity_SquareMillimeterPerSecond, 1e-6),
            (UnitId::KinematicViscosity_Stokes, 1e-4),
            (UnitId::KinematicViscosity_Centistokes, 1e-6),
            (UnitId::MassFlowRate_GramPerSecond, 1e-3),
            (UnitId::MassFlowRate_KilogramPerMinute, 1.0_f64.div(60.0)),
            (UnitId::MassFlowRate_KilogramPerHour, 1.0_f64.div(3600.0)),
            (UnitId::Momentum_GramCentimeterPerSecond, 1e-5),
            (UnitId::Momentum_NewtonSecond, 1.0),
            (
                UnitId::HydraulicLeakageCoefficient_LiterPerMinutePerBar,
                0.001_f64.div(60.0).div(100_000.0),
            ),
        ];

        let families = &SIMULATION_FAMILIES;
        assert_eq!(
            cases.len(),
            families
                .iter()
                .map(|family| family.unit_ids().len() - 1)
                .sum::<usize>()
        );
        for family in families {
            let base = family.unit_ids().first().unwrap();
            assert_eq!(convert_to_base(*base), 1.0);
        }

        for (unit, factor) in cases {
            let family = unit.family_id();
            let base = family.unit_ids().first().unwrap();
            assert_approx_eq(convert_to_base(unit).div(factor), 1.0);
            assert_approx_eq(convert(1.0, unit, *base).unwrap().div(factor), 1.0);
            assert_approx_eq(convert(factor, *base, unit).unwrap(), 1.0);
            assert_approx_eq(convert(-1.0, unit, *base).unwrap().div(factor), -1.0);
        }
    }

    #[test]
    fn simulation_families_are_not_interchangeable() {
        for family in &SIMULATION_FAMILIES {
            for unit in family.unit_ids() {
                for other in UnitId::ALL {
                    if *other == UnitId::None || other.family_id() == *family {
                        continue;
                    }
                    assert_eq!(
                        convert(1.0, *unit, *other),
                        Err("Units are not compatible for conversion".into())
                    );
                    assert_eq!(
                        convert(1.0, *other, *unit),
                        Err("Units are not compatible for conversion".into())
                    );
                }
            }
        }
    }

    #[test]
    fn volume_uses_cubic_meters_as_its_base() {
        assert_eq!(
            UnitFamilyId::Volume.unit_ids().first(),
            Some(&UnitId::Volume_CubicMeter)
        );
        let cases = [
            (UnitId::Volume_CubicMeter, 1.0),
            (UnitId::Volume_CubicCentiMeter, 1e-6),
            (UnitId::Volume_CubicMilliMeter, 1e-9),
            (UnitId::Volume_Liter, 1e-3),
            (UnitId::Volume_Picoliter, 1e-15),
            (UnitId::Volume_Nanoliter, 1e-12),
            (UnitId::Volume_Microliter, 1e-9),
            (UnitId::Volume_Milliliter, 1e-6),
            (UnitId::Volume_Kiloliter, 1.0),
            (UnitId::Volume_Megaliter, 1e3),
            (UnitId::Volume_Gallon, 0.003_785_41),
            (UnitId::Volume_ImperialGallon, 0.004_546_09),
            (UnitId::Volume_FluidOunce, 0.000_029_573_5),
            (UnitId::Volume_ImperialFluidOunce, 0.000_028_413_1),
            (UnitId::Volume_Cup, 0.000_236_588_236_5),
            (UnitId::Volume_Pint, 0.000_473_176),
            (UnitId::Volume_Quart, 0.000_946_353),
        ];
        assert_eq!(cases.len(), UnitFamilyId::Volume.unit_ids().len());

        for (unit, factor) in cases {
            assert_eq!(convert_to_base(unit), factor);
            assert_approx_eq(
                convert(1.0, unit, UnitId::Volume_CubicMeter)
                    .unwrap()
                    .div(factor),
                1.0,
            );
            assert_approx_eq(
                convert(factor, UnitId::Volume_CubicMeter, unit).unwrap(),
                1.0,
            );
        }
    }

    #[test]
    fn converts_angular_frequency_prefixes() {
        let cases = [
            (UnitId::AngularFrequency_RadianPerSecond, 1.0),
            (UnitId::AngularFrequency_PicoradianPerSecond, 1e-12),
            (UnitId::AngularFrequency_NanoradianPerSecond, 1e-9),
            (UnitId::AngularFrequency_MicroradianPerSecond, 1e-6),
            (UnitId::AngularFrequency_MilliradianPerSecond, 1e-3),
            (UnitId::AngularFrequency_KiloradianPerSecond, 1e3),
            (UnitId::AngularFrequency_MegaradianPerSecond, 1e6),
            (UnitId::AngularFrequency_GigaradianPerSecond, 1e9),
            (UnitId::AngularFrequency_TeraradianPerSecond, 1e12),
        ];

        for (unit, factor) in cases {
            assert_eq!(convert_to_base(unit), factor);
            assert_approx_eq(
                convert(1.0, unit, UnitId::AngularFrequency_RadianPerSecond).unwrap(),
                factor,
            );
            assert_approx_eq(
                convert(factor, UnitId::AngularFrequency_RadianPerSecond, unit).unwrap(),
                1.0,
            );
        }
    }

    #[test]
    fn angular_frequency_is_not_interchangeable_with_frequency_or_angular_velocity() {
        for angular_frequency in UnitFamilyId::AngularFrequency.unit_ids() {
            for other in UnitFamilyId::Frequency
                .unit_ids()
                .iter()
                .chain(UnitFamilyId::AngularVelocity.unit_ids())
            {
                assert_eq!(
                    convert(1.0, *angular_frequency, *other),
                    Err("Units are not compatible for conversion".into())
                );
                assert_eq!(
                    convert(1.0, *other, *angular_frequency),
                    Err("Units are not compatible for conversion".into())
                );
            }
        }
    }

    #[test]
    fn hydraulic_quantities_are_not_interchangeable() {
        let cases = [
            (
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                UnitId::Volume_CubicMeter,
            ),
            (
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
                UnitId::Volume_CubicMeter,
            ),
            (
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
            ),
        ];

        for (from, to) in cases {
            assert_eq!(
                convert(1.0, from, to),
                Err("Units are not compatible for conversion".into())
            );
            assert_eq!(
                convert(1.0, to, from),
                Err("Units are not compatible for conversion".into())
            );
        }
    }

    #[test]
    fn preserves_values_when_units_match() {
        assert_eq!(
            convert(42.5, UnitId::Length_Meter, UnitId::Length_Meter),
            Ok(42.5)
        );
    }

    #[test]
    fn convert_to_base_temperature() {
        assert_approx_eq(convert_to_base(UnitId::Temperature_Celsius), 0.0);
        assert_approx_eq(convert_to_base(UnitId::Temperature_Fahrenheit), 0.0);
        assert_approx_eq(convert_to_base(UnitId::Temperature_Kelvin), 1.0);
    }

    #[test]
    fn precomputed_factors_preserve_calculations_in_const_context() {
        const FACTORS: [f64; 14] = [
            convert_to_base(UnitId::Angle_Degree),
            convert_to_base(UnitId::Angle_Arcminute),
            convert_to_base(UnitId::Angle_Arcsecond),
            convert_to_base(UnitId::Speed_KilometerPerHour),
            convert_to_base(UnitId::Speed_Knot),
            convert_to_base(UnitId::AngularVelocity_DegreePerSecond),
            convert_to_base(UnitId::AngularVelocity_RevolutionPerMinute),
            convert_to_base(UnitId::Acceleration_CentimeterPerSecondSquared),
            convert_to_base(UnitId::Acceleration_FootPerSecondSquared),
            convert_to_base(UnitId::Speed_FootPerSecond),
            convert_to_base(UnitId::VolumeFlowRate_CubicMeterPerSecond),
            convert_to_base(UnitId::VolumeFlowRate_LiterPerMinute),
            convert_to_base(UnitId::DisplacementPerRevolution_CubicMeterPerRevolution),
            convert_to_base(UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution),
        ];
        let expected = [
            1.0_f64.to_radians(),
            std::f64::consts::PI.div(10_800.0),
            std::f64::consts::PI.div(648_000.0),
            1.0_f64.div(3.6),
            1852.0_f64.div(3600.0),
            1.0_f64.to_radians(),
            std::f64::consts::TAU.div(60.0),
            0.01,
            0.3048,
            0.3048,
            1.0,
            1.0_f64.div(60_000.0),
            1.0,
            0.000_001,
        ];

        for (actual, expected) in FACTORS.into_iter().zip(expected) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn converts_proportional_units() {
        let cases = [
            (
                1.0,
                UnitId::Area_SquareKiloMeter,
                UnitId::Area_Hectare,
                100.0,
            ),
            (
                2_500.0,
                UnitId::Current_Milliampere,
                UnitId::Current_Ampere,
                2.5,
            ),
            (1.0, UnitId::Length_Yard, UnitId::Length_Foot, 3.0),
            (3.0, UnitId::Amount_Kilomole, UnitId::Amount_Mole, 3_000.0),
            (2.0, UnitId::Time_Hour, UnitId::Time_Minute, 120.0),
            (
                1.0,
                UnitId::Volume_ImperialGallon,
                UnitId::Volume_Liter,
                4.54609,
            ),
            (1.0, UnitId::Mass_Stone, UnitId::Mass_Kilogram, 6.35029),
        ];

        for (value, from_unit, to_unit, expected) in cases {
            let actual = convert(value, from_unit, to_unit).unwrap();
            assert_approx_eq(actual, expected);
        }
    }

    #[test]
    fn converts_temperature_scales_with_offsets() {
        let cases = [
            (
                0.0,
                UnitId::Temperature_Celsius,
                UnitId::Temperature_Fahrenheit,
                32.0,
            ),
            (
                -40.0,
                UnitId::Temperature_Fahrenheit,
                UnitId::Temperature_Celsius,
                -40.0,
            ),
            (
                273.15,
                UnitId::Temperature_Kelvin,
                UnitId::Temperature_Celsius,
                0.0,
            ),
            (
                373.15,
                UnitId::Temperature_Kelvin,
                UnitId::Temperature_Fahrenheit,
                212.0,
            ),
        ];

        for (value, from_unit, to_unit, expected) in cases {
            let actual = convert(value, from_unit, to_unit).unwrap();
            assert_approx_eq(actual, expected);
        }
    }

    #[test]
    fn converts_practical_prefixes_in_existing_families() {
        let cases = [
            (
                UnitId::Area_SquareMicroMeter,
                UnitId::Area_SquareMeter,
                1e-12,
            ),
            (UnitId::Current_Megaampere, UnitId::Current_Ampere, 1e6),
            (UnitId::Length_Picometer, UnitId::Length_Meter, 1e-12),
            (UnitId::Length_Nanometer, UnitId::Length_Meter, 1e-9),
            (UnitId::Length_Micrometer, UnitId::Length_Meter, 1e-6),
            (UnitId::Time_PicoSecond, UnitId::Time_Second, 1e-12),
            (UnitId::Time_KiloSecond, UnitId::Time_Second, 1e3),
            (UnitId::Volume_Picoliter, UnitId::Volume_Liter, 1e-12),
            (UnitId::Volume_Nanoliter, UnitId::Volume_Liter, 1e-9),
            (UnitId::Volume_Microliter, UnitId::Volume_Liter, 1e-6),
            (UnitId::Volume_Kiloliter, UnitId::Volume_Liter, 1e3),
            (UnitId::Volume_Megaliter, UnitId::Volume_Liter, 1e6),
            (UnitId::Mass_Picogram, UnitId::Mass_Gram, 1e-12),
            (UnitId::Mass_Nanogram, UnitId::Mass_Gram, 1e-9),
            (UnitId::Mass_Microgram, UnitId::Mass_Gram, 1e-6),
            (UnitId::Mass_Milligram, UnitId::Mass_Gram, 1e-3),
            (UnitId::Mass_Megagram, UnitId::Mass_Gram, 1e6),
        ];

        for (unit, base, factor) in cases {
            assert_approx_eq(convert(1.0, unit, base).unwrap().div(factor), 1.0);
            assert_approx_eq(convert(factor, base, unit).unwrap(), 1.0);
            assert_approx_eq(convert(-1.0, unit, base).unwrap().div(factor), -1.0);
        }

        assert_approx_eq(
            convert(
                1.0,
                UnitId::Area_SquareMicroMeter,
                UnitId::Area_SquareMilliMeter,
            )
            .unwrap(),
            1e-6,
        );
        assert_eq!(
            convert(1.0, UnitId::Mass_Megagram, UnitId::Mass_Tonne),
            Ok(1.0)
        );
    }

    #[test]
    fn converts_prefixed_kelvin_with_temperature_offsets() {
        let units = [
            (UnitId::Temperature_Nanokelvin, 1e-9),
            (UnitId::Temperature_Microkelvin, 1e-6),
            (UnitId::Temperature_Millikelvin, 1e-3),
            (UnitId::Temperature_Kilokelvin, 1e3),
        ];

        for (unit, scale) in units {
            assert_approx_eq(
                convert(1.0, unit, UnitId::Temperature_Kelvin)
                    .unwrap()
                    .div(scale),
                1.0,
            );
            assert_approx_eq(
                convert(scale, UnitId::Temperature_Kelvin, unit).unwrap(),
                1.0,
            );
            let freezing = 273.15_f64.div(scale);
            let boiling = 373.15_f64.div(scale);
            assert_approx_eq(
                convert(0.0, UnitId::Temperature_Celsius, unit)
                    .unwrap()
                    .div(freezing),
                1.0,
            );
            assert_approx_eq(
                convert(32.0, UnitId::Temperature_Fahrenheit, unit)
                    .unwrap()
                    .div(freezing),
                1.0,
            );
            assert_approx_eq(
                convert(freezing, unit, UnitId::Temperature_Celsius).unwrap(),
                0.0,
            );
            assert_approx_eq(
                convert(freezing, unit, UnitId::Temperature_Fahrenheit).unwrap(),
                32.0,
            );
            assert_approx_eq(
                convert(100.0, UnitId::Temperature_Celsius, unit)
                    .unwrap()
                    .div(boiling),
                1.0,
            );
            assert_approx_eq(
                convert(boiling, unit, UnitId::Temperature_Celsius).unwrap(),
                100.0,
            );
            assert_approx_eq(
                convert(boiling, unit, UnitId::Temperature_Fahrenheit).unwrap(),
                212.0,
            );
            assert_eq!(convert(-273.15, UnitId::Temperature_Celsius, unit), Ok(0.0));
        }

        assert_approx_eq(
            convert(
                1000.0,
                UnitId::Temperature_Nanokelvin,
                UnitId::Temperature_Microkelvin,
            )
            .unwrap(),
            1.0,
        );
        assert!(
            convert(
                f64::MAX,
                UnitId::Temperature_Kilokelvin,
                UnitId::Temperature_Celsius
            )
            .is_err()
        );
        assert!(
            convert(
                f64::NAN,
                UnitId::Temperature_Nanokelvin,
                UnitId::Temperature_Kelvin
            )
            .is_err()
        );
        assert!(
            convert(
                1.0,
                UnitId::Temperature_Millikelvin,
                UnitId::Current_Milliampere
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_incompatible_units() {
        assert_eq!(
            convert(1.0, UnitId::Length_Meter, UnitId::Time_Second),
            Err("Units are not compatible for conversion".into())
        );
    }

    #[test]
    fn named_si_derived_units_use_coherent_family_bases() {
        let prefixes = [
            ("", 1.0),
            ("pico", 1e-12),
            ("nano", 1e-9),
            ("micro", 1e-6),
            ("milli", 1e-3),
            ("kilo", 1e3),
            ("mega", 1e6),
            ("giga", 1e9),
            ("tera", 1e12),
        ];

        for family in UnitFamilyId::ALL {
            if !(UnitFamilyId::Frequency.to_u8()..=UnitFamilyId::MagneticFlux.to_u8())
                .contains(&family.to_u8())
            {
                continue;
            }

            let units = family.unit_ids();
            let base = units.first().unwrap();
            let base_key = base.string_id();
            let (family_key, name) = base_key.as_str().rsplit_once('_').unwrap();

            let prefix_count = if matches!(
                family,
                UnitFamilyId::SolidAngle
                    | UnitFamilyId::Inductance
                    | UnitFamilyId::MagneticFluxDensity
            ) {
                5
            } else if family == UnitFamilyId::Capacitance {
                6
            } else {
                prefixes.len()
            };

            for (prefix, scale) in prefixes.into_iter().take(prefix_count) {
                let key = format!("{family_key}_{prefix}{name}");
                let unit = UnitId::from_unit_id_str(&key).unwrap();
                assert!(units.contains(&unit));
                assert_approx_eq(convert_to_base(unit).div(scale), 1.0);
                assert_approx_eq(convert(1.0, unit, *base).unwrap().div(scale), 1.0);
                assert_approx_eq(convert(scale, *base, unit).unwrap(), 1.0);
                assert_approx_eq(convert(-1.0, unit, *base).unwrap().div(scale), -1.0);
                assert_eq!(convert(12.5, unit, unit), Ok(12.5));
                assert_eq!(convert(12.5, unit, UnitId::None), Ok(12.5));
                assert!(convert(f64::NAN, unit, unit).is_err());
            }
        }
    }

    #[test]
    fn converts_additional_engineering_units() {
        let cases = [
            (
                UnitId::VolumeFlowRate_LiterPerMinute,
                UnitId::VolumeFlowRate_CubicMeterPerSecond,
                1.0_f64.div(60_000.0),
            ),
            (
                UnitId::DisplacementPerRevolution_CubicCentimeterPerRevolution,
                UnitId::DisplacementPerRevolution_CubicMeterPerRevolution,
                0.000_001,
            ),
            (
                UnitId::Speed_CentimeterPerSecond,
                UnitId::Speed_MeterPerSecond,
                0.01,
            ),
            (
                UnitId::Speed_MillimeterPerSecond,
                UnitId::Speed_MeterPerSecond,
                0.001,
            ),
            (
                UnitId::Speed_InchPerSecond,
                UnitId::Speed_MeterPerSecond,
                0.0254,
            ),
            (
                UnitId::Speed_FootPerSecond,
                UnitId::Speed_InchPerSecond,
                12.0,
            ),
            (
                UnitId::Acceleration_MillimeterPerSecondSquared,
                UnitId::Acceleration_MeterPerSecondSquared,
                0.001,
            ),
            (
                UnitId::Acceleration_InchPerSecondSquared,
                UnitId::Acceleration_MeterPerSecondSquared,
                0.0254,
            ),
            (
                UnitId::AngularVelocity_RevolutionPerSecond,
                UnitId::AngularVelocity_RadianPerSecond,
                std::f64::consts::TAU,
            ),
            (
                UnitId::AngularVelocity_RevolutionPerSecond,
                UnitId::AngularVelocity_RevolutionPerMinute,
                60.0,
            ),
            (
                UnitId::Torque_NewtonMillimeter,
                UnitId::Torque_NewtonMeter,
                0.001,
            ),
            (
                UnitId::Torque_NewtonCentimeter,
                UnitId::Torque_NewtonMeter,
                0.01,
            ),
            (
                UnitId::Torque_PoundForceInch,
                UnitId::Torque_NewtonMeter,
                4.448_221_615_260_5_f64.mul(0.0254),
            ),
            (
                UnitId::Torque_PoundForceFoot,
                UnitId::Torque_PoundForceInch,
                12.0,
            ),
            (
                UnitId::Density_KilogramPerLiter,
                UnitId::Density_KilogramPerCubicMeter,
                1000.0,
            ),
            (
                UnitId::Density_GramPerLiter,
                UnitId::Density_KilogramPerCubicMeter,
                1.0,
            ),
            (
                UnitId::Density_PoundPerCubicFoot,
                UnitId::Density_KilogramPerCubicMeter,
                0.453_592_37_f64.div(0.3048_f64.mul(0.3048).mul(0.3048)),
            ),
            (
                UnitId::Speed_FootPerSecond,
                UnitId::Speed_MeterPerSecond,
                0.3048,
            ),
            (
                UnitId::Speed_FootPerSecond,
                UnitId::Speed_MilePerHour,
                3600.0_f64.div(5280.0),
            ),
            (
                UnitId::Acceleration_CentimeterPerSecondSquared,
                UnitId::Acceleration_MeterPerSecondSquared,
                0.01,
            ),
            (
                UnitId::Acceleration_FootPerSecondSquared,
                UnitId::Acceleration_MeterPerSecondSquared,
                0.3048,
            ),
            (
                UnitId::Acceleration_FootPerSecondSquared,
                UnitId::Acceleration_CentimeterPerSecondSquared,
                30.48,
            ),
            (UnitId::Volume_CubicMeter, UnitId::Volume_Liter, 1000.0),
            (UnitId::Volume_CubicCentiMeter, UnitId::Volume_Liter, 0.001),
            (UnitId::Volume_CubicMilliMeter, UnitId::Volume_Liter, 1e-6),
            (
                UnitId::Angle_Turn,
                UnitId::Angle_Radian,
                std::f64::consts::TAU,
            ),
            (
                UnitId::Angle_Arcminute,
                UnitId::Angle_Degree,
                1.0_f64.div(60.0),
            ),
            (
                UnitId::Angle_Arcsecond,
                UnitId::Angle_Degree,
                1.0_f64.div(3600.0),
            ),
            (UnitId::Force_KilogramForce, UnitId::Force_Newton, 9.80665),
            (
                UnitId::Force_PoundForce,
                UnitId::Force_Newton,
                0.453_592_37_f64.mul(9.80665),
            ),
            (UnitId::Pressure_Bar, UnitId::Pressure_Pascal, 1e5),
            (UnitId::Pressure_Millibar, UnitId::Pressure_Pascal, 100.0),
            (
                UnitId::Pressure_StandardAtmosphere,
                UnitId::Pressure_Pascal,
                101_325.0,
            ),
            (
                UnitId::Pressure_PoundPerSquareInch,
                UnitId::Pressure_Pascal,
                0.453_592_37_f64.mul(9.80665).div(0.0254_f64.mul(0.0254)),
            ),
            (
                UnitId::Pressure_MillimeterOfMercury,
                UnitId::Pressure_Pascal,
                13_595.1_f64.mul(9.80665).mul(0.001),
            ),
            (UnitId::Energy_WattHour, UnitId::Energy_Joule, 3600.0),
            (UnitId::Energy_KilowattHour, UnitId::Energy_Joule, 3.6e6),
            (
                UnitId::Energy_Electronvolt,
                UnitId::Energy_Joule,
                1.602_176_634e-19,
            ),
            (
                UnitId::Power_MechanicalHorsepower,
                UnitId::Power_Watt,
                550.0_f64.mul(0.3048).mul(0.453_592_37).mul(9.80665),
            ),
            (
                UnitId::Power_MetricHorsepower,
                UnitId::Power_Watt,
                75.0_f64.mul(9.80665),
            ),
            (
                UnitId::ElectricCharge_AmpereHour,
                UnitId::ElectricCharge_Coulomb,
                3600.0,
            ),
            (
                UnitId::ElectricCharge_MilliampereHour,
                UnitId::ElectricCharge_Coulomb,
                3.6,
            ),
        ];

        for (unit, base, factor) in cases {
            assert_approx_eq(convert(1.0, unit, base).unwrap().div(factor), 1.0);
            assert_approx_eq(convert(factor, base, unit).unwrap(), 1.0);
            assert_approx_eq(convert(-1.0, unit, base).unwrap().div(factor), -1.0);
        }
    }

    #[test]
    fn converts_new_simulation_families() {
        let cases = [
            (
                UnitId::Speed_MeterPerSecond,
                UnitId::Speed_MeterPerSecond,
                1.0,
            ),
            (
                UnitId::Speed_KilometerPerHour,
                UnitId::Speed_MeterPerSecond,
                1.0_f64.div(3.6),
            ),
            (
                UnitId::Speed_MilePerHour,
                UnitId::Speed_MeterPerSecond,
                1609.344_f64.div(3600.0),
            ),
            (
                UnitId::Speed_Knot,
                UnitId::Speed_MeterPerSecond,
                1852.0_f64.div(3600.0),
            ),
            (
                UnitId::Acceleration_MeterPerSecondSquared,
                UnitId::Acceleration_MeterPerSecondSquared,
                1.0,
            ),
            (
                UnitId::Acceleration_StandardGravity,
                UnitId::Acceleration_MeterPerSecondSquared,
                9.80665,
            ),
            (
                UnitId::AngularVelocity_RadianPerSecond,
                UnitId::AngularVelocity_RadianPerSecond,
                1.0,
            ),
            (
                UnitId::AngularVelocity_DegreePerSecond,
                UnitId::AngularVelocity_RadianPerSecond,
                std::f64::consts::PI.div(180.0),
            ),
            (
                UnitId::AngularVelocity_RevolutionPerMinute,
                UnitId::AngularVelocity_RadianPerSecond,
                std::f64::consts::TAU.div(60.0),
            ),
            (UnitId::Torque_NewtonMeter, UnitId::Torque_NewtonMeter, 1.0),
            (
                UnitId::Torque_PoundForceFoot,
                UnitId::Torque_NewtonMeter,
                0.453_592_37_f64.mul(9.80665).mul(0.3048),
            ),
            (
                UnitId::Density_KilogramPerCubicMeter,
                UnitId::Density_KilogramPerCubicMeter,
                1.0,
            ),
            (
                UnitId::Density_GramPerCubicCentimeter,
                UnitId::Density_KilogramPerCubicMeter,
                1000.0,
            ),
        ];

        for (unit, base, factor) in cases {
            assert_approx_eq(convert_to_base(unit).div(factor), 1.0);
            assert_approx_eq(convert(1.0, unit, base).unwrap().div(factor), 1.0);
            assert_approx_eq(convert(factor, base, unit).unwrap(), 1.0);
            assert_approx_eq(convert(-1.0, unit, base).unwrap().div(factor), -1.0);
        }
    }

    #[test]
    fn engineering_equivalences_preserve_quantity_boundaries() {
        let cases = [
            (
                1.0,
                UnitId::Volume_CubicMeter,
                UnitId::Volume_CubicCentiMeter,
                1e6,
            ),
            (
                1.0,
                UnitId::Volume_CubicCentiMeter,
                UnitId::Volume_Milliliter,
                1.0,
            ),
            (
                1.0,
                UnitId::Volume_CubicMilliMeter,
                UnitId::Volume_Microliter,
                1.0,
            ),
            (1.0, UnitId::Angle_Turn, UnitId::Angle_Degree, 360.0),
            (60.0, UnitId::Angle_Arcsecond, UnitId::Angle_Arcminute, 1.0),
            (
                1.0,
                UnitId::Pressure_Bar,
                UnitId::Pressure_Kilopascal,
                100.0,
            ),
            (
                1.0,
                UnitId::Energy_KilowattHour,
                UnitId::Energy_WattHour,
                1000.0,
            ),
            (
                1.0,
                UnitId::ElectricCharge_AmpereHour,
                UnitId::ElectricCharge_MilliampereHour,
                1000.0,
            ),
            (
                60.0,
                UnitId::AngularVelocity_RevolutionPerMinute,
                UnitId::AngularVelocity_DegreePerSecond,
                360.0,
            ),
        ];

        for (value, from, to, expected) in cases {
            assert_approx_eq(convert(value, from, to).unwrap(), expected);
        }

        let incompatible = [
            (UnitId::Torque_NewtonMeter, UnitId::Energy_Joule),
            (
                UnitId::AngularVelocity_RadianPerSecond,
                UnitId::Frequency_Hertz,
            ),
            (UnitId::Acceleration_StandardGravity, UnitId::Mass_Gram),
            (UnitId::Speed_MeterPerSecond, UnitId::Length_Meter),
            (UnitId::Density_KilogramPerCubicMeter, UnitId::Mass_Kilogram),
        ];

        for (from, to) in incompatible {
            assert!(convert(1.0, from, to).is_err());
            assert!(convert(1.0, to, from).is_err());
        }
        assert!(convert(f64::MAX, UnitId::Energy_KilowattHour, UnitId::Energy_Joule).is_err());
    }

    #[test]
    fn converts_degrees_and_radians() {
        let cases = [
            (
                180.0,
                UnitId::Angle_Degree,
                UnitId::Angle_Radian,
                std::f64::consts::PI,
            ),
            (
                std::f64::consts::PI,
                UnitId::Angle_Radian,
                UnitId::Angle_Degree,
                180.0,
            ),
            (
                90.0,
                UnitId::Angle_Degree,
                UnitId::Angle_Radian,
                std::f64::consts::FRAC_PI_2,
            ),
            (
                -180.0,
                UnitId::Angle_Degree,
                UnitId::Angle_Radian,
                -std::f64::consts::PI,
            ),
            (
                180.0,
                UnitId::Angle_Degree,
                UnitId::Angle_Milliradian,
                std::f64::consts::PI.mul(1000.0),
            ),
            (0.0, UnitId::Angle_Degree, UnitId::Angle_Radian, 0.0),
        ];

        for (value, from, to, expected) in cases {
            assert_approx_eq(convert(value, from, to).unwrap(), expected);
        }

        assert!(convert(1.0, UnitId::Angle_Degree, UnitId::SolidAngle_Steradian).is_err());
        assert!(convert(1.0, UnitId::Angle_Degree, UnitId::Temperature_Celsius).is_err());
    }

    #[test]
    fn converts_between_engineering_prefixes() {
        let cases = [
            (1.0, UnitId::Voltage_Volt, UnitId::Voltage_Millivolt, 1000.0),
            (
                1.0,
                UnitId::Voltage_Megavolt,
                UnitId::Voltage_Millivolt,
                1e9,
            ),
            (
                1.0,
                UnitId::Capacitance_Microfarad,
                UnitId::Capacitance_Nanofarad,
                1000.0,
            ),
            (
                1.0,
                UnitId::Pressure_Megapascal,
                UnitId::Pressure_Kilopascal,
                1000.0,
            ),
            (
                1.0,
                UnitId::Frequency_Gigahertz,
                UnitId::Frequency_Megahertz,
                1000.0,
            ),
            (1.0, UnitId::Power_Terawatt, UnitId::Power_Picowatt, 1e24),
        ];

        for (value, from, to, expected) in cases {
            assert_approx_eq(convert(value, from, to).unwrap(), expected);
        }

        assert!(
            convert(
                f64::MAX,
                UnitId::Voltage_Megavolt,
                UnitId::Voltage_Millivolt
            )
            .is_err()
        );
        assert!(convert(1.0, UnitId::Voltage_Millivolt, UnitId::Power_Milliwatt).is_err());
    }

    #[test]
    fn rejects_distinct_quantities_with_identical_si_dimensions() {
        let pairs = [
            (UnitId::Angle_Radian, UnitId::SolidAngle_Steradian),
            (UnitId::Torque_NewtonMeter, UnitId::Energy_Joule),
            (
                UnitId::AngularVelocity_RadianPerSecond,
                UnitId::Frequency_Hertz,
            ),
        ];

        for (from, to) in pairs {
            assert_eq!(
                convert(1.0, from, to),
                Err("Units are not compatible for conversion".into())
            );
            assert_eq!(
                convert(1.0, to, from),
                Err("Units are not compatible for conversion".into())
            );
        }
    }

    #[test]
    fn converts_concrete_units_to_unitless_values() {
        assert_eq!(convert(42.5, UnitId::Length_Meter, UnitId::None), Ok(42.5));
    }

    #[test]
    fn converts_exact_ratio_with_expected_bits() {
        let converted = convert(1.25, UnitId::Length_Meter, UnitId::Length_Centimeter).unwrap();
        assert_eq!(converted.to_bits(), 125.0_f64.to_bits());
    }

    #[test]
    fn normalizes_negative_zero_bits() {
        let converted = convert(-0.0, UnitId::Length_Meter, UnitId::Length_Meter).unwrap();
        assert_eq!(converted.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn rejects_converting_unitless_values_to_concrete_units() {
        assert_eq!(
            convert(42.5, UnitId::None, UnitId::Length_Meter),
            Err("Cannot convert a unitless value to a unit".into())
        );
    }

    #[test]
    fn rejects_non_finite_inputs_and_results() {
        assert!(convert(f64::NAN, UnitId::Length_Meter, UnitId::Length_Foot).is_err());
        assert!(convert(f64::INFINITY, UnitId::Length_Meter, UnitId::Length_Foot).is_err());
        assert!(convert(f64::MAX, UnitId::Length_Kilometer, UnitId::Length_Meter).is_err());
    }

    #[test]
    fn round_trips_every_unit_within_its_family() {
        for from_unit in UnitId::ALL {
            for to_unit in UnitId::ALL {
                if from_unit.family_id() != to_unit.family_id() {
                    continue;
                }

                let value = if from_unit.family_id() == UnitFamilyId::Temperature
                    && convert_to_base(*from_unit) > 0.0
                {
                    // Use an ordinary absolute temperature to avoid cancellation near absolute zero.
                    300.0_f64.div(convert_to_base(*from_unit))
                } else {
                    12.5
                };
                let converted = convert(value, *from_unit, *to_unit).unwrap();
                let round_tripped = convert(converted, *to_unit, *from_unit).unwrap();
                assert_approx_eq(round_tripped, value);
            }
        }
    }

    #[test]
    fn every_family_has_convertible_units() {
        for family in UnitFamilyId::ALL {
            let units = UnitId::ALL
                .iter()
                .filter(|unit| unit.family_id() == family)
                .count();

            assert!(units > 0, "{family:?} must contain at least one unit");
        }
    }
}
